use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use serde::Serialize;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::embedding::{blob_to_vector, vector_to_blob, EmbeddingProvider};
use crate::recommendation::{RecommendRequest, RecommendationResult};
use crate::session::{
    CompletionCheck, CompletionChecklist, CompletionError, OpenSkillOutcome, SessionKey,
};
use crate::skill::{SkillDocument, SkillSummary};
use crate::validation::{
    normalize_scope_path, normalize_skill_path, validate_skill_source, ValidationReport,
};

#[derive(Debug)]
pub struct SkillStore {
    conn: Connection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SyncReport {
    pub indexed: usize,
    pub removed: usize,
    pub invalid: Vec<InvalidSkill>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InvalidSkill {
    pub path: String,
    pub report: ValidationReport,
}

/// Report from a session garbage-collection pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PruneReport {
    /// Recommendation sessions older than the retention window.
    pub deleted_sessions: usize,
    /// `mcp_session_state` rows whose active recommendation no longer exists.
    pub orphaned_state: usize,
    /// `mcp_session_state` rows older than the retention window.
    pub stale_state: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkillRecord {
    pub path: String,
    pub file_path: String,
    pub name: String,
    pub description: String,
    pub body: String,
    pub source: String,
    pub disable_model_invocation: bool,
    pub content_hash: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FtsMatch {
    pub path: String,
    pub name: String,
    pub description: String,
    pub lexical_score: f32,
}

#[derive(Debug, Clone)]
pub struct EmbeddingRow {
    pub skill_path: String,
    pub model: String,
    pub dimension: i64,
    pub embedded_text_hash: String,
    pub vector: Vec<u8>,
}

impl SkillStore {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;
        configure_connection(&conn, true)?;
        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        configure_connection(&conn, false)?;
        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    pub fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS skills (
                path                    TEXT PRIMARY KEY,
                file_path               TEXT NOT NULL,
                name                    TEXT NOT NULL,
                description             TEXT NOT NULL,
                body                    TEXT NOT NULL,
                source                  TEXT NOT NULL,
                disable_model_invocation INTEGER NOT NULL DEFAULT 0,
                content_hash            TEXT NOT NULL,
                updated_at              TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS skill_fts USING fts5(
                path UNINDEXED,
                name,
                description,
                body
            );

            CREATE TABLE IF NOT EXISTS embeddings (
                skill_path          TEXT PRIMARY KEY REFERENCES skills(path) ON DELETE CASCADE,
                model               TEXT NOT NULL,
                dimension           INTEGER NOT NULL,
                embedded_text_hash  TEXT NOT NULL,
                vector              BLOB NOT NULL,
                updated_at          TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS recommendation_sessions (
                id              TEXT PRIMARY KEY,
                mcp_session_key TEXT NOT NULL,
                intent          TEXT NOT NULL,
                context         TEXT,
                domain          TEXT,
                stack           TEXT,
                created_at      TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS mcp_session_state (
                mcp_session_key          TEXT PRIMARY KEY,
                active_recommendation_id TEXT,
                updated_at              TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS recommendation_session_skills (
                recommendation_id TEXT NOT NULL REFERENCES recommendation_sessions(id) ON DELETE CASCADE,
                skill_path        TEXT NOT NULL,
                rank              INTEGER,
                score             REAL,
                reason            TEXT,
                opened_at         TEXT,
                opened_order      INTEGER,
                PRIMARY KEY (recommendation_id, skill_path)
            );

            ",
        )?;
        ensure_column(
            &self.conn,
            "recommendation_session_skills",
            "opened_order",
            "INTEGER",
        )?;
        self.conn.execute_batch(
            "
            CREATE INDEX IF NOT EXISTS idx_recommendation_sessions_mcp_session_key
                ON recommendation_sessions(mcp_session_key, created_at);

            CREATE INDEX IF NOT EXISTS idx_recommendation_session_skills_opened
                ON recommendation_session_skills(recommendation_id, opened_order, opened_at);
            ",
        )?;
        Ok(())
    }

    pub fn sync_filesystem(
        &mut self,
        skills_root: &Path,
        embedder: Option<&dyn EmbeddingProvider>,
    ) -> Result<SyncReport> {
        let mut seen = HashSet::new();
        let mut indexed = 0;
        let mut invalid = Vec::new();

        for entry in WalkDir::new(skills_root)
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            if !entry.file_type().is_file() || entry.file_name() != "SKILL.md" {
                continue;
            }

            let file_path = entry.path();
            let logical_path = logical_path_for_file(skills_root, file_path)?;
            let source = fs::read_to_string(file_path)
                .with_context(|| format!("failed to read {}", file_path.display()))?;
            let report = validate_skill_source(&logical_path, &source);

            if !report.ok {
                invalid.push(InvalidSkill {
                    path: logical_path,
                    report,
                });
                continue;
            }

            let normalized_path =
                normalize_skill_path(&logical_path).expect("validated skill path should normalize");
            let document =
                SkillDocument::parse(normalized_path.clone(), file_path.to_path_buf(), source)?;

            self.upsert_skill(&document)?;
            if let Some(embedder) = embedder {
                self.upsert_embedding_if_needed(&document, embedder)?;
            }

            seen.insert(normalized_path);
            indexed += 1;
        }

        let removed = self.delete_missing(&seen)?;

        Ok(SyncReport {
            indexed,
            removed,
            invalid,
        })
    }

    pub fn upsert_skill(&mut self, document: &SkillDocument) -> Result<()> {
        let content_hash = hash_text(&document.source);
        let file_path = document.file_path.to_string_lossy().to_string();
        self.conn.execute(
            "
            INSERT INTO skills (
                path, file_path, name, description, body, source,
                disable_model_invocation, content_hash, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'))
            ON CONFLICT(path) DO UPDATE SET
                file_path = excluded.file_path,
                name = excluded.name,
                description = excluded.description,
                body = excluded.body,
                source = excluded.source,
                disable_model_invocation = excluded.disable_model_invocation,
                content_hash = excluded.content_hash,
                updated_at = datetime('now')
            ",
            params![
                &document.path,
                &file_path,
                &document.frontmatter.name,
                &document.frontmatter.description,
                &document.body,
                &document.source,
                document.frontmatter.disable_model_invocation as i64,
                &content_hash,
            ],
        )?;

        self.conn.execute(
            "DELETE FROM skill_fts WHERE path = ?1",
            params![&document.path],
        )?;
        self.conn.execute(
            "INSERT INTO skill_fts (path, name, description, body) VALUES (?1, ?2, ?3, ?4)",
            params![
                &document.path,
                &document.frontmatter.name,
                &document.frontmatter.description,
                &document.body,
            ],
        )?;

        Ok(())
    }

    pub fn upsert_embedding_if_needed(
        &mut self,
        document: &SkillDocument,
        embedder: &dyn EmbeddingProvider,
    ) -> Result<()> {
        let text = document.embedding_text();
        let text_hash = hash_text(&text);
        let spec = embedder.spec();

        let existing = self
            .conn
            .query_row(
                "SELECT model, embedded_text_hash FROM embeddings WHERE skill_path = ?1",
                params![&document.path],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()?;

        if existing
            .as_ref()
            .is_some_and(|(model, hash)| model == &spec.name && hash == &text_hash)
        {
            return Ok(());
        }

        let vector = embedder.embed(&text)?;
        if vector.len() != spec.dimension {
            anyhow::bail!(
                "embedding dimension mismatch for {}: expected {}, got {}",
                document.path,
                spec.dimension,
                vector.len()
            );
        }

        self.conn.execute(
            "
            INSERT INTO embeddings (
                skill_path, model, dimension, embedded_text_hash, vector, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))
            ON CONFLICT(skill_path) DO UPDATE SET
                model = excluded.model,
                dimension = excluded.dimension,
                embedded_text_hash = excluded.embedded_text_hash,
                vector = excluded.vector,
                updated_at = datetime('now')
            ",
            params![
                &document.path,
                &spec.name,
                spec.dimension as i64,
                &text_hash,
                vector_to_blob(&vector),
            ],
        )?;

        Ok(())
    }

    pub fn list_skills(&self, prefix: Option<&str>) -> Result<Vec<SkillSummary>> {
        let mut query = "
            SELECT path, name, description, disable_model_invocation
            FROM skills
        "
        .to_string();
        let mut params_vec: Vec<String> = Vec::new();
        if let Some(prefix) = prefix {
            let normalized =
                normalize_scope_path(prefix).map_err(|issue| anyhow::anyhow!(issue.message))?;
            query.push_str(" WHERE path = ?1 OR path LIKE ?2");
            params_vec.push(normalized.clone());
            params_vec.push(format!("{normalized}/%"));
        }
        query.push_str(" ORDER BY path");

        let mut stmt = self.conn.prepare(&query)?;
        let rows = if params_vec.is_empty() {
            stmt.query_map([], skill_summary_from_row)?
                .collect::<rusqlite::Result<Vec<_>>>()?
        } else {
            stmt.query_map(
                params![&params_vec[0], &params_vec[1]],
                skill_summary_from_row,
            )?
            .collect::<rusqlite::Result<Vec<_>>>()?
        };
        Ok(rows)
    }

    pub fn get_skill(&self, path: &str) -> Result<Option<SkillRecord>> {
        let normalized =
            normalize_skill_path(path).map_err(|issue| anyhow::anyhow!(issue.message))?;
        self.conn
            .query_row(
                "
                SELECT path, file_path, name, description, body, source,
                       disable_model_invocation, content_hash
                FROM skills
                WHERE path = ?1
                ",
                params![normalized],
                skill_record_from_row,
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn search_fts(&self, query: &str, limit: usize) -> Result<Vec<FtsMatch>> {
        let Some(fts_query) = fts_query(query) else {
            return Ok(Vec::new());
        };

        let mut stmt = self.conn.prepare(
            "
            SELECT s.path, s.name, s.description, bm25(skill_fts) AS rank
            FROM skill_fts
            JOIN skills s ON s.path = skill_fts.path
            WHERE skill_fts MATCH ?1
            ORDER BY rank
            LIMIT ?2
            ",
        )?;

        let rows = stmt
            .query_map(params![fts_query, limit as i64], |row| {
                let rank: f64 = row.get(3)?;
                Ok(FtsMatch {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    lexical_score: (-rank.max(-100.0)) as f32,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(rows)
    }

    pub fn all_embeddings(&self) -> Result<Vec<EmbeddingRow>> {
        let mut stmt = self.conn.prepare(
            "SELECT skill_path, model, dimension, embedded_text_hash, vector
             FROM embeddings ORDER BY skill_path",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(EmbeddingRow {
                    skill_path: row.get(0)?,
                    model: row.get(1)?,
                    dimension: row.get(2)?,
                    embedded_text_hash: row.get(3)?,
                    vector: row.get(4)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn embeddings_for_model(&self, model: &str) -> Result<Vec<(String, Vec<f32>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT skill_path, vector FROM embeddings WHERE model = ?1 ORDER BY skill_path",
        )?;
        let rows = stmt
            .query_map(params![model], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, Vec<u8>>(1)?))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        rows.into_iter()
            .map(|(path, blob)| Ok((path, blob_to_vector(&blob)?)))
            .collect()
    }

    /// Return the embedding vector for a single skill path, if present.
    ///
    /// Used by the in-memory embedding cache for incremental updates after a
    /// single skill is created or updated, avoiding a full reload.
    pub fn embedding_for_path(&self, path: &str) -> Result<Option<Vec<f32>>> {
        self.conn
            .query_row(
                "SELECT vector FROM embeddings WHERE skill_path = ?1",
                params![path],
                |row| row.get::<_, Vec<u8>>(0),
            )
            .optional()?
            .map(|blob| blob_to_vector(&blob))
            .transpose()
    }

    pub fn all_skill_records(&self) -> Result<Vec<SkillRecord>> {
        let mut stmt = self.conn.prepare(
            "
            SELECT path, file_path, name, description, body, source,
                   disable_model_invocation, content_hash
            FROM skills
            ORDER BY path
            ",
        )?;
        let rows = stmt
            .query_map([], skill_record_from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn delete_skill(&mut self, path: &str) -> Result<bool> {
        let normalized =
            normalize_skill_path(path).map_err(|issue| anyhow::anyhow!(issue.message))?;
        self.conn.execute(
            "DELETE FROM skill_fts WHERE path = ?1",
            params![&normalized],
        )?;
        let deleted = self
            .conn
            .execute("DELETE FROM skills WHERE path = ?1", params![&normalized])?;
        Ok(deleted > 0)
    }

    pub fn create_recommendation_session(
        &mut self,
        session_key: &SessionKey,
        request: &RecommendRequest,
        results: &[RecommendationResult],
    ) -> Result<String> {
        let recommendation_id = format!("rec_{}", uuid::Uuid::new_v4());
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "
            INSERT INTO recommendation_sessions (
                id, mcp_session_key, intent, context, domain, stack, created_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'))
            ",
            params![
                &recommendation_id,
                session_key.as_str(),
                &request.intent,
                request.context.as_deref(),
                request.domain.as_deref(),
                request.stack.as_deref(),
            ],
        )?;

        for result in results {
            tx.execute(
                "
                INSERT INTO recommendation_session_skills (
                    recommendation_id, skill_path, rank, score, reason, opened_at, opened_order
                )
                VALUES (?1, ?2, ?3, ?4, ?5, NULL, NULL)
                ON CONFLICT(recommendation_id, skill_path) DO UPDATE SET
                    rank = excluded.rank,
                    score = excluded.score,
                    reason = excluded.reason
                ",
                params![
                    &recommendation_id,
                    &result.path,
                    result.rank as i64,
                    result.score as f64,
                    &result.reason,
                ],
            )?;
        }

        set_active_recommendation_tx(&tx, session_key, Some(&recommendation_id))?;
        tx.commit()?;
        Ok(recommendation_id)
    }

    pub fn set_active_recommendation(
        &mut self,
        session_key: &SessionKey,
        recommendation_id: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            "
            INSERT INTO mcp_session_state (
                mcp_session_key, active_recommendation_id, updated_at
            )
            VALUES (?1, ?2, datetime('now'))
            ON CONFLICT(mcp_session_key) DO UPDATE SET
                active_recommendation_id = excluded.active_recommendation_id,
                updated_at = datetime('now')
            ",
            params![session_key.as_str(), recommendation_id],
        )?;
        Ok(())
    }

    pub fn active_recommendation_id(&self, session_key: &SessionKey) -> Result<Option<String>> {
        self.conn
            .query_row(
                "SELECT active_recommendation_id FROM mcp_session_state WHERE mcp_session_key = ?1",
                params![session_key.as_str()],
                |row| row.get(0),
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn mark_skill_opened(
        &mut self,
        session_key: &SessionKey,
        path: &str,
    ) -> Result<OpenSkillOutcome> {
        let normalized =
            normalize_skill_path(path).map_err(|issue| anyhow::anyhow!(issue.message))?;
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let recommendation_id = active_recommendation_id_tx(&tx, session_key)?;

        let skill_exists = tx
            .query_row(
                "SELECT 1 FROM skills WHERE path = ?1",
                params![&normalized],
                |_| Ok(()),
            )
            .optional()?
            .is_some();
        if !skill_exists {
            anyhow::bail!("skill not found: {normalized}");
        }

        let Some(recommendation_id) = recommendation_id else {
            tx.commit()?;
            return Ok(OpenSkillOutcome {
                path: normalized,
                tracked: false,
                recommendation_id: None,
            });
        };

        tx.execute(
            "
            INSERT INTO recommendation_session_skills (
                recommendation_id, skill_path, rank, score, reason, opened_at, opened_order
            )
            VALUES (?1, ?2, NULL, NULL, 'opened outside recommendation list',
                    strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                    COALESCE((
                        SELECT MAX(opened_order) + 1
                        FROM recommendation_session_skills
                        WHERE recommendation_id = ?1
                    ), 1))
            ON CONFLICT(recommendation_id, skill_path) DO UPDATE SET
                opened_at = COALESCE(
                    recommendation_session_skills.opened_at,
                    strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                ),
                opened_order = COALESCE(
                    recommendation_session_skills.opened_order,
                    excluded.opened_order
                )
            ",
            params![&recommendation_id, &normalized],
        )?;
        tx.commit()?;

        Ok(OpenSkillOutcome {
            path: normalized,
            tracked: true,
            recommendation_id: Some(recommendation_id),
        })
    }

    pub fn completion_checklist(
        &self,
        session_key: &SessionKey,
        recommendation_id: Option<&str>,
    ) -> Result<CompletionChecklist> {
        let recommendation_id = match recommendation_id {
            Some(id) => Some(id.to_string()),
            None => self.active_recommendation_id(session_key)?,
        };

        let Some(recommendation_id) = recommendation_id else {
            return Ok(CompletionChecklist {
                recommendation_id: None,
                checks: Vec::new(),
                errors: Vec::new(),
            });
        };

        let mut stmt = self.conn.prepare(
            "
            SELECT rss.skill_path, s.name, s.file_path, s.source
            FROM recommendation_session_skills rss
            JOIN skills s ON s.path = rss.skill_path
            WHERE rss.recommendation_id = ?1
              AND rss.opened_at IS NOT NULL
            ORDER BY rss.opened_order ASC, rss.opened_at ASC, rss.skill_path ASC
            ",
        )?;

        let rows = stmt
            .query_map(params![&recommendation_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        let mut checks = Vec::new();
        let mut errors = Vec::new();

        for (path, name, file_path, source) in rows {
            let document =
                SkillDocument::parse(path.clone(), PathBuf::from(file_path), source.clone())?;
            if let Some(section) = document.section("Self-Check") {
                checks.push(CompletionCheck {
                    path,
                    name,
                    self_check: format!("## Self-Check\n{}", section.content)
                        .trim()
                        .to_string(),
                });
            } else {
                errors.push(CompletionError {
                    path,
                    code: "missing_required_section".to_string(),
                    message: "Missing exact ## Self-Check section.".to_string(),
                });
            }
        }

        Ok(CompletionChecklist {
            recommendation_id: Some(recommendation_id),
            checks,
            errors,
        })
    }

    /// Delete recommendation sessions and MCP session state older than the
    /// retention window, plus orphaned state rows.
    ///
    /// This is the garbage-collection entry point. It removes:
    ///
    /// 1. `recommendation_sessions` rows whose `created_at` is older than the
    ///    cutoff. The linked `recommendation_session_skills` rows are removed
    ///    automatically via `ON DELETE CASCADE`.
    /// 2. `mcp_session_state` rows whose `active_recommendation_id` points to a
    ///    recommendation session that no longer exists (orphaned references).
    /// 3. `mcp_session_state` rows whose `updated_at` is older than the cutoff.
    ///
    /// All three steps run in a single transaction so the database is never
    /// left in a partially pruned state.
    pub fn prune_stale_sessions(&mut self, retention: Duration) -> Result<PruneReport> {
        let cutoff = Utc::now() - chrono::Duration::from_std(retention)
            .unwrap_or_else(|_| chrono::Duration::days(7));
        let cutoff_str = cutoff.format("%Y-%m-%d %H:%M:%S").to_string();

        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;

        let deleted_sessions = tx.execute(
            "DELETE FROM recommendation_sessions WHERE created_at < ?1",
            params![cutoff_str],
        )?;

        let orphaned_state = tx.execute(
            "DELETE FROM mcp_session_state
             WHERE active_recommendation_id IS NOT NULL
               AND active_recommendation_id NOT IN (
                   SELECT id FROM recommendation_sessions
               )",
            [],
        )?;

        let stale_state = tx.execute(
            "DELETE FROM mcp_session_state WHERE updated_at < ?1",
            params![cutoff_str],
        )?;

        tx.commit()?;

        Ok(PruneReport {
            deleted_sessions,
            orphaned_state,
            stale_state,
        })
    }

    fn delete_missing(&mut self, seen: &HashSet<String>) -> Result<usize> {
        let existing = self.existing_paths()?;
        let mut removed = 0;
        for path in existing {
            if !seen.contains(&path) {
                self.delete_skill(&path)?;
                removed += 1;
            }
        }
        Ok(removed)
    }

    fn existing_paths(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT path FROM skills")?;
        let rows = stmt
            .query_map([], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }
}

pub fn open_database(path: &Path) -> Result<Connection> {
    SkillStore::open(path).map(|store| store.conn)
}

fn configure_connection(conn: &Connection, file_backed: bool) -> Result<()> {
    conn.busy_timeout(Duration::from_secs(5))?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    if file_backed {
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
    }
    Ok(())
}

fn ensure_column(
    conn: &Connection,
    table: &str,
    column: &str,
    column_definition: &str,
) -> Result<()> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({table})"))?;
    let columns = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    if columns.iter().any(|existing| existing == column) {
        return Ok(());
    }

    conn.execute(
        &format!("ALTER TABLE {table} ADD COLUMN {column} {column_definition}"),
        [],
    )?;
    Ok(())
}

fn set_active_recommendation_tx(
    tx: &Transaction<'_>,
    session_key: &SessionKey,
    recommendation_id: Option<&str>,
) -> Result<()> {
    tx.execute(
        "
        INSERT INTO mcp_session_state (
            mcp_session_key, active_recommendation_id, updated_at
        )
        VALUES (?1, ?2, datetime('now'))
        ON CONFLICT(mcp_session_key) DO UPDATE SET
            active_recommendation_id = excluded.active_recommendation_id,
            updated_at = datetime('now')
        ",
        params![session_key.as_str(), recommendation_id],
    )?;
    Ok(())
}

fn active_recommendation_id_tx(
    tx: &Transaction<'_>,
    session_key: &SessionKey,
) -> Result<Option<String>> {
    tx.query_row(
        "SELECT active_recommendation_id FROM mcp_session_state WHERE mcp_session_key = ?1",
        params![session_key.as_str()],
        |row| row.get(0),
    )
    .optional()
    .map_err(Into::into)
}

pub fn hash_text(text: &str) -> String {
    let digest = Sha256::digest(text.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn logical_path_for_file(skills_root: &Path, file_path: &Path) -> Result<String> {
    let relative = file_path.strip_prefix(skills_root).with_context(|| {
        format!(
            "{} is not under {}",
            file_path.display(),
            skills_root.display()
        )
    })?;
    let parent = relative
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .context("SKILL.md must live under a skill directory")?;
    path_to_slash_string(parent)
}

fn path_to_slash_string(path: &Path) -> Result<String> {
    let parts = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    Ok(parts.join("/"))
}

fn skill_summary_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SkillSummary> {
    Ok(SkillSummary {
        path: row.get(0)?,
        name: row.get(1)?,
        description: row.get(2)?,
        disable_model_invocation: row.get::<_, i64>(3)? != 0,
    })
}

fn skill_record_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SkillRecord> {
    Ok(SkillRecord {
        path: row.get(0)?,
        file_path: row.get(1)?,
        name: row.get(2)?,
        description: row.get(3)?,
        body: row.get(4)?,
        source: row.get(5)?,
        disable_model_invocation: row.get::<_, i64>(6)? != 0,
        content_hash: row.get(7)?,
    })
}

fn fts_query(query: &str) -> Option<String> {
    let tokens = query
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| token.len() >= 2)
        .map(|token| format!("{}*", token.to_ascii_lowercase()))
        .collect::<Vec<_>>();

    if tokens.is_empty() {
        None
    } else {
        Some(tokens.join(" OR "))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::SkillStore;
    use crate::embedding::DeterministicEmbeddingProvider;
    use crate::embedding_cache::EmbeddingCache;
    use crate::recommendation::{DomainMode, RecommendRequest, RecommendationEngine};
    use crate::session::SessionKey;
    use rusqlite::params;

    #[test]
    fn sync_indexes_valid_skills_and_embeddings() {
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

        let embedder = DeterministicEmbeddingProvider::new(16);
        let report = store
            .sync_filesystem(temp.path(), Some(&embedder))
            .expect("sync should succeed");

        assert_eq!(report.indexed, 1);
        assert!(report.invalid.is_empty());
        assert!(store
            .get_skill("investor/analyze/market-analysis")
            .unwrap()
            .is_some());
        assert_eq!(store.all_embeddings().unwrap().len(), 1);
    }

    #[test]
    fn sync_excludes_invalid_skills() {
        let mut store = SkillStore::in_memory().unwrap();
        let temp = TempDir::new().unwrap();
        let skill_dir = temp
            .path()
            .join("programmer")
            .join("api")
            .join("tiny");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(
            skill_dir.join("SKILL.md"),
            "---
name: tiny.md
description: Use when the agent is checking API response data before exposing fields to a caller.
---

# Tiny

## Core Rules
One.

## Common Traps
Two.

## Self-Check
- [ ] Three.
",
        )
        .unwrap();

        let report = store.sync_filesystem(temp.path(), None).unwrap();
        assert_eq!(report.indexed, 0);
        assert_eq!(report.invalid.len(), 1);
        assert!(store
            .get_skill("programmer/api/tiny")
            .unwrap()
            .is_none());
    }

    #[test]
    fn fts_search_returns_indexed_skill() {
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
        store.sync_filesystem(temp.path(), None).unwrap();

        let results = store.search_fts("market valuation liquidity", 5).unwrap();
        assert_eq!(results[0].path, "investor/analyze/market-analysis");
    }

    #[test]
    fn completion_checklist_uses_opened_skills_only() {
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

        let request = RecommendRequest {
            intent: "Analyze market risks before investing".to_string(),
            context: None,
            domain: Some("investor/analyze".to_string()),
            domain_mode: DomainMode::Boost,
            stack: None,
            changed_files: Vec::new(),
            limit: 8,
        };
        let cache = EmbeddingCache::build(&store, &embedder).unwrap();
        let results = RecommendationEngine::new(&store, &cache, &embedder)
            .recommend(&request)
            .unwrap();
        let session = SessionKey("test-session".to_string());
        let recommendation_id = store
            .create_recommendation_session(&session, &request, &results)
            .unwrap();

        let empty = store.completion_checklist(&session, None).unwrap();
        assert_eq!(
            empty.recommendation_id.as_deref(),
            Some(recommendation_id.as_str())
        );
        assert!(empty.checks.is_empty());

        let opened = store
            .mark_skill_opened(&session, "investor/analyze/market-analysis")
            .unwrap();
        assert!(opened.tracked);

        let checklist = store.completion_checklist(&session, None).unwrap();
        assert_eq!(checklist.checks.len(), 1);
        assert_eq!(
            checklist.checks[0].path,
            "investor/analyze/market-analysis"
        );
        assert!(checklist.checks[0].self_check.starts_with("## Self-Check"));
        assert!(checklist.errors.is_empty());
    }

    #[test]
    fn prune_removes_old_sessions_and_orphaned_state() {
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

        let request = RecommendRequest {
            intent: "Analyze market risks before investing".to_string(),
            context: None,
            domain: Some("investor/analyze".to_string()),
            domain_mode: DomainMode::Boost,
            stack: None,
            changed_files: Vec::new(),
            limit: 8,
        };
        let cache = EmbeddingCache::build(&store, &embedder).unwrap();
        let results = RecommendationEngine::new(&store, &cache, &embedder)
            .recommend(&request)
            .unwrap();

        // Create an old session by backdating its created_at.
        let old_session = SessionKey("old-session".to_string());
        let old_rec_id = store
            .create_recommendation_session(&old_session, &request, &results)
            .unwrap();
        store
            .connection()
            .execute(
                "UPDATE recommendation_sessions SET created_at = datetime('now', '-30 days') WHERE id = ?1",
                params![old_rec_id],
            )
            .unwrap();

        // Create a recent session that should survive.
        let recent_session = SessionKey("recent-session".to_string());
        let recent_rec_id = store
            .create_recommendation_session(&recent_session, &request, &results)
            .unwrap();

        // Prune with a 7-day retention.
        let report = store
            .prune_stale_sessions(std::time::Duration::from_secs(7 * 24 * 60 * 60))
            .unwrap();

        assert_eq!(report.deleted_sessions, 1);
        // The old session's active_recommendation_id in mcp_session_state becomes orphaned.
        assert!(report.orphaned_state >= 1);

        // The old recommendation session is gone.
        let still_exists: bool = store
            .connection()
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM recommendation_sessions WHERE id = ?1)",
                params![old_rec_id],
                |row| row.get(0),
            )
            .unwrap();
        assert!(!still_exists);

        // The recent recommendation session survives.
        let recent_exists: bool = store
            .connection()
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM recommendation_sessions WHERE id = ?1)",
                params![recent_rec_id],
                |row| row.get(0),
            )
            .unwrap();
        assert!(recent_exists);
    }

    #[test]
    fn prune_keeps_recent_sessions() {
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

        let request = RecommendRequest {
            intent: "Analyze market risks before investing".to_string(),
            context: None,
            domain: Some("investor/analyze".to_string()),
            domain_mode: DomainMode::Boost,
            stack: None,
            changed_files: Vec::new(),
            limit: 8,
        };
        let cache = EmbeddingCache::build(&store, &embedder).unwrap();
        let results = RecommendationEngine::new(&store, &cache, &embedder)
            .recommend(&request)
            .unwrap();
        let session = SessionKey("fresh-session".to_string());
        let rec_id = store
            .create_recommendation_session(&session, &request, &results)
            .unwrap();

        let report = store
            .prune_stale_sessions(std::time::Duration::from_secs(7 * 24 * 60 * 60))
            .unwrap();

        assert_eq!(report.deleted_sessions, 0);

        let still_exists: bool = store
            .connection()
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM recommendation_sessions WHERE id = ?1)",
                params![rec_id],
                |row| row.get(0),
            )
            .unwrap();
        assert!(still_exists);
    }
}
