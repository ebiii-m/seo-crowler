use clap::Parser;
use std::path::PathBuf;
use chrono::Utc;
use std::collections::{HashSet, VecDeque};
use url::Url;
use uuid::Uuid;

mod cli;
mod config;
mod crawler;
mod analyzer;
mod semantic;
mod grammar;
mod exporter;
mod pipeline;

use cli::{Cli, Action};
use config::Config;
use std::fs;
use serde::Deserialize;
use crawler::{Crawler, Fetcher, Parser as CrawlerParser, UrlCache};
use analyzer::{Analyzer, CtaChecker, AnchorChecker, SeoChecker, DuplicateChecker, Issue, Severity, PageInfo, Query};
use analyzer::anchor::Query as AnchorQuery;
use semantic::{Chunker, EmbeddingProvider, ChromaDb, Chunk, ChunkMetadata, ChromaDocument};
use grammar::{GrammarProvider, LanguageToolClient, OllamaGrammarClient};
use exporter::ExcelExporter;
use rust_xlsxwriter::Workbook;
use crate::crawler::PageData;

#[derive(Deserialize)]
struct QueriesFile {
    queries: Vec<analyzer::Query>,
}

#[derive(Debug, Clone)]
struct CrawledPage {
    page_data: PageData,
    status: u16,
    response_time_ms: u64,
    fetch_issues: Vec<Issue>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let action = cli.command.into();
    
