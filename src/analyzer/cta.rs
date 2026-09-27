use super::*;
use crate::crawler::PageData;

pub struct CtaChecker;

impl CtaChecker {
    pub fn check(page_data: &PageData) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        let cta_patterns = [
            "contact", "buy", "book", "schedule", "get started",
            "sign up", "register", "subscribe", "order now"
        ];
        
        let mut has_cta = false;
        
        for link in &page_data.links {
            for pattern in &cta_patterns {
                if link.text.to_lowercase().contains(pattern) {
                    has_cta = true;
                    break;
                }
            }
        }
        
        if !has_cta {
            issues.push(Issue {
                url: page_data.url.clone(),
                page: page_data.url.clone(),
                issue_type: "Missing CTA".to_string(),
                message: "Page lacks clear call-to-action buttons".to_string(),
                severity: Severity::Medium,
            });
        }
        
        issues
    }
}
