use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionKey(pub String);

impl SessionKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationSession {
    pub id: String,
    pub mcp_session_key: SessionKey,
    pub intent: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenSkillOutcome {
    pub path: String,
    pub tracked: bool,
    pub recommendation_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionChecklist {
    pub recommendation_id: Option<String>,
    pub checks: Vec<CompletionCheck>,
    pub errors: Vec<CompletionError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionCheck {
    pub path: String,
    pub name: String,
    pub self_check: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionError {
    pub path: String,
    pub code: String,
    pub message: String,
}
