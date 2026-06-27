use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use anyhow::{Context, Result};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ServerHandler,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::embedding::EmbeddingProvider;
use crate::recommendation::{DomainMode, RecommendRequest, RecommendationEngine};
use crate::session::SessionKey;
use crate::skill::{SkillDocument, SkillFrontmatter, SkillSummary};
use crate::storage::{FtsMatch, SkillRecord, SkillStore};
use crate::validation::{normalize_scope_path, normalize_skill_path, validate_skill_source};

#[derive(Clone)]
pub struct AbaloneServer {
    skills_root: PathBuf,
    store: Arc<Mutex<SkillStore>>,
    embedder: Arc<dyn EmbeddingProvider>,
    session_key: SessionKey,
    tool_router: ToolRouter<Self>,
}

impl fmt::Debug for AbaloneServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AbaloneServer")
            .field("skills_root", &self.skills_root)
            .field("session_key", &self.session_key)
            .finish_non_exhaustive()
    }
}

impl AbaloneServer {
    pub fn new(
        skills_root: PathBuf,
        store: SkillStore,
        embedder: Arc<dyn EmbeddingProvider>,
    ) -> Self {
        Self {
            skills_root,
            store: Arc::new(Mutex::new(store)),
            embedder,
            session_key: SessionKey("stdio".to_string()),
            tool_router: Self::tool_router(),
        }
    }

    pub fn with_session_key(mut self, session_key: SessionKey) -> Self {
        self.session_key = session_key;
        self
    }

    fn store(&self) -> Result<MutexGuard<'_, SkillStore>, String> {
        self.store
            .lock()
            .map_err(|_| "skill store lock is poisoned".to_string())
    }

    fn skill_file_path(&self, normalized_path: &str) -> PathBuf {
        normalized_path
            .split('/')
            .fold(self.skills_root.clone(), |path, segment| path.join(segment))
            .join("SKILL.md")
    }

    fn parse_document(&self, path: &str, source: String) -> Result<SkillDocument> {
        let normalized =
            normalize_skill_path(path).map_err(|issue| anyhow::anyhow!(issue.message))?;
        SkillDocument::parse(
            normalized.clone(),
            self.skill_file_path(&normalized),
            source,
        )
    }

    fn ensure_embedding_ready(&self, document: &SkillDocument) -> Result<()> {
        let vector = self.embedder.embed(&document.embedding_text())?;
        let expected = self.embedder.spec().dimension;
        if vector.len() != expected {
            anyhow::bail!(
                "embedding dimension mismatch for {}: expected {}, got {}",
                document.path,
                expected,
                vector.len()
            );
        }
        Ok(())
    }
}

