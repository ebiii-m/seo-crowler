use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Invalid response: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub embedding: Vec<f64>,
}

pub struct EmbeddingProvider {
    client: Client,
    model: String,
}

impl EmbeddingProvider {
    pub fn new(_host: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            model: model.to_string(),
        }
    }
    
    pub async fn generate(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        let url = format!("http://localhost:11434/api/embeddings");
        
        let body = serde_json::json!({
            "model": self.model,
            "prompt": text
        });
        
        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await?;
        
        let data: OllamaResponse = response.json().await?;
        
        Ok(data.embedding.into_iter().map(|x| x as f32).collect())
    }
}
