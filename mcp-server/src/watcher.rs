//! Filesystem watcher that keeps the skill index and embedding cache in sync
//! when skill files are created, updated, or deleted outside of MCP tool calls.
//!
//! Skill file changes are infrequent (skills are being authored, not edited at
//! request time), so a simple debounced re-sync is sufficient. The debouncer
//! coalesces bursts of events (e.g. a batch of skills written at once) into a
//! single re-sync after a quiet period.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context, Result};
use notify_debouncer_mini::{new_debouncer, DebouncedEvent};
use tokio::sync::mpsc;

use crate::embedding::EmbeddingProvider;
use crate::embedding_cache::EmbeddingCache;
use crate::storage::{SkillStore, SyncReport};

/// Handle to a running filesystem watcher.
///
/// Dropping this value stops the watcher. The background debouncer thread and
/// the notification channel are cleaned up automatically.
pub struct SkillWatcher {
    /// Keeps the debouncer alive while the handle is held.
    _debouncer: notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>,
}

impl SkillWatcher {
    /// Spawn a watcher over `skills_root` that re-syncs the store and cache
    /// whenever a skill file changes.
    ///
    /// The watcher runs on its own background thread (provided by `notify`).
    /// Events are debounced for `debounce` so that a batch of writes collapses
    /// into a single re-sync. Each re-sync acquires the same store/cache mutexes
    /// used by MCP tool calls, so index consistency is preserved.
    pub fn spawn(
        skills_root: PathBuf,
        store: Arc<Mutex<SkillStore>>,
        cache: Arc<Mutex<EmbeddingCache>>,
        embedder: Arc<dyn EmbeddingProvider>,
        debounce: Duration,
    ) -> Result<Self> {
        let (tx, mut rx) = mpsc::channel::<()>(16);

        let mut debouncer = new_debouncer(debounce, move |res: std::result::Result<
            Vec<DebouncedEvent>,
            notify::Error,
        >| {
            if res.is_ok() {
                // The event content does not matter; any change triggers a full
                // re-sync which is cheap relative to skill authoring frequency.
                let _ = tx.blocking_send(());
            }
        })
        .context("failed to create filesystem debouncer")?;

        debouncer
            .watcher()
            .watch(&skills_root, notify::RecursiveMode::Recursive)
            .with_context(|| {
                format!(
                    "failed to watch skills root {}",
                    skills_root.display()
                )
            })?;

        tracing::info!(
            skills_root = %skills_root.display(),
            debounce_ms = debounce.as_millis(),
            "filesystem skill watcher started"
        );

        // Drive the re-sync loop on a dedicated async task.
        tokio::spawn(async move {
            while rx.recv().await.is_some() {
                match resync(&skills_root, &store, &cache, embedder.as_ref()) {
                    Ok(report) => {
                        tracing::info!(
                            indexed = report.indexed,
                            removed = report.removed,
                            invalid = report.invalid.len(),
                            "skill watcher re-sync complete"
                        );
                        for invalid in &report.invalid {
                            tracing::warn!(
                                path = %invalid.path,
                                errors = ?invalid.report.errors,
                                "invalid skill excluded during re-sync"
                            );
                        }
                    }
                    Err(err) => {
                        tracing::error!(error = %err, "skill watcher re-sync failed");
                    }
                }
            }
        });

        Ok(Self {
            _debouncer: debouncer,
        })
    }
}

