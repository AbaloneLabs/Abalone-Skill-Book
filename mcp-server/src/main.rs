use std::sync::Arc;

use abalone_mcp_server::config::ServerConfig;
use abalone_mcp_server::embedding::{
    BgeM3EmbeddingProvider, DeterministicEmbeddingProvider, EmbeddingProvider,
};
use abalone_mcp_server::storage::SkillStore;
use abalone_mcp_server::tools::AbaloneServer;
use anyhow::{Context, Result};
use rmcp::{transport::stdio, ServiceExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

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

    let store = SkillStore::open(&config.database_path).with_context(|| {
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

    let server = AbaloneServer::new(config.skills_root.clone(), store, embedder);
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
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
