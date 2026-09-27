use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct RobotsRule {
    pub uri: String,
    pub allow: bool,
}

#[derive(Debug, Clone)]
pub struct Robots {
    pub rules: Vec<RobotsRule>,
}

impl Default for Robots {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
        }
    }
}

pub fn parse_robots(robots_path: &str) -> Robots {
    let content = fs::read(robots_path).unwrap_or_default();
    let mut reader = Reader::from_reader(&content[..]);
    reader.trim_text(true);
    
    let mut robots = Robots::default();
    let mut host = String::new();
    let mut in_sitemaps = false;
    
    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
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
                        robots.rules.push(RobotsRule {
                            uri: uri.clone(),
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
                        robots.rules.push(RobotsRule {
                            uri: uri.clone(),
                            allow: true,
                        });
                    }
                    b"sitemaps" => {
                        in_sitemaps = true;
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Failed to parse robots.txt: {}", e),
            _ => {}
        }
    }
    
    robots
}

pub fn is_allowed(path: &str, robots: &Robots) -> bool {
    let path = path.strip_prefix('/').unwrap_or(path);
    
    for rule in &robots.rules {
        if rule.uri == "*" || rule.uri == path || 
           path.starts_with(&rule.uri) {
            return rule.allow;
        }
    }
    
    true
}
