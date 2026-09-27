use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "seo-auditor")]
#[command(author = "SEO Team")]
#[command(version = "0.1.0")]
#[command(about = "SEO Semantic Auditor - Intelligent SEO Crawler and Analyzer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan a website for SEO issues
    Scan {
        /// Website URL to scan
        #[arg(short, long)]
        site: String,

        /// Query configuration file
        #[arg(short, long)]
        query: Option<PathBuf>,

        /// Output directory for reports
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
}

impl From<Commands> for Action {
    fn from(value: Commands) -> Self {
        match value {
            Commands::Scan {
                site,
                query,
                output,
                config,
            } => Action::Scan {
                site,
                query,
                output,
                config,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Scan {
        site: String,
        query: Option<PathBuf>,
        output: Option<PathBuf>,
        config: Option<PathBuf>,
    },
}
