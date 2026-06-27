use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use serde::Serialize;

use crate::embedding::{cosine_similarity, EmbeddingProvider};
use crate::storage::{FtsMatch, SkillRecord, SkillStore};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RecommendationResult {
    pub rank: usize,
    pub path: String,
    pub name: String,
    pub description: String,
    pub score: f32,
    pub reason: String,
    pub evidence: RecommendationEvidence,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RecommendationEvidence {
    pub lexical_score: f32,
    pub semantic_score: f32,
    pub matched_fields: Vec<String>,
    pub matched_terms: Vec<String>,
    pub domain_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendRequest {
    pub intent: String,
    pub context: Option<String>,
    pub domain: Option<String>,
    pub domain_mode: DomainMode,
    pub stack: Option<String>,
    pub changed_files: Vec<String>,
    pub limit: usize,
}

impl RecommendRequest {
    pub fn query_text(&self) -> String {
        let mut parts = vec![self.intent.clone()];
        if let Some(context) = &self.context {
            parts.push(context.clone());
        }
        if let Some(stack) = &self.stack {
            parts.push(stack.clone());
        }
        if !self.changed_files.is_empty() {
            parts.push(self.changed_files.join(" "));
        }
        parts.join("\n")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainMode {
    Boost,
    Filter,
    None,
}

impl DomainMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Boost => "boost",
            Self::Filter => "filter",
            Self::None => "none",
        }
    }
}

impl Default for DomainMode {
    fn default() -> Self {
        Self::Boost
    }
}

pub struct RecommendationEngine<'a> {
    store: &'a SkillStore,
    embedder: &'a dyn EmbeddingProvider,
}

impl<'a> RecommendationEngine<'a> {
    pub fn new(store: &'a SkillStore, embedder: &'a dyn EmbeddingProvider) -> Self {
        Self { store, embedder }
    }

    pub fn recommend(&self, request: &RecommendRequest) -> Result<Vec<RecommendationResult>> {
        let query = request.query_text();
        let query_embedding = self.embedder.embed(&query)?;
        let embedding_map = self
            .store
            .embeddings_for_model(&self.embedder.spec().name)?
            .into_iter()
            .collect::<HashMap<_, _>>();

        let records = self.store.all_skill_records()?;
        let considered = records
            .into_iter()
            .filter(|record| !record.disable_model_invocation)
            .filter(|record| domain_allows(record, request))
            .collect::<Vec<_>>();

        if !considered.is_empty() && embedding_map.is_empty() {
            anyhow::bail!(
                "recommendation index has no embeddings for model {}",
                self.embedder.spec().name
            );
        }

        let fts_matches = self
            .store
            .search_fts(&query, 200)?
            .into_iter()
            .map(|matched| (matched.path.clone(), matched))
            .collect::<HashMap<_, _>>();

        let query_terms = query_terms(&query);
        let mut scored = Vec::new();

        for record in considered {
            let embedding = embedding_map.get(&record.path).with_context(|| {
                format!(
                    "missing embedding for {} with model {}",
                    record.path,
                    self.embedder.spec().name
                )
            })?;

            let semantic_score = cosine_similarity(&query_embedding, embedding).max(0.0) * 100.0;
            let fts_match = fts_matches.get(&record.path);
            let field_score = lexical_field_score(&record, &query_terms);
            let fts_score = fts_match
                .map(|matched| matched.lexical_score)
                .unwrap_or(0.0);
            let lexical_score = field_score + fts_score;
            let scope_score = scope_score(&record, request);

            let final_score = semantic_score * 0.65 + lexical_score * 1.5 + scope_score;
            let matched_fields = matched_fields(&record, &query_terms, fts_match);
            let matched_terms = matched_terms(&record, &query_terms);

            if final_score <= 0.0 {
                continue;
            }

            scored.push(RecommendationResult {
                rank: 0,
                path: record.path.clone(),
                name: record.name.clone(),
                description: record.description.clone(),
                score: final_score,
                reason: reason_for(&record, semantic_score, lexical_score, scope_score),
                evidence: RecommendationEvidence {
                    lexical_score,
                    semantic_score,
                    matched_fields,
                    matched_terms,
                    domain_mode: request.domain_mode.as_str().to_string(),
                },
            });
        }

        scored.sort_by(|left, right| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| left.path.cmp(&right.path))
        });

        let limit = request.limit.clamp(1, 8);
        for (idx, result) in scored.iter_mut().take(limit).enumerate() {
            result.rank = idx + 1;
        }
        scored.truncate(limit);
        Ok(scored)
    }
}