/// Re-run the filesystem sync and refresh the embedding cache.
///
/// This acquires the store mutex, runs `sync_filesystem` over `skills_root`,
/// then reloads the cache from the updated store so recommendations reflect
/// any externally added, modified, or removed skills.
pub fn resync(
    skills_root: &Path,
    store: &Arc<Mutex<SkillStore>>,
    cache: &Arc<Mutex<EmbeddingCache>>,
    embedder: &dyn EmbeddingProvider,
) -> Result<SyncReport> {
    let report = {
        let mut store = store
            .lock()
            .map_err(|_| anyhow::anyhow!("skill store lock is poisoned during re-sync"))?;
        store.sync_filesystem(skills_root, Some(embedder))?
    };

    {
        let store = store
            .lock()
            .map_err(|_| anyhow::anyhow!("skill store lock is poisoned during cache reload"))?;
        let mut cache = cache
            .lock()
            .map_err(|_| anyhow::anyhow!("embedding cache lock is poisoned during reload"))?;
        cache.load_from_store(&store, embedder)?;
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::Duration;

    use tempfile::TempDir;

    use super::SkillWatcher;
    use crate::embedding::DeterministicEmbeddingProvider;
    use crate::embedding_cache::EmbeddingCache;
    use crate::storage::SkillStore;

    #[tokio::test]
    async fn watcher_picks_up_new_skill_after_debounce() {
        let mut store = SkillStore::in_memory().unwrap();
        let temp = TempDir::new().unwrap();
        let embedder = std::sync::Arc::new(DeterministicEmbeddingProvider::new(32));
        store
            .sync_filesystem(temp.path(), Some(embedder.as_ref()))
            .unwrap();
        let store = std::sync::Arc::new(std::sync::Mutex::new(store));
        let cache = {
            let s = store.lock().unwrap();
            EmbeddingCache::build(&s, embedder.as_ref()).unwrap()
        };
        let cache = std::sync::Arc::new(std::sync::Mutex::new(cache));

        let _watcher = SkillWatcher::spawn(
            temp.path().to_path_buf(),
            store.clone(),
            cache.clone(),
            embedder.clone(),
            Duration::from_millis(100),
        )
        .unwrap();

        // Write a new skill file.
        let skill_dir = temp
            .path()
            .join("investor")
            .join("analyze")
            .join("market-analysis");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(
            skill_dir.join("SKILL.md"),
            include_str!("../../skills/investor/analyze/market-analysis/SKILL.md"),
        )
        .unwrap();

        // Wait for debounce + re-sync.
        tokio::time::sleep(Duration::from_millis(500)).await;

        let cache_len = cache.lock().unwrap().len();
        assert!(
            cache_len >= 1,
            "expected cache to contain the new skill after re-sync, got {cache_len}"
        );
    }

    #[tokio::test]
    async fn watcher_removes_deleted_skill_after_debounce() {
        let mut store = SkillStore::in_memory().unwrap();
        let temp = TempDir::new().unwrap();
        let skill_dir = temp
            .path()
            .join("investor")
            .join("analyze")
            .join("market-analysis");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(
            skill_dir.join("SKILL.md"),
            include_str!("../../skills/investor/analyze/market-analysis/SKILL.md"),
        )
        .unwrap();

        let embedder = std::sync::Arc::new(DeterministicEmbeddingProvider::new(32));
        store
            .sync_filesystem(temp.path(), Some(embedder.as_ref()))
            .unwrap();
        let store = std::sync::Arc::new(std::sync::Mutex::new(store));
        let cache = {
            let s = store.lock().unwrap();
            EmbeddingCache::build(&s, embedder.as_ref()).unwrap()
        };
        let cache = std::sync::Arc::new(std::sync::Mutex::new(cache));

        assert_eq!(cache.lock().unwrap().len(), 1);

        let _watcher = SkillWatcher::spawn(
            temp.path().to_path_buf(),
            store.clone(),
            cache.clone(),
            embedder.clone(),
            Duration::from_millis(100),
        )
        .unwrap();

        // Delete the skill file.
        fs::remove_file(skill_dir.join("SKILL.md")).unwrap();

        // Wait for debounce + re-sync.
        tokio::time::sleep(Duration::from_millis(500)).await;

        let cache_len = cache.lock().unwrap().len();
        assert_eq!(
            cache_len, 0,
            "expected cache to be empty after skill deletion, got {cache_len}"
        );
    }
}
