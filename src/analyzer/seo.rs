use super::*;
use crate::crawler::PageData;

pub struct SeoChecker;

impl SeoChecker {
    pub fn check(page_data: &PageData) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        if page_data.canonical.is_none() {
            issues.push(Issue {
                url: page_data.url.clone(),
                page: page_data.url.clone(),
                issue_type: "Missing Canonical".to_string(),
                message: "Page lacks canonical URL".to_string(),
                severity: Severity::Medium,
            });
        }
        
        if let Some(robots) = &page_data.robots {
            if robots.to_lowercase().contains("noindex") {
                issues.push(Issue {
                    url: page_data.url.clone(),
                    page: page_data.url.clone(),
                    issue_type: "Noindex".to_string(),
                    message: "Page has noindex directive".to_string(),
                    severity: Severity::High,
                });
            }
        }
        
        issues
    }
}
