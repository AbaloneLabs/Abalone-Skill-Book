use std::collections::HashMap;

use anyhow::Result;

use crate::embedding::EmbeddingProvider;
use crate::skill::SkillDocument;
use crate::storage::{SkillRecord, SkillStore};

/// Lightweight skill metadata kept in memory for recommendation scoring.
///
/// This avoids re-loading full skill records (including body and source text)
/// from the database on every recommendation request. Only the fields the
/// ranking engine actually inspects are retained.
#[derive(Debug, Clone)]
pub struct CachedRecord {
    pub path: String,
    pub name: String,
    pub description: String,
    pub disable_model_invocation: bool,
}

impl CachedRecord {
    pub fn from_document(document: &SkillDocument) -> Self {
        Self {
            path: document.path.clone(),
            name: document.frontmatter.name.clone(),
            description: document.frontmatter.description.clone(),
            disable_model_invocation: document.frontmatter.disable_model_invocation,
        }
    }

    pub fn from_record(record: &SkillRecord) -> Self {
        Self {
            path: record.path.clone(),
            name: record.name.clone(),
            description: record.description.clone(),
            disable_model_invocation: record.disable_model_invocation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub vector: Vec<f32>,
    pub record: CachedRecord,
}

/// In-memory store of skill embeddings and metadata, keyed by skill path.
///
/// Built once at startup and updated incrementally when skills are created,
/// updated, or deleted. This eliminates per-request database I/O for the
/// embedding vectors and skill records that the recommendation engine needs,
/// which is critical when the skill repository grows to thousands of entries.
#[derive(Debug, Clone)]
pub struct EmbeddingCache {
    model: String,
    entries: HashMap<String, CacheEntry>,
}

impl EmbeddingCache {
    pub fn build(store: &SkillStore, embedder: &dyn EmbeddingProvider) -> Result<Self> {
        let model = embedder.spec().name.clone();
        let entries = load_entries(store, &model)?;
        Ok(Self { model, entries })
    }

    /// Replace all entries from the database.
    ///
    /// Called after a filesystem re-sync to bring the cache in line with the
    /// current index state.
    pub fn load_from_store(
        &mut self,
        store: &SkillStore,
        embedder: &dyn EmbeddingProvider,
    ) -> Result<()> {
        self.model = embedder.spec().name.clone();
        self.entries = load_entries(store, &self.model)?;
        Ok(())
    }

    /// Refresh a single skill entry from the database.
    ///
    /// Used after a single skill is created or updated via an MCP tool, so the
    /// cache reflects the change without a full reload. If the skill has no
    /// embedding row yet, the entry is left untouched.
    pub fn refresh_entry(&mut self, store: &SkillStore, path: &str) -> Result<()> {
        let Some(record) = store.get_skill(path)? else {
            self.remove(path);
            return Ok(());
        };
        let Some(vector) = store.embedding_for_path(path)? else {
            return Ok(());
        };
        self.upsert(
            record.path.clone(),
            vector,
            CachedRecord::from_record(&record),
        );
        Ok(())
    }

    pub fn upsert(&mut self, path: String, vector: Vec<f32>, record: CachedRecord) {
        self.entries.insert(path, CacheEntry { vector, record });
    }

    pub fn remove(&mut self, path: &str) {
        self.entries.remove(path);
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &HashMap<String, CacheEntry> {
        &self.entries
    }
}

fn load_entries(store: &SkillStore, model: &str) -> Result<HashMap<String, CacheEntry>> {
    let records = store.all_skill_records()?;
    let mut vectors: HashMap<String, Vec<f32>> = store
        .embeddings_for_model(model)?
        .into_iter()
        .collect();

    let mut entries = HashMap::with_capacity(records.len());
    for record in records {
        if let Some(vector) = vectors.remove(&record.path) {
            entries.insert(
                record.path.clone(),
                CacheEntry {
                    vector,
                    record: CachedRecord::from_record(&record),
                },
            );
        }
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::EmbeddingCache;
    use crate::embedding::{DeterministicEmbeddingProvider, EmbeddingProvider};
    use crate::storage::SkillStore;

    #[test]
    fn cache_builds_from_store_and_tracks_entry_count() {
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

        let embedder = DeterministicEmbeddingProvider::new(32);
        store.sync_filesystem(temp.path(), Some(&embedder)).unwrap();

        let cache = EmbeddingCache::build(&store, &embedder).unwrap();
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.model(), embedder.spec().name);
        assert!(cache.entries().contains_key("investor/analyze/market-analysis"));
    }

    #[test]
    fn upsert_and_remove_update_entries_incrementally() {
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

        let embedder = DeterministicEmbeddingProvider::new(32);
        store.sync_filesystem(temp.path(), Some(&embedder)).unwrap();
        let mut cache = EmbeddingCache::build(&store, &embedder).unwrap();

        assert_eq!(cache.len(), 1);

        cache.upsert(
            "programmer/api/test".to_string(),
            vec![0.0; 32],
            super::CachedRecord {
                path: "programmer/api/test".to_string(),
                name: "test.md".to_string(),
                description: "test".to_string(),
                disable_model_invocation: false,
            },
        );
        assert_eq!(cache.len(), 2);

        cache.remove("investor/analyze/market-analysis");
        assert_eq!(cache.len(), 1);
        assert!(!cache
            .entries()
            .contains_key("investor/analyze/market-analysis"));
    }

    #[test]
    fn load_from_store_rebuilds_all_entries() {
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

        let embedder = DeterministicEmbeddingProvider::new(32);
        store.sync_filesystem(temp.path(), Some(&embedder)).unwrap();
        let mut cache = EmbeddingCache::build(&store, &embedder).unwrap();

        cache.upsert(
            "stale/entry".to_string(),
            vec![0.0; 32],
            super::CachedRecord {
                path: "stale/entry".to_string(),
                name: "stale.md".to_string(),
                description: "stale".to_string(),
                disable_model_invocation: false,
            },
        );
        assert_eq!(cache.len(), 2);

        cache.load_from_store(&store, &embedder).unwrap();
        assert_eq!(cache.len(), 1);
        assert!(!cache.entries().contains_key("stale/entry"));
    }
}
