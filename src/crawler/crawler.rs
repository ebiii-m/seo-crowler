use crate::crawler::{cache::UrlCache, fetcher::Fetcher, robots::Robots};
use reqwest::Url;

pub struct Crawler {
    cache: UrlCache,
    fetcher: Fetcher,
    concurrency: usize,
    timeout: u64,
    max_depth: usize,
    max_urls: usize,
}

impl Crawler {
    pub fn new(config: &crate::config::CrawlerConfig) -> Self {
        Self {
            cache: UrlCache::new(),
            fetcher: Fetcher::new(config.timeout),
            concurrency: config.concurrency,
            timeout: config.timeout,
            max_depth: config.max_depth,
            max_urls: config.max_urls,
        }
    }
    
    pub async fn crawl(&self, base_url: &str, sitemap_path: Option<&str>) -> Vec<String> {
        let mut urls = Vec::new();
        
        if let Some(path) = sitemap_path {
            if let Some(sitemap_urls) = self.discover_sitemap(path).await {
                urls.extend(sitemap_urls);
            }
        }
        
        let _robots = self.fetch_robots(base_url).await;
        
        let _base = Url::parse(base_url).unwrap_or_else(|_| base_url.parse().unwrap());
        
        if !urls.contains(&base_url.to_string()) {
            urls.push(base_url.to_string());
        }
        
        if urls.len() > self.max_urls {
            urls.truncate(self.max_urls);
        }
        
        urls
    }
    
    async fn discover_sitemap(&self, path: &str) -> Option<Vec<String>> {
        let sitemap_url = format!("{}/sitemap.xml", path);
        
        match reqwest::get(&sitemap_url).await {
            Ok(response) => {
                if response.status().is_success() {
                    let content = response.text().await.ok()?;
                    if content.contains("<urlset") || content.contains("<url>") {
                        return Some(std::iter::empty().collect());
                    }
                }
            }
            Err(_) => {}
        }
        
        None
    }
    
    async fn fetch_robots(&self, base_url: &str) -> Robots {
        let robots_url = format!("{}/robots.txt", base_url);
        
        match reqwest::get(&robots_url).await {
            Ok(response) => {
                if response.status().is_success() {
                    let content = response.text().await.ok().unwrap_or_default();
                    let mut reader = quick_xml::Reader::from_str(&content);
                    reader.trim_text(true);
                    
                    let mut rules = Vec::new();
                    let mut host = String::new();
                    
                    loop {
                        match reader.read_event() {
                            Ok(quick_xml::events::Event::Start(ref e)) => {
                                match e.name().as_ref() {
                                    b"user-agent" => {
                                        let mut value = String::new();
                                        if let Ok(text) = reader.read_text(e.name()) {
                                            value = text.into_owned();
                                        }
                                        host = value;
                                    }
                                    b"disallow" => {
                                        let mut uri = String::new();
                                        if let Ok(text) = reader.read_text(e.name()) {
                                            uri = text.into_owned();
                                        }
                                        if host.is_empty() {
                                            host = "*".to_string();
                                        }
                                        rules.push(crate::crawler::RobotsRule {
                                            uri,
                                            allow: false,
                                        });
                                    }
                                    b"allow" => {
                                        let mut uri = String::new();
                                        if let Ok(text) = reader.read_text(e.name()) {
                                            uri = text.into_owned();
                                        }
                                        if host.is_empty() {
                                            host = "*".to_string();
                                        }
                                        rules.push(crate::crawler::RobotsRule {
                                            uri,
                                            allow: true,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            Ok(quick_xml::events::Event::Eof) => break,
                            Err(_) => break,
                            _ => {}
                        }
                    }
                    
                    Robots { rules }
                } else {
                    Robots { rules: Vec::new() }
                }
            }
            Err(_) => Robots { rules: Vec::new() },
        }
    }
}
