# SEO Semantic Auditor

## Project Handoff For Local Coding Agent

## Overview

Build a Rust based SEO intelligence crawler.

Main goals:

* Crawl one or multiple websites
* Discover URLs from sitemap
* Extract page content
* Analyze SEO problems
* Perform semantic search using local ChromaDB
* Use local embedding models
* Detect grammar/spelling issues
* Export reports as Excel

---

# Technology Stack

Language:

* Rust

Runtime:

* Tokio async runtime

Main libraries:

* reqwest
* scraper
* quick-xml
* serde
* serde_yaml
* clap
* rust_xlsxwriter
* async-trait

External services:

## Vector Database

ChromaDB local

## Embedding Provider

Default:

Ollama

Recommended models:

* nomic-embed-text
* bge-small

## Grammar

Options:

* LanguageTool local server
* Ollama LLM

---

# Final Project Structure

```
seo-semantic-auditor/

Cargo.toml
README.md
config.yaml
docker-compose.yml

queries/

    seo.yaml
    ecommerce.yaml
    content.yaml


src/

    main.rs

    cli/

        mod.rs


    config/

        mod.rs


    crawler/

        mod.rs
        robots.rs
        sitemap.rs
        fetcher.rs
        parser.rs
        crawler.rs
        cache.rs


    analyzer/

        mod.rs
        models.rs
        cta.rs
        anchor.rs
        seo.rs
        duplicate.rs


    semantic/

        mod.rs
        models.rs
        chunker.rs
        embedding.rs
        chroma.rs


    grammar/

        mod.rs
        models.rs
        languagetool.rs
        ollama.rs


    exporter/

        mod.rs
        excel.rs
        models.rs


    pipeline/

        mod.rs


tests/

    sitemap_test.rs
    parser_test.rs
    analyzer_test.rs
```

---

# Execution Flow

```
CLI
 |
 |
Config Loader
 |
 |
Pipeline
 |
 |
Sitemap Discovery
 |
 |
URL Collector
 |
 |
Concurrent Crawler
 |
 |
HTML Parser
 |
 +----------------+
 |                |
 v                v

SEO Analyzer     Semantic Index


 |
 |
Grammar Checker


 |
 |
Report Generator


 |
 |
Excel File
```

---

# Config Example

config.yaml

```yaml
site:

  url:
    https://example.com


crawler:

  concurrency: 10

  timeout: 20


semantic:

  enabled: true

  chroma:
    http://localhost:8000


embedding:

  provider:
    ollama

  model:
    nomic-embed-text


grammar:

  enabled: true


queries:

  file:
    ./queries/seo.yaml


report:

  output:
    ./reports/seo-report.xlsx
```

---

# Query System

The analyzer must be driven by YAML.

Example:

queries/seo.yaml

```yaml
queries:


- name: weak_anchor

  type: anchor


  patterns:

    - click here

    - read more



- name: service_search

  type: semantic


  query:

    - our services

    - what we offer


  threshold:

    0.8
```

Adding new checks should not require Rust code changes.

---

# Module Responsibilities

## crawler

Responsibilities:

* robots.txt parser
* sitemap detection
* sitemap index support
* URL extraction
* concurrent fetching
* retry
* timeout
* status codes
* response time

Output:

Page HTML

---

## parser

Extract:

* title
* meta description
* headings
* body text
* links
* anchor text
* canonical
* robots meta

Output:

PageData

Example:

```json
{
"url":"",
"title":"",
"description":"",
"headings":[],
"text":"",
"links":[]
}
```

---

## analyzer

Checks:

### CTA

Find:

* Contact
* Buy
* Book
* Schedule
* Get started

Output:

Issue

---

### Anchor

Detect weak anchors:

Examples:

* click here
* here
* read more

---

### SEO

Detect:

* missing title
* missing heading
* missing description
* missing canonical
* noindex

---

### Duplicate

Detect:

* duplicate titles
* duplicate descriptions

---

# Semantic Layer

Purpose:

Semantic content search.

Flow:

```
Text

 |

Chunking

 |

Embedding

 |

Vector

 |

ChromaDB
```

Chunk size:

Around 300 words

Metadata:

```json
{
"url":"",
"type":"page"
}
```

Search example:

Query:

```
book appointment
```

Should find:

```
Schedule your visit today
```

---

# Chroma Requirements

Docker:

```yaml
services:

 chroma:

  image:
    chromadb/chroma

  ports:

    - 8000:8000
```

---

# Ollama Requirements

Install:

```
ollama pull nomic-embed-text
```

Endpoint:

```
http://localhost:11434
```

---

# Grammar Layer

Interface:

GrammarProvider

Providers:

## LanguageTool

For:

* spelling
* grammar

## Ollama

For:

* rewriting
* suggestions

Output:

```json
{
"url":"",
"problem":"",
"suggestion":""
}
```

---

# Excel Output

Final file:

```
seo-report.xlsx
```

Sheets:

## Summary

Metrics

## Pages

URL information

## Issues

All detected problems

## Technical SEO

Status codes

Canonical

Robots

## Links

Anchor analysis

## Semantic

Semantic matches

## Grammar

Language problems

---

# CLI

Expected:

```
seo-auditor scan \
--site https://example.com \
--query queries/seo.yaml \
--output reports/
```

---

# Production Requirements

Before final release:

Run:

```
cargo check

cargo test

cargo build --release
```

Fix:

* imports
* module paths
* async errors
* serialization issues

---

# Remaining Development Tasks

Priority order:

1. Complete module implementation

2. Make project compile

3. Connect Chroma API

4. Connect Ollama embeddings

5. Finish Excel exporter

6. Add integration tests

7. Add CLI

8. Benchmark crawler

---

# Important Design Rules

* Do not hardcode SEO checks.
* New checks must be YAML driven.
* Keep embedding provider abstract.
* Keep Chroma optional.
* Everything should work locally.
* Avoid sending website content to external APIs by default.

End of handoff.

