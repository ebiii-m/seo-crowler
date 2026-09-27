use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub site: SiteConfig,
    pub crawler: CrawlerConfig,
    pub semantic: SemanticConfig,
    pub embedding: EmbeddingConfig,
    pub grammar: GrammarConfig,
    pub queries: QueriesConfig,
    pub report: ReportConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlerConfig {
    pub concurrency: usize,
    pub timeout: u64,
    pub max_depth: usize,
    pub max_urls: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticConfig {
    pub enabled: bool,
    pub chroma: ChromaConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaConfig {
    pub http: String,
    pub collection: String,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    pub provider: EmbeddingProviderType,
    pub model: String,
    pub ollama: Option<OllamaConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingProviderType {
    Ollama,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarConfig {
    pub enabled: bool,
    pub languagetool: Option<LanguageToolConfig>,
    pub ollama: Option<OllamaGrammarConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageToolConfig {
    pub enabled: bool,
    pub host: String,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaGrammarConfig {
    pub enabled: bool,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueriesConfig {
    pub file: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    pub output: PathBuf,
    pub sheets: ReportSheets,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportSheets {
    pub summary: bool,
    pub pages: bool,
    pub issues: bool,
    pub technical: bool,
    pub links: bool,
    pub semantic: bool,
    pub grammar: bool,
}

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("config.yaml"));

        let content = std::fs::read_to_string(&path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}