#[tool_router(router = tool_router)]
impl AbaloneServer {
    #[tool(
        description = "Return concise agent-facing usage instructions for the Abalone skill workflow."
    )]
    pub fn get_usage_guide(&self) -> Result<String, String> {
        json_response(json!({
            "purpose": "Use Abalone before meaningful work starts and before claiming completion.",
            "recommended_flow": [
                "Call recommend_skills with the current work intent.",
                "Inspect the ranked metadata-only recommendations.",
                "Open only the relevant skills with get_skill.",
                "Before finishing, call get_completion_checklist.",
                "Apply every returned Self-Check section from the skills you actually opened."
            ],
            "notes": [
                "recommend_skills never returns full skill bodies.",
                "get_completion_checklist uses opened skills only.",
                "If recommendations are too broad, revise the intent and call recommend_skills again."
            ]
        }))
    }

    #[tool(description = "List skill tree scopes and skill counts without returning skill bodies.")]
    pub fn list_skill_tree(
        &self,
        Parameters(params): Parameters<ListSkillTreeParams>,
    ) -> Result<String, String> {
        let prefix = normalize_optional_scope(params.path.as_deref())?;
        let summaries = self
            .store()?
            .list_skills(prefix.as_deref())
            .map_err(to_tool_err)?;
        let entries = build_tree_entries(&summaries, prefix.as_deref(), params.depth);
        json_response(json!({
            "path": prefix,
            "depth": params.depth,
            "entries": entries
        }))
    }

    #[tool(description = "List skill summaries under an optional scope without returning bodies.")]
    pub fn list_skills(
        &self,
        Parameters(params): Parameters<ListSkillsParams>,
    ) -> Result<String, String> {
        let prefix = normalize_optional_scope(params.path.as_deref())?;
        let skills = self
            .store()?
            .list_skills(prefix.as_deref())
            .map_err(to_tool_err)?;
        json_response(json!({
            "path": prefix,
            "skills": skills
        }))
    }

    #[tool(
        description = "Recommend up to 8 ranked skill summaries for the agent's current work intent. Skill bodies are never returned."
    )]
    pub fn recommend_skills(
        &self,
        Parameters(params): Parameters<RecommendSkillsParams>,
    ) -> Result<String, String> {
        if params.intent.trim().is_empty() {
            return Err("intent must not be empty".to_string());
        }

        let request = RecommendRequest {
            intent: params.intent.trim().to_string(),
            context: non_empty(params.context),
            domain: normalize_optional_scope(params.domain.as_deref())?,
            domain_mode: parse_domain_mode(params.domain_mode.as_deref())?,
            stack: non_empty(params.stack),
            changed_files: params.changed_files.unwrap_or_default(),
            limit: params.limit.unwrap_or(8).clamp(1, 8),
        };

        let store = self.store()?;
        let engine = RecommendationEngine::new(&store, self.embedder.as_ref());
        let results = engine.recommend(&request).map_err(to_tool_err)?;
        let recommendation_id = store
            .create_recommendation_session(&self.session_key, &request, &results)
            .map_err(to_tool_err)?;

        json_response(json!({
            "recommendation_id": recommendation_id,
            "limit": request.limit,
            "results": results
        }))
    }

    #[tool(
        description = "Open a skill and return its full SKILL.md source. This records the skill as opened for completion checks."
    )]
    pub fn get_skill(
        &self,
        Parameters(params): Parameters<GetSkillParams>,
    ) -> Result<String, String> {
        let normalized = normalize_skill_path(&params.path).map_err(|issue| issue.message)?;
        let store = self.store()?;
        let record = store
            .get_skill(&normalized)
            .map_err(to_tool_err)?
            .ok_or_else(|| format!("skill not found: {normalized}"))?;
        let opened = store
            .mark_skill_opened(&self.session_key, &normalized)
            .map_err(to_tool_err)?;

        json_response(json!({
            "path": record.path,
            "name": record.name,
            "description": record.description,
            "source": record.source,
            "opened": opened
        }))
    }

    #[tool(
        description = "Return exact ## Self-Check sections from skills opened in the active recommendation session."
    )]
    pub fn get_completion_checklist(
        &self,
        Parameters(params): Parameters<GetCompletionChecklistParams>,
    ) -> Result<String, String> {
        let checklist = self
            .store()?
            .completion_checklist(&self.session_key, params.recommendation_id.as_deref())
            .map_err(to_tool_err)?;
        json_response(checklist)
    }

    #[tool(
        description = "Search indexed skill metadata and bodies without returning full skill bodies."
    )]
    pub fn search_skills(
        &self,
        Parameters(params): Parameters<SearchSkillsParams>,
    ) -> Result<String, String> {
        if params.query.trim().is_empty() {
            return Err("query must not be empty".to_string());
        }

        let domain = normalize_optional_scope(params.domain.as_deref())?;
        let domain_mode = parse_domain_mode(params.domain_mode.as_deref())?;
        let limit = params.limit.unwrap_or(8).clamp(1, 50);
        let mut matches = self
            .store()?
            .search_fts(params.query.trim(), limit)
            .map_err(to_tool_err)?;
        if matches!(domain_mode, DomainMode::Filter) {
            if let Some(domain) = &domain {
                matches.retain(|matched| {
                    matched.path == *domain || matched.path.starts_with(&format!("{domain}/"))
                });
            }
        }

        json_response(json!({
            "query": params.query,
            "domain": domain,
            "domain_mode": domain_mode.as_str(),
            "results": matches
        }))
    }

    #[tool(
        description = "Validate an existing skill by path or a full SKILL.md body before writing it."
    )]
    pub fn validate_skill(
        &self,
        Parameters(params): Parameters<ValidateSkillParams>,
    ) -> Result<String, String> {
        let path = params
            .path
            .as_deref()
            .ok_or_else(|| "path is required for validation".to_string())?;
        let normalized = normalize_skill_path(path).map_err(|issue| issue.message)?;

        let source = match params.body {
            Some(body) => body,
            None => {
                self.store()?
                    .get_skill(&normalized)
                    .map_err(to_tool_err)?
                    .ok_or_else(|| format!("skill not found: {normalized}"))?
                    .source
            }
        };

        let report = validate_skill_source(&normalized, &source);
        json_response(report)
    }

    #[tool(
        description = "Create a skill after validating the exact required format. Failed validation does not write files or indexes."
    )]
    pub fn create_skill(
        &self,
        Parameters(params): Parameters<CreateSkillParams>,
    ) -> Result<String, String> {
        let normalized = normalize_skill_path(&params.path).map_err(|issue| issue.message)?;
        let file_path = self.skill_file_path(&normalized);

        if file_path.exists() {
            return Err(format!(
                "skill file already exists: {}",
                file_path.display()
            ));
        }
        if self
            .store()?
            .get_skill(&normalized)
            .map_err(to_tool_err)?
            .is_some()
        {
            return Err(format!("skill already exists in index: {normalized}"));
        }

        let source = build_skill_source(&params.name, &params.description, &params.body)
            .map_err(to_tool_err)?;
        let report = validate_skill_source(&normalized, &source);
        if !report.ok {
            return json_response(json!({
                "created": false,
                "validation": report
            }));
        }

        let document = self
            .parse_document(&normalized, source.clone())
            .map_err(to_tool_err)?;
        self.ensure_embedding_ready(&document)
            .map_err(to_tool_err)?;
        atomic_write(&file_path, &source).map_err(to_tool_err)?;

        let index_result = {
            let store = self.store()?;
            store
                .upsert_skill(&document)
                .and_then(|_| store.upsert_embedding_if_needed(&document, self.embedder.as_ref()))
        };
        if let Err(err) = index_result {
            let _ = fs::remove_file(&file_path);
            return Err(err.to_string());
        }

        json_response(json!({
            "created": true,
            "path": normalized,
            "file_path": file_path
        }))
    }

    #[tool(
        description = "Update a skill after validating the exact required format. Failed validation does not write files or indexes."
    )]
    pub fn update_skill(
        &self,
        Parameters(params): Parameters<UpdateSkillParams>,
    ) -> Result<String, String> {
        let normalized = normalize_skill_path(&params.path).map_err(|issue| issue.message)?;
        let existing = self
            .store()?
            .get_skill(&normalized)
            .map_err(to_tool_err)?
            .ok_or_else(|| format!("skill not found: {normalized}"))?;
        let file_path = PathBuf::from(&existing.file_path);

        let description = params.description.unwrap_or(existing.description.clone());
        let body = params.body.unwrap_or(existing.body.clone());
        let source =
            build_skill_source(&existing.name, &description, &body).map_err(to_tool_err)?;
        let report = validate_skill_source(&normalized, &source);
        if !report.ok {
            return json_response(json!({
                "updated": false,
                "validation": report
            }));
        }

        let document = self
            .parse_document(&normalized, source.clone())
            .map_err(to_tool_err)?;
        self.ensure_embedding_ready(&document)
            .map_err(to_tool_err)?;

        let old_source = fs::read_to_string(&file_path).unwrap_or(existing.source.clone());
        atomic_write(&file_path, &source).map_err(to_tool_err)?;

        let index_result = {
            let store = self.store()?;
            store
                .upsert_skill(&document)
                .and_then(|_| store.upsert_embedding_if_needed(&document, self.embedder.as_ref()))
        };
        if let Err(err) = index_result {
            let _ = atomic_write(&file_path, &old_source);
            if let Ok(old_doc) = self.parse_document(&normalized, old_source) {
                if let Ok(store) = self.store.lock() {
                    let _ = store.upsert_skill(&old_doc);
                    let _ = store.upsert_embedding_if_needed(&old_doc, self.embedder.as_ref());
                }
            }
            return Err(err.to_string());
        }

        json_response(json!({
            "updated": true,
            "path": normalized,
            "file_path": file_path
        }))
    }

    #[tool(description = "Delete a skill file and remove it from the index.")]
    pub fn delete_skill(
        &self,
        Parameters(params): Parameters<DeleteSkillParams>,
    ) -> Result<String, String> {
        let normalized = normalize_skill_path(&params.path).map_err(|issue| issue.message)?;
        let existing = self
            .store()?
            .get_skill(&normalized)
            .map_err(to_tool_err)?
            .ok_or_else(|| format!("skill not found: {normalized}"))?;

        let file_path = PathBuf::from(&existing.file_path);
        if file_path.exists() {
            fs::remove_file(&file_path).map_err(to_tool_err)?;
        }
        let deleted = self
            .store()?
            .delete_skill(&normalized)
            .map_err(to_tool_err)?;

        json_response(json!({
            "deleted": deleted,
            "path": normalized,
            "file_path": file_path
        }))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for AbaloneServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("Abalone recommends existing skill guidance to agents and returns completion checks from opened skills.")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListSkillTreeParams {
    pub path: Option<String>,
    pub depth: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListSkillsParams {
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RecommendSkillsParams {
    pub intent: String,
    pub context: Option<String>,
    pub domain: Option<String>,
    pub domain_mode: Option<String>,
    pub stack: Option<String>,
    pub changed_files: Option<Vec<String>>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetSkillParams {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetCompletionChecklistParams {
    pub recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchSkillsParams {
    pub query: String,
    pub domain: Option<String>,
    pub domain_mode: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ValidateSkillParams {
    pub path: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateSkillParams {
    pub path: String,
    pub name: String,
    pub description: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateSkillParams {
    pub path: String,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteSkillParams {
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SkillTreeEntry {
    path: String,
    depth: usize,
    skill_count: usize,
    is_skill: bool,
}

fn build_tree_entries(
    summaries: &[SkillSummary],
    prefix: Option<&str>,
    max_depth: Option<usize>,
) -> Vec<SkillTreeEntry> {
    let base_len = prefix
        .filter(|path| !path.is_empty())
        .map(|path| path.split('/').count())
        .unwrap_or(0);
    let mut entries: BTreeMap<String, SkillTreeEntry> = BTreeMap::new();

    for summary in summaries {
        let segments = summary.path.split('/').collect::<Vec<_>>();
        for idx in (base_len + 1)..=segments.len() {
            let depth = idx - base_len;
            if max_depth.is_some_and(|max| depth > max) {
                break;
            }
            let path = segments[..idx].join("/");
            let entry = entries
                .entry(path.clone())
                .or_insert_with(|| SkillTreeEntry {
                    path,
                    depth,
                    skill_count: 0,
                    is_skill: idx == segments.len(),
                });
            entry.skill_count += 1;
            entry.is_skill |= idx == segments.len();
        }
    }

    entries.into_values().collect()
}

fn build_skill_source(name: &str, description: &str, body: &str) -> Result<String> {
    let frontmatter = SkillFrontmatter {
        name: name.trim().to_string(),
        description: description.trim().to_string(),
        disable_model_invocation: false,
    };
    let yaml = serde_yaml::to_string(&frontmatter)?;
    Ok(format!("---\n{}---\n\n{}", yaml, body.trim_start()))
}

fn atomic_write(path: &Path, source: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    let temp_path = path.with_extension(format!("SKILL.md.tmp.{}", uuid::Uuid::new_v4().simple()));
    fs::write(&temp_path, source)
        .with_context(|| format!("failed to write {}", temp_path.display()))?;
    fs::rename(&temp_path, path).with_context(|| {
        format!(
            "failed to atomically move {} to {}",
            temp_path.display(),
            path.display()
        )
    })?;
    Ok(())
}

fn normalize_optional_scope(path: Option<&str>) -> Result<Option<String>, String> {
    let Some(path) = path else {
        return Ok(None);
    };
    if path.trim().is_empty() {
        return Ok(None);
    }
    normalize_scope_path(path)
        .map(Some)
        .map_err(|issue| issue.message)
}

fn parse_domain_mode(mode: Option<&str>) -> Result<DomainMode, String> {
    match mode.unwrap_or("boost").trim().to_ascii_lowercase().as_str() {
        "boost" => Ok(DomainMode::Boost),
        "filter" => Ok(DomainMode::Filter),
        "none" => Ok(DomainMode::None),
        other => Err(format!(
            "invalid domain_mode `{other}`; expected boost, filter, or none"
        )),
    }
}

fn non_empty(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn json_response(value: impl Serialize) -> Result<String, String> {
    serde_json::to_string_pretty(&value).map_err(|err| err.to_string())
}

fn to_tool_err(err: impl fmt::Display) -> String {
    err.to_string()
}

impl Serialize for FtsMatch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct SerializableFtsMatch<'a> {
            path: &'a str,
            name: &'a str,
            description: &'a str,
            lexical_score: f32,
        }

        SerializableFtsMatch {
            path: &self.path,
            name: &self.name,
            description: &self.description,
            lexical_score: self.lexical_score,
        }
        .serialize(serializer)
    }
}

impl Serialize for SkillRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct SerializableSkillRecord<'a> {
            path: &'a str,
            file_path: &'a str,
            name: &'a str,
            description: &'a str,
            disable_model_invocation: bool,
            content_hash: &'a str,
        }

        SerializableSkillRecord {
            path: &self.path,
            file_path: &self.file_path,
            name: &self.name,
            description: &self.description,
            disable_model_invocation: self.disable_model_invocation,
            content_hash: &self.content_hash,
        }
        .serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Arc;
    use std::time::Duration;

    use rmcp::model::{CallToolRequestParams, ClientInfo};
    use rmcp::ServiceExt;
    use serde_json::Value;
    use tempfile::TempDir;

    use super::{
        AbaloneServer, CreateSkillParams, GetCompletionChecklistParams, GetSkillParams,
        ListSkillsParams, RecommendSkillsParams, UpdateSkillParams,
    };
    use crate::embedding::DeterministicEmbeddingProvider;
    use crate::storage::SkillStore;

    #[test]
    fn tool_flow_recommends_opens_and_returns_completion_check() {
        let temp = TempDir::new().unwrap();
        let skill_dir = temp
            .path()
            .join("investor")
            .join("analyze")
            .join("market")
            .join("market-analysis");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(
            skill_dir.join("SKILL.md"),
            include_str!("../../skills/investor/analyze/market/market-analysis/SKILL.md"),
        )
        .unwrap();

        let store = SkillStore::in_memory().unwrap();
        let embedder = Arc::new(DeterministicEmbeddingProvider::new(32));
        store
            .sync_filesystem(temp.path(), Some(embedder.as_ref()))
            .unwrap();
        let server = AbaloneServer::new(temp.path().to_path_buf(), store, embedder);

        let listed: Value = serde_json::from_str(
            &server
                .list_skills(rmcp::handler::server::wrapper::Parameters(
                    ListSkillsParams {
                        path: Some("investor".to_string()),
                    },
                ))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(listed["skills"].as_array().unwrap().len(), 1);

        let recommended: Value = serde_json::from_str(
            &server
                .recommend_skills(rmcp::handler::server::wrapper::Parameters(
                    RecommendSkillsParams {
                        intent: "what to consider before investing in stocks".to_string(),
                        context: None,
                        domain: Some("investor".to_string()),
                        domain_mode: Some("boost".to_string()),
                        stack: None,
                        changed_files: None,
                        limit: None,
                    },
                ))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            recommended["results"][0]["path"],
            "investor/analyze/market/market-analysis"
        );

        let opened: Value = serde_json::from_str(
            &server
                .get_skill(rmcp::handler::server::wrapper::Parameters(GetSkillParams {
                    path: "investor/analyze/market/market-analysis".to_string(),
                }))
                .unwrap(),
        )
        .unwrap();
        assert!(opened["source"].as_str().unwrap().contains("## Core Rules"));

        let checklist: Value = serde_json::from_str(
            &server
                .get_completion_checklist(rmcp::handler::server::wrapper::Parameters(
                    GetCompletionChecklistParams {
                        recommendation_id: None,
                    },
                ))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(checklist["checks"].as_array().unwrap().len(), 1);
        assert!(checklist["checks"][0]["self_check"]
            .as_str()
            .unwrap()
            .starts_with("## Self-Check"));
    }

    #[test]
    fn public_tool_schemas_do_not_expose_session_arguments() {
        let temp = TempDir::new().unwrap();
        let store = SkillStore::in_memory().unwrap();
        let embedder = Arc::new(DeterministicEmbeddingProvider::new(32));
        let server = AbaloneServer::new(temp.path().to_path_buf(), store, embedder);

        for tool in server.tool_router.list_all() {
            let schema = serde_json::to_string(&tool.input_schema).unwrap();
            assert!(
                !schema.contains("session_id")
                    && !schema.contains("sessionId")
                    && !schema.contains("__sessionId"),
                "{} leaked a manual session argument: {schema}",
                tool.name
            );
        }
    }

    #[test]
    fn create_and_update_validation_failures_are_atomic() {
        let temp = TempDir::new().unwrap();
        let store = SkillStore::in_memory().unwrap();
        let embedder = Arc::new(DeterministicEmbeddingProvider::new(32));
        let server = AbaloneServer::new(temp.path().to_path_buf(), store, embedder);

        let path = "programmer/api/response/data-minimization";
        let invalid_body = "# Tiny\n\n## Core Rules\nShort.\n\n## Common Traps\nShort.\n\n## Self-Check\n- [ ] Short.";
        let create_response: Value = serde_json::from_str(
            &server
                .create_skill(rmcp::handler::server::wrapper::Parameters(CreateSkillParams {
                    path: path.to_string(),
                    name: "data_minimization.md".to_string(),
                    description: "Use when the agent is designing API response fields before exposing private persistence data, authorization-sensitive fields, or derived user profile information.".to_string(),
                    body: invalid_body.to_string(),
                }))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(create_response["created"], false);
        assert!(!temp
            .path()
            .join("programmer/api/response/data-minimization/SKILL.md")
            .exists());
        assert!(server.store().unwrap().get_skill(path).unwrap().is_none());

        let valid_body = substantial_body("API Response Data Minimization");
        let create_response: Value = serde_json::from_str(
            &server
                .create_skill(rmcp::handler::server::wrapper::Parameters(CreateSkillParams {
                    path: path.to_string(),
                    name: "data_minimization.md".to_string(),
                    description: "Use when the agent is designing API response fields before exposing private persistence data, authorization-sensitive fields, or derived user profile information.".to_string(),
                    body: valid_body,
                }))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(create_response["created"], true);

        let file_path = temp
            .path()
            .join("programmer/api/response/data-minimization/SKILL.md");
        let old_file_source = fs::read_to_string(&file_path).unwrap();
        let old_index_source = server
            .store()
            .unwrap()
            .get_skill(path)
            .unwrap()
            .unwrap()
            .source;

        let update_response: Value = serde_json::from_str(
            &server
                .update_skill(rmcp::handler::server::wrapper::Parameters(
                    UpdateSkillParams {
                        path: path.to_string(),
                        description: None,
                        body: Some(invalid_body.to_string()),
                    },
                ))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(update_response["updated"], false);
        assert_eq!(fs::read_to_string(&file_path).unwrap(), old_file_source);
        assert_eq!(
            server
                .store()
                .unwrap()
                .get_skill(path)
                .unwrap()
                .unwrap()
                .source,
            old_index_source
        );
    }

    #[test]
    fn completion_checklist_returns_all_opened_skills_in_open_order() {
        let temp = TempDir::new().unwrap();
        let store = SkillStore::in_memory().unwrap();
        let embedder = Arc::new(DeterministicEmbeddingProvider::new(32));
        let server = AbaloneServer::new(temp.path().to_path_buf(), store, embedder);

        create_valid_test_skill(
            &server,
            "programmer/api/response/data-minimization",
            "data_minimization.md",
            "Use when the agent is designing API response fields before exposing private persistence data, authorization-sensitive fields, or derived user profile information.",
            "API Response Data Minimization",
        );
        create_valid_test_skill(
            &server,
            "programmer/database/schema/query-safety",
            "query_safety.md",
            "Use when the agent is changing database schema, query behavior, persistence boundaries, indexing, or data access paths before implementing storage work.",
            "Database Query Safety",
        );

        server
            .recommend_skills(rmcp::handler::server::wrapper::Parameters(
                RecommendSkillsParams {
                    intent: "API response and database persistence work".to_string(),
                    context: None,
                    domain: Some("programmer".to_string()),
                    domain_mode: Some("boost".to_string()),
                    stack: None,
                    changed_files: None,
                    limit: None,
                },
            ))
            .unwrap();
        server
            .get_skill(rmcp::handler::server::wrapper::Parameters(GetSkillParams {
                path: "programmer/api/response/data-minimization".to_string(),
            }))
            .unwrap();
        server
            .get_skill(rmcp::handler::server::wrapper::Parameters(GetSkillParams {
                path: "programmer/database/schema/query-safety".to_string(),
            }))
            .unwrap();

        let checklist: Value = serde_json::from_str(
            &server
                .get_completion_checklist(rmcp::handler::server::wrapper::Parameters(
                    GetCompletionChecklistParams {
                        recommendation_id: None,
                    },
                ))
                .unwrap(),
        )
        .unwrap();
        let checks = checklist["checks"].as_array().unwrap();
        assert_eq!(checks.len(), 2);
        assert_eq!(
            checks[0]["path"],
            "programmer/api/response/data-minimization"
        );
        assert_eq!(checks[1]["path"], "programmer/database/schema/query-safety");
    }

    #[tokio::test]
    async fn mcp_transport_recommends_opens_and_returns_completion_check() -> anyhow::Result<()> {
        let temp = TempDir::new().unwrap();
        let skill_dir = temp
            .path()
            .join("investor")
            .join("analyze")
            .join("market")
            .join("market-analysis");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(
            skill_dir.join("SKILL.md"),
            include_str!("../../skills/investor/analyze/market/market-analysis/SKILL.md"),
        )
        .unwrap();

        let store = SkillStore::in_memory().unwrap();
        let embedder = Arc::new(DeterministicEmbeddingProvider::new(32));
        store
            .sync_filesystem(temp.path(), Some(embedder.as_ref()))
            .unwrap();
        let server = AbaloneServer::new(temp.path().to_path_buf(), store, embedder);
        let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);

        let server_handle = tokio::spawn(async move {
            server.serve(server_transport).await?.waiting().await?;
            anyhow::Ok(())
        });
        let mut client = ClientInfo::default().serve(client_transport).await?;

        let recommended = client
            .call_tool(
                CallToolRequestParams::new("recommend_skills").with_arguments(
                    serde_json::json!({
                        "intent": "what to consider before investing in stocks",
                        "domain": "investor",
                        "domain_mode": "boost",
                        "limit": 8
                    })
                    .as_object()
                    .unwrap()
                    .clone(),
                ),
            )
            .await?;
        let recommended = tool_result_json(recommended)?;
        assert_eq!(
            recommended["results"][0]["path"],
            "investor/analyze/market/market-analysis"
        );

        let opened = client
            .call_tool(
                CallToolRequestParams::new("get_skill").with_arguments(
                    serde_json::json!({
                        "path": "investor/analyze/market/market-analysis"
                    })
                    .as_object()
                    .unwrap()
                    .clone(),
                ),
            )
            .await?;
        let opened = tool_result_json(opened)?;
        assert!(opened["source"].as_str().unwrap().contains("## Core Rules"));

        let checklist = client
            .call_tool(CallToolRequestParams::new("get_completion_checklist"))
            .await?;
        let checklist = tool_result_json(checklist)?;
        assert_eq!(checklist["checks"].as_array().unwrap().len(), 1);
        assert!(checklist["checks"][0]["self_check"]
            .as_str()
            .unwrap()
            .starts_with("## Self-Check"));

        client.close().await?;
        tokio::time::timeout(Duration::from_secs(5), server_handle).await???;
        Ok(())
    }

    fn tool_result_json(result: rmcp::model::CallToolResult) -> anyhow::Result<Value> {
        let text = result
            .content
            .first()
            .and_then(|content| content.raw.as_text())
            .map(|text| text.text.as_str())
            .ok_or_else(|| anyhow::anyhow!("expected text tool result"))?;
        Ok(serde_json::from_str(text)?)
    }

    fn create_valid_test_skill(
        server: &AbaloneServer,
        path: &str,
        name: &str,
        description: &str,
        title: &str,
    ) {
        let response: Value = serde_json::from_str(
            &server
                .create_skill(rmcp::handler::server::wrapper::Parameters(
                    CreateSkillParams {
                        path: path.to_string(),
                        name: name.to_string(),
                        description: description.to_string(),
                        body: substantial_body(title),
                    },
                ))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(response["created"], true);
    }

    fn substantial_body(title: &str) -> String {
        let core = "Define the boundary before writing implementation details. Separate public contract, private state, caller authority, failure behavior, observability, migration risk, and rollback conditions. ";
        let traps = "Do not optimize for the easiest implementation when the boundary decides who can see data, mutate state, depend on ordering, or infer sensitive facts from errors. ";
        format!(
            "# {title}\n\n## Core Rules\n\n{}\n\n## Common Traps\n\n{}\n\n## Self-Check\n\n- [ ] Public contract and private implementation state are separated.\n- [ ] Authorization, failure behavior, and rollback expectations were checked.\n- [ ] The implementation was reviewed against caller-visible side effects.\n",
            core.repeat(35),
            traps.repeat(35)
        )
    }
}
