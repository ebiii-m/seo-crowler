use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChromaError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Invalid response: {0}")]
    Invalid(String),
    
    #[error("Collection not found: {0}")]
    CollectionNotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaDocument {
    pub id: String,
    pub document: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub ids: Vec<String>,
    pub documents: Vec<String>,
    pub metadatas: Vec<serde_json::Value>,
    pub distances: Vec<f32>,
}

pub struct ChromaDb {
    client: Client,
    collection_name: String,
}

impl ChromaDb {
    pub fn new(_host: &str, collection: &str) -> Self {
        Self {
            client: Client::new(),
            collection_name: collection.to_string(),
        }
    }
    
    pub async fn create_collection(&self) -> Result<(), ChromaError> {
        let url = format!("http://localhost:8000/api/v1/collections");
        
        let response = self.client.post(&url)
            .json(&json!({}))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(ChromaError::CollectionNotFound(self.collection_name.clone()));
        }
        
        Ok(())
    }
    
    pub async fn add_documents(&self, documents: &[ChromaDocument]) -> Result<(), ChromaError> {
        let base_url = format!("/api/v1/collection/{}", self.collection_name);
        let url = format!("http://localhost:8000{}", base_url);
        
        let body = json!({
            "documents": documents.iter().map(|d| d.document.as_str()).collect::<Vec<&str>>(),
            "metadatas": documents.iter().map(|d| serde_json::to_value(&d.metadata).unwrap()).collect::<Vec<serde_json::Value>>(),
            "ids": documents.iter().map(|d| d.id.as_str()).collect::<Vec<&str>>()
        });
        
        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(ChromaError::Invalid(response.status().to_string()));
        }
        
        Ok(())
    }
    
    pub async fn search(&self, query_text: &str, limit: usize) -> Result<SearchResult, ChromaError> {
        let base_url = format!("/api/v1/query");
        let url = format!("http://localhost:8000{}", base_url);
        
        let body = json!({
            "query_texts": [query_text],
            "n_results": limit,
            "where": {}
        });
        
        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await?;
        
        let data: SearchResult = response.json().await?;
        
        Ok(data)
    }
}
