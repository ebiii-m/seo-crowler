use super::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    done: bool,
}

pub struct OllamaGrammarClient {
    client: Client,
    model: String,
    temperature: f32,
    max_tokens: usize,
}

impl OllamaGrammarClient {
    pub fn new(host: &str, model: &str, temperature: f32, max_tokens: usize) -> Self {
        let base_url = host.trim_end_matches('/');
        Self {
            client: Client::new(),
            model: model.to_string(),
            temperature,
            max_tokens,
        }
    }
    
    pub async fn rewrite(&self, text: &str) -> Result<GrammarResult, GrammarError> {
        let url = format!("{}/api/generate", "http://localhost:11434");
        
        let prompt = format!(
            "Fix any grammar, spelling, or style issues in this text. Return only the corrected text, no explanations:\n\n{}",
            text
        );
        
        let body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false,
            "temperature": self.temperature,
            "options": {
                "num_predict": self.max_tokens
            }
        });
        
        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(GrammarError::Invalid(response.status().to_string()));
        }
        
        let ollama_response: OllamaGenerateResponse = response.json().await?;
        
        let corrected = ollama_response.response.trim().to_string();
        
        // If the corrected text differs from original, we found issues
        if corrected != text {
            Ok(GrammarResult {
                url: "page".to_string(),
                problem: Some(format!("Grammar/style issues detected")),
                suggestion: Some(corrected),
            })
        } else {
            Ok(GrammarResult {
                url: "page".to_string(),
                problem: None,
                suggestion: None,
            })
        }
    }
}
