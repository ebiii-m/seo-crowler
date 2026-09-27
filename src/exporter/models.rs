use serde::{Deserialize, Serialize};
use crate::analyzer::{Issue, PageInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub pages: Vec<PageInfo>,
    pub issues: Vec<Issue>,
    pub stats: CrawlerStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlerStats {
    pub total: usize,
    pub fetched: usize,
    pub errors: usize,
    pub timeouts: usize,
}

pub struct ExcelExporter;

impl ExcelExporter {
    pub fn export(&self, data: &ReportData, output_path: &str) -> Result<(), rust_xlsxwriter::XlsxError> {
        let mut workbook = rust_xlsxwriter::Workbook::new();
        
        let summary = workbook.add_worksheet();
        summary.set_name("Summary")?;
        summary.write(0, 0, "SEO Semantic Auditor Report")?;
        summary.write(2, 0, "Total Pages:")?;
        summary.write(3, 0, &data.pages.len().to_string())?;
        
        let pages = workbook.add_worksheet();
        pages.set_name("Pages")?;
        pages.write_row(0, 0, ["URL", "Title", "Status", "Response Time (ms)"])?;
        
        let issues = workbook.add_worksheet();
        issues.set_name("Issues")?;
        issues.write_row(0, 0, ["URL", "Issue Type", "Severity", "Message"])?;
        
        let technical = workbook.add_worksheet();
        technical.set_name("Technical SEO")?;
        technical.write_row(0, 0, ["URL", "Status", "Canonical", "Robots"])?;
        
        let links = workbook.add_worksheet();
        links.set_name("Links")?;
        links.write_row(0, 0, ["URL", "Anchor Text", "Rel", "Title"])?;
        
        let semantic = workbook.add_worksheet();
        semantic.set_name("Semantic")?;
        semantic.write_row(0, 0, ["Query", "Match", "Score", "URL"])?;
        
        let grammar = workbook.add_worksheet();
        grammar.set_name("Grammar")?;
        grammar.write_row(0, 0, ["URL", "Problem", "Suggestion"])?;
        
        workbook.save(output_path)?;
        
        Ok(())
    }
}
