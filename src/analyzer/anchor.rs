use super::*;
use crate::crawler::PageData;
use std::collections::HashMap;

pub struct AnchorChecker;

impl AnchorChecker {
    pub fn check(page_data: &PageData, queries: &[Query]) -> Vec<Issue> {
        let mut issues = Vec::new();
        let mut anchor_map: HashMap<String, usize> = HashMap::new();
        
        for link in &page_data.links {
            *anchor_map.entry(link.text.clone()).or_insert(0) += 1;
        }
        
        for query in queries {
            for pattern in &query.patterns {
                for (anchor, count) in &anchor_map {
                    if anchor.to_lowercase().contains(&pattern.to_lowercase()) {
                        issues.push(Issue {
                            url: page_data.url.clone(),
                            page: page_data.url.clone(),
                            issue_type: format!("Weak Anchor: {}", pattern),
                            message: format!("Found weak anchor: '{}' ({} times)", anchor, count),
                            severity: Severity::Low,
                        });
                    }
                }
            }
        }
        
        issues
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    pub name: String,
    pub patterns: Vec<String>,
}
