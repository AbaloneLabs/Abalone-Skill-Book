use std::path::PathBuf;

use anyhow::{Context, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub skills_root: PathBuf,
    pub database_path: PathBuf,
    pub model_dir: PathBuf,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self> {
        let cwd = std::env::current_dir().context("failed to read current directory")?;
        let repo_root = if cwd.join("skills").is_dir() {
            cwd.clone()
        } else if cwd
            .parent()
            .is_some_and(|parent| parent.join("skills").is_dir())
        {
            cwd.parent()
                .map(PathBuf::from)
                .unwrap_or_else(|| cwd.clone())
        } else {
            cwd.clone()
        };

        let skills_root =
            env_path("ABALONE_SKILLS_ROOT").unwrap_or_else(|| repo_root.join("skills"));
        let database_path = env_path("ABALONE_DATABASE_PATH")
            .unwrap_or_else(|| repo_root.join(".abalone").join("abalone.sqlite3"));
        let model_dir = env_path("ABALONE_MODEL_DIR")
            .unwrap_or_else(|| repo_root.join("models").join("bge-m3"));

        Ok(Self {
            skills_root,
            database_path,
            model_dir,
        })
    }
}

fn env_path(key: &str) -> Option<PathBuf> {
    std::env::var_os(key)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}
