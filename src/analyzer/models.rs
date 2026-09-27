use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::crawler::PageData;

#[derive(Error, Debug)]
pub enum AnalysisError {
    #[error("Invalid data: {0}")]
    Invalid(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub url: String,
    pub page: String,
    pub issue_type: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Low => write!(f, "Low"),
            Severity::Medium => write!(f, "Medium"),
            Severity::High => write!(f, "High"),
            Severity::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub url: String,
    pub title: String,
    pub status: u16,
    pub response_time_ms: u64,
}

pub struct Analyzer;

impl Analyzer {
    pub fn analyze(page_data: &PageData) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        if page_data.title.is_empty() {
            issues.push(Issue {
                url: page_data.url.clone(),
                page: page_data.url.clone(),
                issue_type: "Missing Title".to_string(),
                message: "Page has no title tag".to_string(),
                severity: Severity::High,
            });
        }
        
        if page_data.headings.is_empty() {
            issues.push(Issue {
                url: page_data.url.clone(),
                page: page_data.url.clone(),
                issue_type: "Missing Headings".to_string(),
                message: "Page has no heading tags".to_string(),
                severity: Severity::Medium,
            });
        }
        
        if page_data.description.is_empty() {
            issues.push(Issue {
                url: page_data.url.clone(),
                page: page_data.url.clone(),
                issue_type: "Missing Description".to_string(),
                message: "Page has no meta description".to_string(),
                severity: Severity::Medium,
            });
        }
        
        for link in &page_data.links {
            if Self::is_weak_anchor(&link.text) {
                issues.push(Issue {
                    url: page_data.url.clone(),
                    page: page_data.url.clone(),
                    issue_type: "Weak Anchor".to_string(),
                    message: format!("Weak anchor text: '{}'", link.text),
                    severity: Severity::Low,
                });
            }
        }
        
        issues
    }
    
    fn is_weak_anchor(text: &str) -> bool {
        let weak_patterns = [
            "click here", "here", "read more", "learn more",
            "find out more", "this link", "the link", "button", "link"
        ];
        
        for pattern in &weak_patterns {
            if text.to_lowercase().contains(pattern) {
                return true;
            }
        }
        
        false
    }
}