    match action {
        Action::Scan {
            site,
            query,
            output,
            config,
        } => {
            let config_path = config.as_ref().map(|p| p.as_path());
            let mut cfg = Config::load(config_path)?;
            cfg.site.url = site.clone();
            
            let query_path = query.as_ref().unwrap_or(&cfg.queries.file);
            let query_content = fs::read_to_string(query_path)?;
            let queries_file: QueriesFile = serde_yaml::from_str(&query_content)?;
            
            // Parse anchor queries for AnchorChecker
            let anchor_queries: Vec<AnchorQuery> = queries_file.queries.iter()
                .filter_map(|q| {
                    if let analyzer::Query::Anchor { name, patterns, .. } = q {
                        Some(AnchorQuery {
                            name: name.clone(),
                            patterns: patterns.clone(),
                        })
                    } else {
                        None
                    }
                })
                .collect();
            
            // Parse semantic queries
            let _semantic_queries: Vec<analyzer::Query> = queries_file.queries.iter()
                .filter(|q| matches!(q, analyzer::Query::Semantic { .. }))
                .cloned()
                .collect();
            
            println!("SEO Semantic Auditor - Scanning {}...", site);
            
            // Initialize components
            let fetcher = Fetcher::new(cfg.crawler.timeout);
            let parser = CrawlerParser;
            
            // Initialize grammar checkers
            let mut languagetool = None;
            let mut ollama_grammar = None;
            
            if cfg.grammar.enabled {
                if let Some(lt_cfg) = &cfg.grammar.languagetool {
                    if lt_cfg.enabled {
                        languagetool = Some(LanguageToolClient::new(&lt_cfg.host, lt_cfg.timeout));
                    }
                }
                if let Some(og_cfg) = &cfg.grammar.ollama {
                    if og_cfg.enabled {
                        ollama_grammar = Some(OllamaGrammarClient::new(
                            &og_cfg.model,  // host param unused
                            &og_cfg.model,
                            og_cfg.temperature,
                            og_cfg.max_tokens,
                        ));
                    }
                }
            }
            
            // Initialize optional services
            let _semantic_enabled = cfg.semantic.enabled;
            
            // Discover all URLs (crawl with link following)
            println!("Discovering URLs...");
            let urls = discover_all_urls(&fetcher, &parser, &site, cfg.crawler.max_urls, cfg.crawler.max_depth).await;
            println!("Discovered {} URLs", urls.len());
            
            // Crawl each URL
            let mut crawled_pages = Vec::new();
            let mut all_issues = Vec::new();
            let mut page_infos = Vec::new();
            
            for (i, url) in urls.iter().enumerate() {
                println!("  [{}/{}] Fetching: {}", i + 1, urls.len(), url);
                
                match fetcher.fetch(url).await {
                    Ok(fetch_result) => {
                        match CrawlerParser::parse(&fetch_result.content.unwrap_or_default(), url) {
                            Ok(page_data) => {
                                let page_info = PageInfo {
                                    url: page_data.url.clone(),
                                    title: page_data.title.clone(),
                                    status: fetch_result.status,
                                    response_time_ms: fetch_result.response_time_ms,
                                };
                                page_infos.push(page_info);
                                
                                // Run analyzers
                                let mut page_issues = Vec::new();
                                
                                // Base SEO analysis
                                page_issues.extend(Analyzer::analyze(&page_data));
                                
                                // CTA check
                                page_issues.extend(CtaChecker::check(&page_data));
                                
                                // Anchor check
                                page_issues.extend(AnchorChecker::check(&page_data, &anchor_queries));
                                
                                // Technical SEO
                                page_issues.extend(SeoChecker::check(&page_data));
                                
                                // Grammar check - LanguageTool
                                if let Some(ref lt) = languagetool {
                                    if let Ok(grammar_result) = lt.check(&page_data.text).await {
                                        if let Some(problem) = grammar_result.problem {
                                            page_issues.push(Issue {
                                                url: page_data.url.clone(),
                                                page: page_data.url.clone(),
                                                issue_type: "Grammar".to_string(),
                                                message: problem,
                                                severity: Severity::Low,
                                            });
                                        }
                                    }
                                }
                                
                                // Grammar check - Ollama
                                if let Some(ref og) = ollama_grammar {
                                    if let Ok(grammar_result) = og.rewrite(&page_data.text).await {
                                        if let Some(problem) = grammar_result.problem {
                                            page_issues.push(Issue {
                                                url: page_data.url.clone(),
                                                page: page_data.url.clone(),
                                                issue_type: "Grammar".to_string(),
                                                message: problem,
                                                severity: Severity::Low,
                                            });
                                        }
                                    }
                                }
                                
                                all_issues.extend(page_issues.clone());
                                
                                crawled_pages.push(CrawledPage {
                                    page_data,
                                    status: fetch_result.status,
                                    response_time_ms: fetch_result.response_time_ms,
                                    fetch_issues: page_issues,
                                });
                                
                                println!("    ✓ {} ({}ms)", url, fetch_result.response_time_ms);
                            }
                            Err(e) => {
                                println!("    ✗ Parse error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("    ✗ Fetch error: {}", e);
                    }
                }
                
                // Small delay to be polite
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            
            // Duplicate check across all pages
            let all_page_data: Vec<PageData> = crawled_pages.iter().map(|p| p.page_data.clone()).collect();
            all_issues.extend(DuplicateChecker::check(&all_page_data));
            
            println!("
Crawl complete: {} pages, {} issues", crawled_pages.len(), all_issues.len());
            
            // Generate Excel report
            if let Some(output_path) = output {
                println!("Generating Excel report to {}...", output_path.display());
                generate_report(
                    &site, 
                    &crawled_pages, 
                    &all_issues, 
                    &page_infos,
                    &[],
                    output_path.to_str().unwrap()
                )?;
                println!("Report generated successfully!");
            }
            
            Ok(())
        }
    }
}

async fn discover_all_urls(
    fetcher: &Fetcher,
    parser: &CrawlerParser,
    base_url: &str,
    max_urls: usize,
    max_depth: usize,
) -> Vec<String> {
    let mut urls = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let base = Url::parse(base_url).unwrap();
    let base_domain = base.host_str().unwrap_or("").to_string();
    
    queue.push_back((base_url.to_string(), 0));
    visited.insert(base_url.to_string());
    
    while let Some((url, depth)) = queue.pop_front() {
        if urls.len() >= max_urls {
            break;
        }
        if depth > max_depth {
            continue;
        }
        
        urls.push(url.clone());
        
        if depth < max_depth {
            match fetcher.fetch(&url).await {
                Ok(fetch_result) => {
                    if let Some(content) = fetch_result.content {
                        if let Ok(page_data) = CrawlerParser::parse(&content, &url) {
                            for link in &page_data.links {
                                if let Ok(abs_url) = resolve_url(&url, &link.href) {
                                    let parsed = match Url::parse(&abs_url) {
                                        Ok(p) => p,
                                        Err(_) => continue,
                                    };
                                    if parsed.host_str().unwrap_or("") == base_domain 
                                        && !visited.contains(&abs_url)
                                        && (abs_url.starts_with("http://") || abs_url.starts_with("https://")) {
                                        visited.insert(abs_url.clone());
                                        queue.push_back((abs_url, depth + 1));
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }
    
    urls
}

fn resolve_url(base: &str, href: &str) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = Url::parse(base)?;
    let resolved = base_url.join(href)?;
    Ok(resolved.to_string())
}

fn generate_report(
    site: &str,
    crawled_pages: &[CrawledPage],
    all_issues: &[Issue],
    page_infos: &[PageInfo],
    _semantic_results: &[(String, String, f32)],
    output_path: &str,
) -> Result<(), rust_xlsxwriter::XlsxError> {
    let mut workbook = Workbook::new();
    
    // Summary sheet
    let summary = workbook.add_worksheet();
    summary.set_name("Summary")?;
    summary.write(0, 0, "SEO Semantic Auditor Report")?;
    summary.write(2, 0, "Site:")?;
    summary.write(2, 1, site)?;
    summary.write(3, 0, "Scan Date:")?;
    summary.write(3, 1, &Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())?;
    summary.write(5, 0, "Total Pages Crawled:")?;
    summary.write(5, 1, crawled_pages.len() as u32)?;
    summary.write(6, 0, "Total Issues Found:")?;
    summary.write(6, 1, all_issues.len() as u32)?;
    
    let high = all_issues.iter().filter(|i| i.severity == Severity::High).count() as u32;
    let medium = all_issues.iter().filter(|i| i.severity == Severity::Medium).count() as u32;
    let low = all_issues.iter().filter(|i| i.severity == Severity::Low).count() as u32;
    let critical = all_issues.iter().filter(|i| i.severity == Severity::Critical).count() as u32;
    
    summary.write(8, 0, "Critical:")?;
    summary.write(8, 1, critical)?;
    summary.write(9, 0, "High:")?;
    summary.write(9, 1, high)?;
    summary.write(10, 0, "Medium:")?;
    summary.write(10, 1, medium)?;
    summary.write(11, 0, "Low:")?;
    summary.write(11, 1, low)?;
    
    // Pages sheet
    let pages = workbook.add_worksheet();
    pages.set_name("Pages")?;
    pages.write_row(0, 0, ["URL", "Title", "Status", "Response Time (ms)"])?;
    for (i, info) in page_infos.iter().enumerate() {
        pages.write_row((i + 1) as u32, 0, [info.url.as_str(), info.title.as_str(), info.status.to_string().as_str(), info.response_time_ms.to_string().as_str()])?;
    }
    
    // Issues sheet
    let issues_sheet = workbook.add_worksheet();
    issues_sheet.set_name("Issues")?;
    issues_sheet.write_row(0, 0, ["URL", "Issue Type", "Severity", "Message"])?;
    for (i, issue) in all_issues.iter().enumerate() {
        issues_sheet.write_row((i + 1) as u32, 0, [issue.url.as_str(), issue.issue_type.as_str(), issue.severity.to_string().as_str(), issue.message.as_str()])?;
    }
    
    // Technical SEO sheet
    let technical = workbook.add_worksheet();
    technical.set_name("Technical SEO")?;
    technical.write_row(0, 0, ["URL", "Status", "Canonical", "Robots", "Meta Tags"])?;
    for (i, page) in crawled_pages.iter().enumerate() {
        let canonical = page.page_data.canonical.as_deref().unwrap_or("None");
        let robots = page.page_data.robots.as_deref().unwrap_or("None");
        let meta_count = page.page_data.meta_tags.len() as u32;
        technical.write_row((i + 1) as u32, 0, [page.page_data.url.as_str(), page.status.to_string().as_str(), canonical, robots, meta_count.to_string().as_str()])?;
    }
    
    // Links sheet
    let links = workbook.add_worksheet();
    links.set_name("Links")?;
    links.write_row(0, 0, ["Source URL", "Target URL", "Anchor Text", "Rel", "Title"])?;
    let mut row = 1u32;
    for page in crawled_pages {
        for link in &page.page_data.links {
            let rel = link.rel.as_deref().unwrap_or("None");
            let title = link.title.as_deref().unwrap_or("None");
            links.write_row(row, 0, [page.page_data.url.as_str(), link.href.as_str(), link.text.as_str(), rel, title])?;
            row += 1;
        }
    }
    
    // Semantic sheet
    let semantic = workbook.add_worksheet();
    semantic.set_name("Semantic")?;
    semantic.write_row(0, 0, ["Query Name", "Query Text", "Threshold", "Status"])?;
    semantic.write_row(1, 0, ["N/A", "Semantic search requires ChromaDB + Ollama", "0", "Not configured"])?;
    
    // Grammar sheet
    let grammar = workbook.add_worksheet();
    grammar.set_name("Grammar")?;
    grammar.write_row(0, 0, ["URL", "Problem", "Suggestion"])?;
    let grammar_issues: Vec<_> = all_issues.iter().filter(|i| i.issue_type == "Grammar").collect();
    if grammar_issues.is_empty() {
        grammar.write_row(1, 0, ["N/A", "No grammar issues found", ""])?;
    } else {
        for (i, issue) in grammar_issues.iter().enumerate() {
            grammar.write_row((i + 1) as u32, 0, [issue.url.as_str(), issue.message.as_str(), "Check manually"])?;
        }
    }
    
    workbook.save(output_path)?;
    Ok(())
}
