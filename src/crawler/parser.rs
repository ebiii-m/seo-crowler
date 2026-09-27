use scraper::{Html, Selector, ElementRef};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("HTML parsing error: {0}")]
    Parse(String),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData {
    pub url: String,
    pub title: String,
    pub description: String,
    pub headings: Vec<String>,
    pub text: String,
    pub links: Vec<Link>,
    pub canonical: Option<String>,
    pub robots: Option<String>,
    pub meta_tags: Vec<MetaTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub text: String,
    pub rel: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTag {
    pub name: String,
    pub content: String,
}

pub struct Parser;

impl Parser {
    pub fn parse(html: &str, url: &str) -> Result<PageData, ParseError> {
        let document = Html::parse_document(html);
        
        let title_selector = Selector::parse("title").unwrap();
        let title = document.select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or("".to_string());
        
        let description_selector = Selector::parse("meta[name=description]").unwrap();
        let description = document.select(&description_selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .unwrap_or("")
            .to_string();
        
        let headings_selector = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();
        let headings: Vec<String> = document.select(&headings_selector)
            .map(|el| el.text().collect::<String>())
            .collect();
        
        let body_selector = Selector::parse("body").unwrap();
        let body = document.select(&body_selector)
            .next()
            .map(|el| el.text().collect::<String>());
        let text = body.unwrap_or_else(|| document.root_element().text().collect::<String>());
        
        let links_selector = Selector::parse("a").unwrap();
        let mut links = Vec::new();
        
        for el in document.select(&links_selector) {
            if let Some(href) = el.value().attr("href") {
                let text = el.text().collect::<String>();
                links.push(Link {
                    href: href.to_string(),
                    text,
                    rel: el.value().attr("rel").map(|s| s.to_string()),
                    title: el.value().attr("title").map(|s| s.to_string()),
                });
            }
        }
        
        let canonical_selector = Selector::parse("link[rel=canonical]").unwrap();
        let canonical = document.select(&canonical_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|s| s.to_string());
        
        let robots_selector = Selector::parse("meta[name=robots]").unwrap();
        let robots = document.select(&robots_selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(|s| s.to_string());
        
        let meta_tags: Vec<MetaTag> = document.select(&description_selector.clone())
            .map(|el| {
                MetaTag {
                    name: el.value().attr("name").unwrap_or("").to_string(),
                    content: el.value().attr("content").unwrap_or("").to_string(),
                }
            })
            .collect();
        
        Ok(PageData {
            url: url.to_string(),
            title,
            description,
            headings,
            text,
            links,
            canonical,
            robots,
            meta_tags,
        })
    }
}
