#[cfg(test)]
mod tests {
    use seo_semantic_auditor::crawler::Parser;

    #[test]
    fn test_parse_html() {
        let html = r#"<html>
            <head>
                <title>Test Page</title>
                <meta name="description" content="Test description">
            </head>
            <body>
                <h1>Main Heading</h1>
                <a href="https://example.com/link1">Click here</a>
                <a href="https://example.com/link2">Read more</a>
            </body>
        </html>"#;
        
        let page_data = Parser::parse(html, "https://example.com/test").unwrap();
        
        assert_eq!(page_data.title, "Test Page");
        assert_eq!(page_data.description, "Test description");
        assert_eq!(page_data.headings.len(), 1);
        assert_eq!(page_data.headings[0], "Main Heading");
        assert_eq!(page_data.links.len(), 2);
    }
}
