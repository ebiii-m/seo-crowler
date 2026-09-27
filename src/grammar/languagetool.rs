use super::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct LanguageToolResponse {
    matches: Vec<LanguageToolMatch>,
}

#[derive(Debug, Deserialize)]
struct LanguageToolMatch {
    message: String,
    short_message: Option<String>,
    replacements: Vec<LanguageToolReplacement>,
    context: LanguageToolContext,
    rule: LanguageToolRule,
}

#[derive(Debug, Deserialize)]
struct LanguageToolReplacement {
    value: String,
}

#[derive(Debug, Deserialize)]
struct LanguageToolContext {
    text: String,
    offset: usize,
    length: usize,
}

#[derive(Debug, Deserialize)]
struct LanguageToolRule {
    id: String,
    description: String,
    #[serde(rename = "type")]
    rule_type: LanguageToolRuleType,
}

#[derive(Debug, Deserialize)]
struct LanguageToolRuleType {
    type_name: String,
}

pub struct LanguageToolClient {
    client: Client,
    host: String,
    timeout: u64,
}

impl LanguageToolClient {
    pub fn new(host: &str, timeout: u64) -> Self {
        Self {
            client: Client::new(),
            host: host.to_string(),
            timeout,
        }
    }
    
    pub async fn check(&self, text: &str) -> Result<GrammarResult, GrammarError> {
        let url = format!("{}/v2/check", self.host.trim_end_matches('/'));
        
        let body = serde_json::json!({
            "text": text,
            "language": "en-US",
            "enabledOnly": false
        });
        
        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(GrammarError::Invalid(response.status().to_string()));
        }
        
        let lt_response: LanguageToolResponse = response.json().await?;
        
        // Extract first grammar issue found
        if let Some(first_match) = lt_response.matches.first() {
            let suggestion = first_match.replacements.first()
                .map(|r| r.value.clone())
                .unwrap_or_else(|| "No suggestion available".to_string());
            
            Ok(GrammarResult {
                url: "page".to_string(),
                problem: Some(first_match.message.clone()),
                suggestion: Some(suggestion),
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
