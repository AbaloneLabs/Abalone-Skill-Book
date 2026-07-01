use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::{Context, Result};
use half::f16;
use ort::session::{builder::GraphOptimizationLevel, Session, SessionInputValue};
use ort::value::Tensor;
use tokenizers::tokenizer::TruncationDirection;
use tokenizers::Tokenizer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddingModelSpec {
    pub name: String,
    pub dimension: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddingInput {
    pub path: String,
    pub name: String,
    pub description: String,
}

impl EmbeddingInput {
    pub fn ranking_text(&self) -> String {
        format!("{}\n{}\n{}", self.path, self.name, self.description)
    }
}

pub trait EmbeddingProvider: Send + Sync {
    fn spec(&self) -> &EmbeddingModelSpec;
    fn embed(&self, input: &str) -> Result<Vec<f32>>;
}

#[derive(Debug)]
pub struct BgeM3EmbeddingProvider {
    spec: EmbeddingModelSpec,
    tokenizer: Tokenizer,
    session: Mutex<Session>,
    max_tokens: usize,
}

impl BgeM3EmbeddingProvider {
    pub fn from_dir(model_dir: impl AsRef<Path>) -> Result<Self> {
        let model_dir = model_dir.as_ref();
        let tokenizer_path = model_dir.join("tokenizer.json");
        let model_path = model_dir.join("onnx").join("model_fp16.onnx");
        let config_path = model_dir.join("config.json");

        if !tokenizer_path.is_file() {
            anyhow::bail!("missing tokenizer file: {}", tokenizer_path.display());
        }
        if !model_path.is_file() {
            anyhow::bail!("missing BGE-M3 ONNX model: {}", model_path.display());
        }

        let dimension = read_hidden_size(&config_path).with_context(|| {
            format!(
                "failed to read embedding dimension from {}",
                config_path.display()
            )
        })?;
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|err| anyhow::anyhow!("failed to load tokenizer: {err}"))?;
        let mut builder = Session::builder()
            .context("failed to create ONNX session builder")?
            .with_optimization_level(GraphOptimizationLevel::Level1)
            .map_err(|err| anyhow::anyhow!("failed to set ONNX graph optimization level: {err}"))?
            .with_memory_pattern(false)
            .map_err(|err| anyhow::anyhow!("failed to configure ONNX memory pattern: {err}"))?;
        let session = builder
            .commit_from_file(&model_path)
            .with_context(|| format!("failed to load ONNX model {}", model_path.display()))?;

        Ok(Self {
            spec: EmbeddingModelSpec {
                name: format!("bge-m3-onnx-fp16:{}", stable_model_path(model_dir)),
                dimension,
            },
            tokenizer,
            session: Mutex::new(session),
            max_tokens: 512,
        })
    }

    pub fn model_dir_is_present(model_dir: impl AsRef<Path>) -> bool {
        let model_dir = model_dir.as_ref();
        model_dir.join("tokenizer.json").is_file()
            && model_dir.join("onnx").join("model_fp16.onnx").is_file()
            && model_dir.join("config.json").is_file()
    }
}

impl EmbeddingProvider for BgeM3EmbeddingProvider {
    fn spec(&self) -> &EmbeddingModelSpec {
        &self.spec
    }

