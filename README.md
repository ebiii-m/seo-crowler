# SEO Semantic Auditor

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

A high-performance **SEO intelligence crawler** written in Rust. Crawls websites, analyzes SEO issues, performs semantic content search with vector embeddings, checks grammar/spelling, and exports comprehensive Excel reports.

## Features

| Feature | Description |
|---------|-------------|
| **Concurrent Crawling** | Async HTTP with configurable concurrency, depth, and URL limits |
| **SEO Analysis** | Title, headings, meta tags, canonical, CTA detection, anchor quality, duplicates |
| **Semantic Search** | Vector embeddings via Ollama + ChromaDB for content discovery |
| **Grammar Checking** | LanguageTool server + Ollama LLM for spelling/grammar/style |
| **YAML-Driven Queries** | All checks defined in YAML - no code changes needed |
| **Excel Reports** | 7-sheet reports: Summary, Pages, Issues, Technical SEO, Links, Semantic, Grammar |

## Quick Start

### Minimal (No External Services)
```bash
git clone https://github.com/yourusername/seo-semantic-auditor
cd seo-semantic-auditor
./quick_start.sh https://your-site.com
```

### Full Pipeline (All Features)
```bash
git clone https://github.com/yourusername/seo-semantic-auditor
cd seo-semantic-auditor
./run_pipeline.sh https://your-site.com
```

## Installation

### Prerequisites
- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Docker & Docker Compose** (for full pipeline)

### Build from Source
```bash
cargo build --release
```

Binary: `./target/release/seo-semantic-auditor`

## Usage

```bash
# Basic scan
./target/release/seo-semantic-auditor scan   --site https://example.com   --output reports/report.xlsx

# With custom query file
./target/release/seo-semantic-auditor scan   --site https://example.com   --query queries/comprehensive.yaml   --output reports/report.xlsx

# With custom config
./target/release/seo-semantic-auditor scan   --site https://example.com   --config config.prod.yaml   --output reports/report.xlsx
```

### CLI Options
| Flag | Description | Default |
|------|-------------|---------|
| `--site`, `-s` | Target website URL | Required |
| `--query`, `-q` | Query YAML file | `queries/comprehensive.yaml` |
| `--output`, `-o` | Output Excel file | `reports/seo-report.xlsx` |
| `--config`, `-c` | Config YAML file | `config.yaml` |

## Configuration

Edit `config.yaml`:

```yaml
site:
  url: https://example.com

crawler:
  concurrency: 10      # Parallel requests
  timeout: 20          # Request timeout (seconds)
  max_depth: 3         # Link traversal depth
  max_urls: 1000       # Maximum URLs to crawl

semantic:
  enabled: true
  chroma:
    http: http://localhost:8000
    collection: seo-content
    chunk_size: 300
    chunk_overlap: 50

embedding:
  provider: ollama
  model: nomic-embed-text
  ollama:
    host: http://localhost:11434

grammar:
  enabled: true
  languagetool:
    enabled: true
    host: http://localhost:2372
    timeout: 30
  ollama:
    enabled: true
    model: llama2
    temperature: 0.7
    max_tokens: 500

queries:
  file: ./queries/comprehensive.yaml

report:
  output: ./reports/seo-report.xlsx
  sheets:
    summary: true
    pages: true
    issues: true
    technical: true
    links: true
    semantic: true
    grammar: true
```

## Query System

All SEO checks are defined in YAML - **no code changes needed**.

### Query Types

**Anchor Checks** - Detect weak link text:
```yaml
- name: weak_anchor
  type: anchor
  patterns:
    - click here
    - read more
    - learn more
```

**Semantic Searches** - Find content by meaning:
```yaml
- name: services_search
  type: semantic
  query:
    - our services
    - what we offer
  threshold: 0.8
```

See [QUERIES.md](QUERIES.md) for full documentation and [queries/comprehensive.yaml](queries/comprehensive.yaml) for 22 pre-built checks.

## Services (Optional)

For semantic search and grammar checking:

```bash
# Start all services
docker-compose up -d

# Pull required models
docker exec seo-auditor-ollama ollama pull nomic-embed-text
docker exec seo-auditor-ollama ollama pull llama2
```

| Service | Purpose | Port |
|---------|---------|------|
| **ChromaDB** | Vector database for semantic search | 8000 |
| **Ollama** | Embeddings + LLM grammar | 11434 |
| **LanguageTool** | Grammar/spelling checking | 2372 |

## Report Output

Excel file with 7 sheets:

| Sheet | Content |
|-------|---------|
| **Summary** | Metrics, issue counts by severity (Critical/High/Medium/Low) |
| **Pages** | URL, Title, HTTP Status, Response Time |
| **Issues** | All findings with severity and message |
| **Technical SEO** | Canonical, Robots, Meta tag counts |
| **Links** | Source URL, Target URL, Anchor Text, Rel, Title |
| **Semantic** | Semantic query matches with scores |
| **Grammar** | LanguageTool/Ollama findings |

## Project Structure

```
seo-semantic-auditor/
├── src/
│   ├── main.rs              # Pipeline orchestration
│   ├── cli/                 # CLI argument parsing
│   ├── config/              # YAML configuration
│   ├── crawler/             # HTTP fetching, HTML parsing, URL discovery
│   ├── analyzer/            # SEO checks (CTA, anchors, SEO, duplicates)
│   ├── semantic/            # Chunking, embeddings, ChromaDB
│   ├── grammar/             # LanguageTool, Ollama LLM
│   └── exporter/            # Excel generation (rust_xlsxwriter)
├── queries/
│   ├── seo.yaml             # Basic queries
│   └── comprehensive.yaml   # 22 comprehensive checks
├── config.yaml              # Configuration
├── docker-compose.yml       # Service definitions
├── run_pipeline.sh          # Full automated pipeline
├── quick_start.sh           # Quick test (no Docker)
├── QUERIES.md               # Query system documentation
└── Cargo.toml
```

## Scripts

| Script | Purpose |
|--------|---------|
| `quick_start.sh` | Build + run basic SEO (no Docker) |
| `run_pipeline.sh` | Full pipeline: Docker, models, build, scan |

## Development

```bash
# Run tests
cargo test

# Check code
cargo check

# Format
cargo fmt

# Lint
cargo clippy
```

## License

MIT License - see [LICENSE](LICENSE)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add your checks to `queries/comprehensive.yaml` or create new query files
4. Submit a PR

## Roadmap

- [ ] JavaScript rendering (headless browser)
- [ ] Sitemap.xml generation
- [ ] CI/CD GitHub Action templates
- [ ] More query types (images, structured data, performance)
- [ ] Web dashboard