fn domain_allows(record: &SkillRecord, request: &RecommendRequest) -> bool {
    match (request.domain_mode, request.domain.as_deref()) {
        (DomainMode::Filter, Some(domain)) => {
            record.path == domain || record.path.starts_with(&format!("{domain}/"))
        }
        _ => true,
    }
}

fn scope_score(record: &SkillRecord, request: &RecommendRequest) -> f32 {
    let mut score = 0.0;
    if let (DomainMode::Boost | DomainMode::Filter, Some(domain)) =
        (request.domain_mode, request.domain.as_deref())
    {
        if record.path == domain || record.path.starts_with(&format!("{domain}/")) {
            score += 20.0;
        }
    }

    if let Some(stack) = &request.stack {
        for token in query_terms(stack) {
            if record.path.to_ascii_lowercase().contains(&token) {
                score += 3.0;
            }
        }
    }

    score
}

fn lexical_field_score(record: &SkillRecord, terms: &[String]) -> f32 {
    let mut score = 0.0;
    let path = record.path.to_ascii_lowercase();
    let name = record.name.to_ascii_lowercase();
    let description = record.description.to_ascii_lowercase();

    for term in terms {
        if path.contains(term) {
            score += 6.0;
        }
        if name.contains(term) {
            score += 5.0;
        }
        if description.contains(term) {
            score += 4.0;
        }
    }

    score
}

fn matched_fields(
    record: &SkillRecord,
    terms: &[String],
    fts_match: Option<&FtsMatch>,
) -> Vec<String> {
    let mut fields = HashSet::new();
    let path = record.path.to_ascii_lowercase();
    let name = record.name.to_ascii_lowercase();
    let description = record.description.to_ascii_lowercase();

    for term in terms {
        if path.contains(term) {
            fields.insert("path".to_string());
        }
        if name.contains(term) {
            fields.insert("name".to_string());
        }
        if description.contains(term) {
            fields.insert("description".to_string());
        }
    }
    if fts_match.is_some() {
        fields.insert("body".to_string());
    }

    let mut fields = fields.into_iter().collect::<Vec<_>>();
    fields.sort();
    fields
}

fn matched_terms(record: &SkillRecord, terms: &[String]) -> Vec<String> {
    let haystack = format!(
        "{} {} {}",
        record.path.to_ascii_lowercase(),
        record.name.to_ascii_lowercase(),
        record.description.to_ascii_lowercase()
    );
    let mut matched = terms
        .iter()
        .filter(|term| haystack.contains(term.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    matched.sort();
    matched.dedup();
    matched
}

fn reason_for(
    record: &SkillRecord,
    semantic_score: f32,
    lexical_score: f32,
    scope_score: f32,
) -> String {
    let mut reasons = Vec::new();
    if semantic_score > 0.0 {
        reasons.push(format!("semantic match {:.1}", semantic_score));
    }
    if lexical_score > 0.0 {
        reasons.push(format!("lexical/path evidence {:.1}", lexical_score));
    }
    if scope_score > 0.0 {
        reasons.push(format!("scope boost {:.1}", scope_score));
    }

    if reasons.is_empty() {
        format!("Recommended by ranking signals for {}", record.path)
    } else {
        format!("{}: {}", record.path, reasons.join(", "))
    }
}

pub fn query_terms(query: &str) -> Vec<String> {
    let mut terms = query
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| token.len() >= 2)
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();
    terms.sort();
    terms.dedup();
    terms
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::{DomainMode, RecommendRequest, RecommendationEngine};
    use crate::embedding::DeterministicEmbeddingProvider;
    use crate::storage::SkillStore;

    #[test]
    fn recommends_market_skill_for_broad_investing_intent() {
        let mut store = SkillStore::in_memory().unwrap();
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

        let embedder = DeterministicEmbeddingProvider::new(32);
        store.sync_filesystem(temp.path(), Some(&embedder)).unwrap();

        let engine = RecommendationEngine::new(&store, &embedder);
        let results = engine
            .recommend(&RecommendRequest {
                intent: "Analyze what to consider before investing in stocks".to_string(),
                context: None,
                domain: Some("investor/analyze".to_string()),
                domain_mode: DomainMode::Boost,
                stack: None,
                changed_files: Vec::new(),
                limit: 8,
            })
            .unwrap();

        assert_eq!(results[0].path, "investor/analyze/market/market-analysis");
        assert_eq!(results[0].rank, 1);
        assert!(results[0].evidence.semantic_score > 0.0);
    }
}