    fn embed(&self, input: &str) -> Result<Vec<f32>> {
        let mut encoding = self
            .tokenizer
            .encode(input, true)
            .map_err(|err| anyhow::anyhow!("failed to tokenize embedding input: {err}"))?;
        encoding.truncate(self.max_tokens, 0, TruncationDirection::Right);

        let ids = encoding
            .get_ids()
            .iter()
            .map(|id| *id as i64)
            .collect::<Vec<_>>();
        let attention_mask = encoding
            .get_attention_mask()
            .iter()
            .map(|value| *value as i64)
            .collect::<Vec<_>>();

        if ids.is_empty() {
            anyhow::bail!("cannot embed empty token sequence");
        }
        if ids.len() != attention_mask.len() {
            anyhow::bail!(
                "tokenizer produced mismatched ids and attention mask lengths: {} vs {}",
                ids.len(),
                attention_mask.len()
            );
        }

        let sequence_len = ids.len();
        let mut session = self
            .session
            .lock()
            .map_err(|_| anyhow::anyhow!("embedding session lock is poisoned"))?;
        let input_names = session
            .inputs()
            .iter()
            .map(|input| input.name().to_string())
            .collect::<Vec<_>>();

        let mut inputs = Vec::<(String, SessionInputValue<'_>)>::new();
        for name in input_names {
            match name.as_str() {
                "input_ids" => inputs.push((
                    name,
                    Tensor::<i64>::from_array(([1usize, sequence_len], ids.clone()))?.into(),
                )),
                "attention_mask" => inputs.push((
                    name,
                    Tensor::<i64>::from_array(([1usize, sequence_len], attention_mask.clone()))?
                        .into(),
                )),
                "token_type_ids" => inputs.push((
                    name,
                    Tensor::<i64>::from_array(([1usize, sequence_len], vec![0_i64; sequence_len]))?
                        .into(),
                )),
                other => {
                    anyhow::bail!("unsupported BGE-M3 ONNX input `{other}`");
                }
            }
        }

        let outputs = session.run(inputs)?;
        let output = outputs
            .get("last_hidden_state")
            .unwrap_or_else(|| &outputs[0]);

        if let Ok((shape, data)) = output.try_extract_tensor::<f32>() {
            return mean_pool_f32(shape, data, &attention_mask, self.spec.dimension);
        }
        if let Ok((shape, data)) = output.try_extract_tensor::<f16>() {
            let data = data.iter().map(|value| value.to_f32()).collect::<Vec<_>>();
            return mean_pool_f32(shape, &data, &attention_mask, self.spec.dimension);
        }

        anyhow::bail!("BGE-M3 output tensor is neither f32 nor f16");
    }
}

#[derive(Debug, Clone)]
pub struct DeterministicEmbeddingProvider {
    spec: EmbeddingModelSpec,
}

impl DeterministicEmbeddingProvider {
    pub fn new(dimension: usize) -> Self {
        Self {
            spec: EmbeddingModelSpec {
                name: format!("deterministic-test-{dimension}"),
                dimension,
            },
        }
    }
}

impl EmbeddingProvider for DeterministicEmbeddingProvider {
    fn spec(&self) -> &EmbeddingModelSpec {
        &self.spec
    }

