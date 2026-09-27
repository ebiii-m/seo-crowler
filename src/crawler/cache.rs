use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UrlEntry {
    pub url: String,
    pub last_modified: Option<String>,
    pub changed: Option<String>,
    pub priority: Option<f32>,
    pub images: Vec<String>,
}

impl UrlEntry {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            last_modified: None,
            changed: None,
            priority: None,
            images: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct UrlCache {
    pub entries: Arc<RwLock<HashSet<String>>>,
    pub data: Arc<RwLock<Vec<UrlEntry>>>,
    pub stats: Arc<RwLock<CrawlerStats>>,
}

#[derive(Debug, Default, Clone)]
pub struct CrawlerStats {
    pub total: usize,
    pub fetched: usize,
    pub errors: usize,
    pub timeouts: usize,
}

impl UrlCache {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashSet::new())),
            data: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(CrawlerStats::default())),
        }
    }

    pub async fn is_cached(&self, url: &str) -> bool {
        let entries = self.entries.read().await;
        entries.contains(url)
    }

    pub async fn add(&self, entry: UrlEntry) {
        let mut entries = self.entries.write().await;
        let mut data = self.data.write().await;
        
        entries.insert(entry.url.clone());
        data.push(entry);
    }

    pub async fn get(&self, url: &str) -> Option<UrlEntry> {
        let data = self.data.read().await;
        data.iter()
            .find(|e| e.url == url)
            .cloned()
    }

    pub async fn remove(&self, url: &str) {
        let mut entries = self.entries.write().await;
        let mut data = self.data.write().await;
        
        entries.remove(url);
        data.retain(|e| e.url != url);
    }

    pub async fn increment_stats(&self, error: bool, timeout: bool) {
        let mut stats = self.stats.write().await;
        stats.total += 1;
        if error {
            stats.errors += 1;
        }
        if timeout {
            stats.timeouts += 1;
        }
    }
}
