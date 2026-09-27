use super::*;
use crate::crawler::PageData;
use std::collections::HashMap;

pub struct DuplicateChecker;

impl DuplicateChecker {
    pub fn check(pages: &[PageData]) -> Vec<Issue> {
        let mut issues = Vec::new();
        let mut title_map: HashMap<String, Vec<&PageData>> = HashMap::new();
        let mut desc_map: HashMap<String, Vec<&PageData>> = HashMap::new();
        
        for page in pages {
            title_map.entry(page.title.clone())
                .or_insert_with(Vec::new)
                .push(page);
            
            desc_map.entry(page.description.clone())
                .or_insert_with(Vec::new)
                .push(page);
        }
        
        for (title, pages_with_title) in &title_map {
            if pages_with_title.len() > 1 {
                for page in pages_with_title.iter().skip(1) {
                    issues.push(Issue {
                        url: page.url.clone(),
                        page: page.url.clone(),
                        issue_type: "Duplicate Title".to_string(),
                        message: format!("Duplicate title: '{}'", title),
                        severity: Severity::High,
                    });
                }
            }
        }
        
        for (desc, pages_with_desc) in &desc_map {
            if pages_with_desc.len() > 1 {
                for page in pages_with_desc.iter().skip(1) {
                    issues.push(Issue {
                        url: page.url.clone(),
                        page: page.url.clone(),
                        issue_type: "Duplicate Description".to_string(),
                        message: format!("Duplicate description found"),
                        severity: Severity::Medium,
                    });
                }
            }
        }
        
        issues
    }
}
