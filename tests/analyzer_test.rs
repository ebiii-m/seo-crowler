#[cfg(test)]
mod tests {
    use seo_semantic_auditor::analyzer::Analyzer;
    use seo_semantic_auditor::analyzer::Severity;
    use seo_semantic_auditor::crawler::PageData;

    #[test]
    fn test_analyze_page() {
        let page_data = PageData {
            url: "https://example.com/test".to_string(),
            title: "".to_string(),  // Empty title should trigger "Missing Title" issue
            description: "Test description".to_string(),
            headings: Vec::new(),
            text: "Test content".to_string(),
            links: Vec::new(),
            canonical: None,
            robots: None,
            meta_tags: Vec::new(),
        };
        
        let issues = Analyzer::analyze(&page_data);
        
        assert!(!issues.is_empty());
        
        let missing_title_issue = issues.iter()
            .find(|i| i.issue_type == "Missing Title")
            .unwrap();
        
        assert_eq!(missing_title_issue.severity, Severity::High);
    }
}
