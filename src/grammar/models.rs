use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrammarError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Invalid response: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarResult {
    pub url: String,
    pub problem: Option<String>,
    pub suggestion: Option<String>,
}

pub struct GrammarProvider;

impl GrammarProvider {
    pub async fn check(&self, text: &str, url: &str) -> Result<GrammarResult, GrammarError> {
        // Placeholder implementation
        Ok(GrammarResult {
            url: url.to_string(),
            problem: None,
            suggestion: None,
        })
    }
}