    fn embed(&self, input: &str) -> Result<Vec<f32>> {
        let mut vector = vec![0.0; self.spec.dimension];
        if self.spec.dimension == 0 {
            return Ok(vector);
        }

        for token in input.split(|ch: char| !ch.is_ascii_alphanumeric()) {
            if token.is_empty() {
                continue;
            }
            let mut hash = 0usize;
            for byte in token.bytes() {
                hash = hash.wrapping_mul(31).wrapping_add(byte as usize);
            }
            vector[hash % self.spec.dimension] += 1.0;
        }

        let norm = vector.iter().map(|value| value * value).sum::<f32>().sqrt();
        if norm > 0.0 {
            for value in &mut vector {
                *value /= norm;
            }
        }

        Ok(vector)
    }
}

pub fn cosine_similarity(left: &[f32], right: &[f32]) -> f32 {
    if left.is_empty() || left.len() != right.len() {
        return 0.0;
    }

    let mut dot = 0.0;
    let mut left_norm = 0.0;
    let mut right_norm = 0.0;

    for (l, r) in left.iter().zip(right) {
        dot += l * r;
        left_norm += l * l;
        right_norm += r * r;
    }

    if left_norm == 0.0 || right_norm == 0.0 {
        return 0.0;
    }

    dot / (left_norm.sqrt() * right_norm.sqrt())
}

pub fn vector_to_blob(vector: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(std::mem::size_of_val(vector));
    for value in vector {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}

pub fn blob_to_vector(bytes: &[u8]) -> Result<Vec<f32>> {
    if !bytes.len().is_multiple_of(std::mem::size_of::<f32>()) {
        anyhow::bail!("embedding blob byte length is not divisible by 4");
    }

    Ok(bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect())
}

fn read_hidden_size(config_path: &Path) -> Result<usize> {
    let source = fs::read_to_string(config_path)?;
    let json = serde_json::from_str::<serde_json::Value>(&source)?;
    let hidden_size = json
        .get("hidden_size")
        .and_then(serde_json::Value::as_u64)
        .context("config.json is missing numeric hidden_size")?;
    Ok(hidden_size as usize)
}

fn stable_model_path(model_dir: &Path) -> String {
    model_dir
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(model_dir))
        .to_string_lossy()
        .to_string()
}

fn mean_pool_f32(
    shape: &[i64],
    data: &[f32],
    attention_mask: &[i64],
    dimension: usize,
) -> Result<Vec<f32>> {
    if shape.len() != 3 {
        anyhow::bail!("expected BGE-M3 output rank 3, got shape {shape:?}");
    }
    let batch = shape[0] as usize;
    let sequence_len = shape[1] as usize;
    let hidden = shape[2] as usize;
    if batch != 1 {
        anyhow::bail!("expected BGE-M3 batch size 1, got {batch}");
    }
    if hidden != dimension {
        anyhow::bail!("expected BGE-M3 hidden size {dimension}, got {hidden}");
    }
    if sequence_len != attention_mask.len() {
        anyhow::bail!(
            "output sequence length {} does not match attention mask length {}",
            sequence_len,
            attention_mask.len()
        );
    }
    if data.len() != sequence_len * hidden {
        anyhow::bail!(
            "output data length {} does not match shape [{batch}, {sequence_len}, {hidden}]",
            data.len()
        );
    }

    let mut pooled = vec![0.0_f32; hidden];
    let mut token_count = 0.0_f32;
    for (token_idx, &mask_value) in attention_mask.iter().enumerate().take(sequence_len) {
        if mask_value == 0 {
            continue;
        }
        token_count += 1.0;
        let offset = token_idx * hidden;
        for dim in 0..hidden {
            pooled[dim] += data[offset + dim];
        }
    }

    if token_count == 0.0 {
        anyhow::bail!("attention mask has no active tokens");
    }
    for value in &mut pooled {
        *value /= token_count;
    }
    normalize(&mut pooled);
    Ok(pooled)
}

fn normalize(vector: &mut [f32]) {
    let norm = vector.iter().map(|value| value * value).sum::<f32>().sqrt();
    if norm > 0.0 {
        for value in vector {
            *value /= norm;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        blob_to_vector, cosine_similarity, mean_pool_f32, read_hidden_size, vector_to_blob,
        BgeM3EmbeddingProvider, DeterministicEmbeddingProvider, EmbeddingProvider,
    };
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn cosine_similarity_handles_basic_vectors() {
        let score = cosine_similarity(&[1.0, 0.0], &[1.0, 0.0]);
        assert!((score - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn cosine_similarity_rejects_mismatched_dimensions() {
        assert_eq!(cosine_similarity(&[1.0], &[1.0, 0.0]), 0.0);
    }

    #[test]
    fn vector_blob_round_trip_preserves_values() {
        let vector = vec![1.0, -2.5, 3.25];
        let blob = vector_to_blob(&vector);
        assert_eq!(blob_to_vector(&blob).unwrap(), vector);
    }

    #[test]
    fn deterministic_provider_is_stable() {
        let provider = DeterministicEmbeddingProvider::new(8);
        assert_eq!(
            provider.embed("api response").unwrap(),
            provider.embed("api response").unwrap()
        );
    }

    #[test]
    fn mean_pool_normalizes_active_tokens() {
        let vector = mean_pool_f32(&[1, 2, 2], &[1.0, 0.0, 0.0, 1.0], &[1, 1], 2).unwrap();

        assert!((vector[0] - 0.70710677).abs() < 0.0001);
        assert!((vector[1] - 0.70710677).abs() < 0.0001);
    }

    #[test]
    fn reads_hidden_size_from_config() {
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.json");
        fs::write(&config_path, r#"{"hidden_size": 1024}"#).unwrap();
        assert_eq!(read_hidden_size(&config_path).unwrap(), 1024);
    }

    #[test]
    fn detects_missing_bge_assets() {
        let temp = TempDir::new().unwrap();
        assert!(!BgeM3EmbeddingProvider::model_dir_is_present(temp.path()));
    }
}
