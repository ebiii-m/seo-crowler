# SEO Semantic Auditor - Query System Documentation

## Overview

The query system drives all SEO checks through YAML configuration. No Rust code changes are needed to add new checks.

## Query File Structure

```yaml
queries:
  - name: unique_identifier
    type: check_type
    # type-specific fields
```

## Query Types

### 1. Anchor Check (`type: anchor`)

Detects weak anchor text patterns in links.

```yaml
- name: weak_anchor
  type: anchor
  patterns:
    - click here
    - read more
    - learn more
    - here
    - this link
    - the link
    - button
    - link
```

**How it works:** Scans all `<a>` elements on each page. If anchor text contains any pattern (case-insensitive), creates a `Low` severity issue.

**Output:** Issues with type `Weak Anchor: {pattern}`

---

### 2. Semantic Search (`type: semantic`)

Finds content semantically similar to query phrases using embeddings.

```yaml
- name: service_search
  type: semantic
  query:
    - our services
    - what we offer
    - services page
  threshold: 0.8
```

**Fields:**
- `query`: Array of search phrases
- `threshold`: Similarity threshold (0.0-1.0), higher = more strict

**Requirements:** 
- ChromaDB running (`docker-compose up chroma`)
- Ollama with embedding model (`ollama pull nomic-embed-text`)
- `semantic.enabled: true` in config.yaml

**Output:** Issues with type `Semantic Match`, showing similarity score

---

## Complete Query Reference

### Built-in Checks (No Query Needed)

These run automatically without query configuration:

| Check | Severity | Description |
|-------|----------|-------------|
| Missing Title | High | Page has no `<title>` tag |
| Missing Headings | Medium | No `<h1>`-`<h6>` tags |
| Missing Description | Medium | No `meta[name="description"]` |
| Missing Canonical | Medium | No `link[rel="canonical"]` |
| Noindex | High | `meta[name="robots"]` contains `noindex` |
| Weak Anchor | Low | Anchor text matches patterns |
| Missing CTA | Medium | No call-to-action links found |
| Duplicate Title | High | Same title on multiple pages |
| Duplicate Description | Medium | Same description on multiple pages |

---

## Creating Custom Checks

### Anchor Patterns
Add new weak anchor patterns:

```yaml
- name: custom_anchors
  type: anchor
  patterns:
    - download now
    - view more
    - continue reading
    - see details
```

### Semantic Queries
Add content searches:

```yaml
- name: pricing_search
  type: semantic
  query:
    - pricing plans
    - subscription cost
    - how much does it cost
  threshold: 0.75

- name: contact_search
  type: semantic
  query:
    - contact us
    - get in touch
    - book a demo
    - schedule a call
  threshold: 0.8
```

### CTA Patterns (Built-in)
The CTA checker looks for these patterns automatically:
- contact, buy, book, schedule, get started
- sign up, register, subscribe, order now

To customize, modify `src/analyzer/cta.rs` and rebuild.

---

## Configuration Integration

### config.yaml
```yaml
queries:
  file: ./queries/seo.yaml

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
```

### Required Services
```bash
# Start ChromaDB
docker-compose up -d chroma

# Pull embedding model
ollama pull nomic-embed-text

# Start Ollama (if not running)
ollama serve
```

---

## Severity Levels

| Level | Description |
|-------|-------------|
| Critical | Site-breaking issues |
| High | Major SEO impact |
| Medium | Moderate SEO impact |
| Low | Minor issues / suggestions |

---

## Example: Complete Query File

See `queries/seo.yaml` for the comprehensive example with all check types.

---

## Extending the System

To add a new query type:

1. Add variant to `analyzer::Query` enum in `src/analyzer/mod.rs`
2. Create checker module in `src/analyzer/`
3. Add handling in `main.rs` query processing
4. No other code changes needed

The system is designed for YAML-driven extensibility.
