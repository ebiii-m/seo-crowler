#[cfg(test)]
mod tests {
    use seo_semantic_auditor::crawler::sitemap::parse_sitemap;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_sitemap() {
        let dir = tempdir().unwrap();
        let sitemap_path = dir.path().join("sitemap.xml");
        
        let sitemap_content = r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
            <url>
                <loc>https://example.com/page1</loc>
            </url>
            <url>
                <loc>https://example.com/page2</loc>
            </url>
        </urlset>"#;
        
        fs::write(&sitemap_path, sitemap_content).unwrap();
        
        let urls = parse_sitemap(&sitemap_path.to_str().unwrap());
        
        assert_eq!(urls.len(), 2);
        assert_eq!(urls[0], "https://example.com/page1");
        assert_eq!(urls[1], "https://example.com/page2");
    }
}
