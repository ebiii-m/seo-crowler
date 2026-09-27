pub mod models;
pub mod cta;
pub mod anchor;
pub mod seo;
pub mod duplicate;

pub use models::*;
pub use cta::*;
pub use anchor::*;
pub use seo::*;
pub use duplicate::*;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum Query {
    Anchor {
        name: String,
        #[serde(rename = "type")]
        query_type: String,
        patterns: Vec<String>,
    },
    Semantic {
        name: String,
        #[serde(rename = "type")]
        query_type: String,
        query: Vec<String>,
        threshold: f32,
    },
}
