#!/bin/bash
# SEO Semantic Auditor - Full Pipeline Runner
# Sets up services, builds project, and runs scan

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default configuration
SITE_URL="${1:-https://example.com}"
QUERY_FILE="${2:-queries/comprehensive.yaml}"
OUTPUT_FILE="${3:-reports/seo-report.xlsx}"
CONFIG_FILE="${4:-config.yaml}"

# Service ports
CHROMA_PORT=8000
OLLAMA_PORT=11434
LANGUAGETOOL_PORT=2372

# Embedding model
EMBEDDING_MODEL="nomic-embed-text"
OLLAMA_MODEL="llama2"

print_header() {
    echo -e "\n${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

check_command() {
    if command -v "$1" &> /dev/null; then
        print_success "$1 found: $(command -v $1)"
        return 0
    else
        print_error "$1 not found"
        return 1
    fi
}

wait_for_service() {
    local url=$1
    local name=$2
    local max_attempts=30
    local attempt=1
    
    echo -n "Waiting for $name..."
    while [ $attempt -le $max_attempts ]; do
        if curl -s -f "$url" > /dev/null 2>&1; then
            echo -e " ${GREEN}ready${NC}"
            return 0
        fi
        echo -n "."
        sleep 2
        ((attempt++))
    done
    echo -e " ${RED}timeout${NC}"
    return 1
}

# ============================================
# Main Pipeline
# ============================================

print_header "SEO Semantic Auditor - Full Pipeline"
echo "Site: $SITE_URL"
echo "Query: $QUERY_FILE"
echo "Output: $OUTPUT_FILE"
echo "Config: $CONFIG_FILE"

# 1. Check prerequisites
print_header "Checking Prerequisites"

check_command docker || { print_error "Docker required but not installed"; exit 1; }
check_command docker-compose || { print_error "Docker Compose required"; exit 1; }
check_command cargo || { print_error "Rust/Cargo required"; exit 1; }
check_command curl || { print_error "curl required"; exit 1; }

# 2. Start Docker services
print_header "Starting Docker Services"

# Check if docker-compose.yml exists
if [ ! -f "docker-compose.yml" ]; then
    print_warning "docker-compose.yml not found, creating minimal one..."
    cat > docker-compose.yml << 'EOF'
services:
  chroma:
    image: chromadb/chroma:latest
    ports:
      - "8000:8000"
    volumes:
      - chroma_data:/chroma/blazing
    environment:
      - IS_PERSISTENT=true
      - PERSISTENCE_PATH=/chroma/blazing

  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
    volumes:
      - ollama_data:/root/.ollama

  languagetool:
    image: erikvl87/languagetool:latest
    ports:
      - "2372:8081"

volumes:
  chroma_data:
  ollama_data:
EOF
fi

print_header "Starting ChromaDB, Ollama, LanguageTool"
docker-compose up -d

# 3. Wait for services
print_header "Waiting for Services to be Ready"

wait_for_service "http://localhost:$CHROMA_PORT/api/v1/heartbeat" "ChromaDB" || {
    print_warning "ChromaDB not ready, continuing anyway..."
}

wait_for_service "http://localhost:$OLLAMA_PORT/api/tags" "Ollama" || {
    print_warning "Ollama not ready, continuing anyway..."
}

wait_for_service "http://localhost:$LANGUAGETOOL_PORT/v2/languages" "LanguageTool" || {
    print_warning "LanguageTool not ready, continuing anyway..."
}

# 4. Pull embedding model
print_header "Pulling Embedding Model ($EMBEDDING_MODEL)"
if docker exec seo-semantic-auditor-ollama-1 ollama list | grep -q "$EMBEDDING_MODEL"; then
    print_success "$EMBEDDING_MODEL already pulled"
else
    echo "Pulling $EMBEDDING_MODEL (this may take a few minutes)..."
    docker exec seo-semantic-auditor-ollama-1 ollama pull "$EMBEDDING_MODEL" &&         print_success "$EMBEDDING_MODEL pulled" ||         print_warning "Failed to pull $EMBEDDING_MODEL"
fi

# 5. Pull Ollama model for grammar
print_header "Pulling Grammar Model ($OLLAMA_MODEL)"
if docker exec seo-semantic-auditor-ollama-1 ollama list | grep -q "$OLLAMA_MODEL"; then
    print_success "$OLLAMA_MODEL already pulled"
else
    echo "Pulling $OLLAMA_MODEL (this may take a few minutes)..."
    docker exec seo-semantic-auditor-ollama-1 ollama pull "$OLLAMA_MODEL" &&         print_success "$OLLAMA_MODEL pulled" ||         print_warning "Failed to pull $OLLAMA_MODEL"
fi

# 6. Build project
print_header "Building SEO Semantic Auditor (Release)"
cargo build --release && print_success "Build complete" || {
    print_error "Build failed"
    exit 1
}

# 7. Create output directory
mkdir -p "$(dirname "$OUTPUT_FILE")"

# 8. Run scan
print_header "Running SEO Scan"
echo "Command: ./target/release/seo-semantic-auditor scan --site "$SITE_URL" --query "$QUERY_FILE" --output "$OUTPUT_FILE" --config "$CONFIG_FILE""
echo ""

./target/release/seo-semantic-auditor scan     --site "$SITE_URL"     --query "$QUERY_FILE"     --output "$OUTPUT_FILE"     --config "$CONFIG_FILE"

# 9. Verify output
if [ -f "$OUTPUT_FILE" ]; then
    print_header "Scan Complete"
    print_success "Report generated: $OUTPUT_FILE"
    echo "File size: $(du -h "$OUTPUT_FILE" | cut -f1)"
    echo ""
    echo "Open the Excel file to view results:"
    echo "  Sheets: Summary, Pages, Issues, Technical SEO, Links, Semantic, Grammar"
else
    print_error "Report not generated"
    exit 1
fi

print_header "Pipeline Complete"
echo -e "${GREEN}All done!${NC} Report: $OUTPUT_FILE"
