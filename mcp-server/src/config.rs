use std::path::PathBuf;

use anyhow::{Context, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub skills_root: PathBuf,
    pub database_path: PathBuf,
    pub model_dir: PathBuf,
    pub http: HttpConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
    pub path: String,
    pub allowed_hosts: Vec<String>,
    pub allowed_origins: Vec<String>,
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
        let http = HttpConfig::from_env()?;

        Ok(Self {
            skills_root,
            database_path,
            model_dir,
            http,
        })
    }
}

impl HttpConfig {
    pub fn from_env() -> Result<Self> {
        let host = env_string("ABALONE_HTTP_HOST").unwrap_or_else(|| "127.0.0.1".to_string());
        let port = env_parse("ABALONE_HTTP_PORT")?.unwrap_or(8732);
        let path = normalize_http_path(
            env_string("ABALONE_MCP_PATH")
                .unwrap_or_else(|| "/mcp".to_string())
                .as_str(),
        );
        let allowed_hosts = env_list("ABALONE_HTTP_ALLOWED_HOSTS").unwrap_or_else(|| {
            let mut hosts = vec![
                "localhost".to_string(),
                "127.0.0.1".to_string(),
                "::1".to_string(),
            ];
            if host != "0.0.0.0" && host != "::" && !hosts.iter().any(|value| value == &host) {
                hosts.push(host.clone());
            }
            hosts
        });
        let allowed_origins = env_list("ABALONE_HTTP_ALLOWED_ORIGINS").unwrap_or_default();

        Ok(Self {
            host,
            port,
            path,
            allowed_hosts,
            allowed_origins,
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn env_path(key: &str) -> Option<PathBuf> {
    std::env::var_os(key)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn env_string(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn env_parse<T>(key: &str) -> Result<Option<T>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    env_string(key)
        .map(|value| {
            value
                .parse::<T>()
                .with_context(|| format!("invalid {key}: {value}"))
        })
        .transpose()
}

fn env_list(key: &str) -> Option<Vec<String>> {
    env_string(key).map(|value| {
        value
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    })
}

fn normalize_http_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() || trimmed == "/" {
        return "/mcp".to_string();
    }
    let without_trailing = trimmed.trim_end_matches('/');
    if without_trailing.starts_with('/') {
        without_trailing.to_string()
    } else {
        format!("/{without_trailing}")
    }
}
