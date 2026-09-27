use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Timeout")]
    Timeout,
    
    #[error("HTTP Error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Invalid status code: {0}")]
    InvalidStatus(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchResult {
    pub url: String,
    pub status: u16,
    pub content_type: String,
    pub response_time_ms: u64,
    pub content: Option<String>,
    pub last_modified: Option<String>,
    pub changed: Option<String>,
}

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new(timeout: u64) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(timeout))
                .build()
                .expect("Failed to build client"),
        }
    }
    
    pub async fn fetch(&self, url: &str) -> Result<FetchResult, FetchError> {
        let start = std::time::Instant::now();
        
        let response = self.client.get(url)
            .send()
            .await?;
        
        let status = response.status().as_u16();
        
        if !(200..300).contains(&status) {
            return Err(FetchError::InvalidStatus(status as usize));
        }
        
        let content_type = response.headers().get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();
        
        let last_modified = response.headers().get("last-modified")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        
        let changed = response.headers().get("etag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        
        let response_time_ms = start.elapsed().as_millis() as u64;
        
        let content = if content_type.starts_with("text/html") {
            Some(response.text().await?)
        } else {
            None
        };
        
        Ok(FetchResult {
            url: url.to_string(),
            status,
            content_type,
            response_time_ms,
            content,
            last_modified,
            changed,
        })
    }
}
