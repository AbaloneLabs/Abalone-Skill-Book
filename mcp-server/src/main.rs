use std::sync::{Arc, Mutex};
use std::time::Duration;

use abalone_mcp_server::config::ServerConfig;
use abalone_mcp_server::embedding::{
    BgeM3EmbeddingProvider, DeterministicEmbeddingProvider, EmbeddingProvider,
};
use abalone_mcp_server::storage::SkillStore;
use abalone_mcp_server::tools::AbaloneServer;
use anyhow::{Context, Result};
use axum::{routing::get, Router};
use rmcp::{
    transport::{
        stdio,
        streamable_http_server::{
            session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
        },
    },
    ServiceExt,
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let run_mode = RunMode::from_args()?;
    let config = ServerConfig::from_env()?;
    tracing::info!(
        skills_root = %config.skills_root.display(),
        database_path = %config.database_path.display(),
        model_dir = %config.model_dir.display(),
        "starting abalone MCP server"
    );

    let embedder = load_embedding_provider(&config)?;
    tracing::info!(
        embedding_model = %embedder.spec().name,
        embedding_dimension = embedder.spec().dimension,
        "loaded embedding provider"
    );

    let mut store = SkillStore::open(&config.database_path).with_context(|| {
        format!(
            "failed to open database at {}",
            config.database_path.display()
        )
    })?;
    let sync_report = store
        .sync_filesystem(&config.skills_root, Some(embedder.as_ref()))
        .with_context(|| {
            format!(
                "failed to sync skills from {}",
                config.skills_root.display()
            )
        })?;
    tracing::info!(
        indexed = sync_report.indexed,
        removed = sync_report.removed,
        invalid = sync_report.invalid.len(),
        "skill sync complete"
    );
    for invalid in &sync_report.invalid {
        tracing::warn!(
            path = %invalid.path,
            errors = ?invalid.report.errors,
            "invalid skill excluded from index"
        );
    }

    let server = AbaloneServer::from_shared_store(
        config.skills_root.clone(),
        Arc::new(Mutex::new(store)),
        embedder,
    );

    match run_mode {
        RunMode::Serve => serve_http(config, server).await?,
        RunMode::Stdio => {
            server.serve(stdio()).await?.waiting().await?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RunMode {
    Serve,
    Stdio,
}

impl RunMode {
    fn from_args() -> Result<Self> {
        match std::env::args().nth(1).as_deref() {
            None | Some("serve") | Some("http") => Ok(Self::Serve),
            Some("stdio") => Ok(Self::Stdio),
            Some("-h") | Some("--help") | Some("help") => {
                print_help();
                std::process::exit(0);
            }
            Some(other) => anyhow::bail!(
                "unknown command `{other}`; expected `serve` for HTTP or `stdio` for subprocess mode"
            ),
        }
    }
}

async fn serve_http(config: ServerConfig, server: AbaloneServer) -> Result<()> {
    let cancellation_token = CancellationToken::new();
    let service: StreamableHttpService<AbaloneServer, LocalSessionManager> =
        StreamableHttpService::new(
            move || Ok(server.clone()),
            Arc::new(LocalSessionManager::default()),
            StreamableHttpServerConfig::default()
                .with_allowed_hosts(config.http.allowed_hosts.clone())
                .with_allowed_origins(config.http.allowed_origins.clone())
                .with_sse_keep_alive(Some(Duration::from_secs(15)))
                .with_cancellation_token(cancellation_token.child_token()),
        );

    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .nest_service(&config.http.path, service);

    let listener = tokio::net::TcpListener::bind(config.http.bind_addr())
        .await
        .with_context(|| format!("failed to bind {}", config.http.bind_addr()))?;
    let local_addr = listener.local_addr()?;
    tracing::info!(
        endpoint = %format!("http://{local_addr}{}", config.http.path),
        allowed_hosts = ?config.http.allowed_hosts,
        allowed_origins = ?config.http.allowed_origins,
        "serving Abalone MCP Streamable HTTP"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(cancellation_token))
        .await
        .context("HTTP server failed")?;
    Ok(())
}

async fn shutdown_signal(cancellation_token: CancellationToken) {
    let _ = tokio::signal::ctrl_c().await;
    cancellation_token.cancel();
}

fn print_help() {
    eprintln!(
        "Usage:\n  abalone_mcp_server serve   # Streamable HTTP MCP server (default)\n  abalone_mcp_server stdio   # stdio MCP server for local subprocess clients"
    );
}

fn load_embedding_provider(config: &ServerConfig) -> Result<Arc<dyn EmbeddingProvider>> {
    match std::env::var("ABALONE_EMBEDDING_PROVIDER")
        .unwrap_or_else(|_| "bge-m3".to_string())
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "bge-m3" | "bge_m3" | "bge" => {
            if !BgeM3EmbeddingProvider::model_dir_is_present(&config.model_dir) {
                anyhow::bail!(
                    "BGE-M3 model assets are missing under {}. Expected tokenizer.json, config.json, and onnx/model_fp16.onnx. Set ABALONE_MODEL_DIR or use ABALONE_EMBEDDING_PROVIDER=deterministic for tests.",
                    config.model_dir.display()
                );
            }
            Ok(Arc::new(BgeM3EmbeddingProvider::from_dir(
                &config.model_dir,
            )?))
        }
        "deterministic" => Ok(Arc::new(DeterministicEmbeddingProvider::new(128))),
        other => anyhow::bail!(
            "unsupported ABALONE_EMBEDDING_PROVIDER `{other}`; expected bge-m3 or deterministic"
        ),
    }
}
