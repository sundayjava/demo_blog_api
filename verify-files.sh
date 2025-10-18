#!/bin/bash

# Blog API - File Verification Script
# Checks if all required files exist

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "=========================================="
echo "Blog API - File Verification"
echo "=========================================="
echo ""

missing=0
total=0

check_file() {
    total=$((total + 1))
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} $1"
    else
        echo -e "${RED}✗${NC} $1 - MISSING!"
        missing=$((missing + 1))
    fi
}

echo -e "${BLUE}Checking source files...${NC}"
echo ""

# Core files
check_file "src/main.rs"
check_file "src/error.rs"
check_file "src/routes.rs"

# Config
check_file "src/config/mod.rs"

# Database
check_file "src/db/mod.rs"
check_file "src/db/repository/mod.rs"
check_file "src/db/repository/user.rs"
check_file "src/db/repository/post.rs"
check_file "src/db/repository/comment.rs"

# Handlers
check_file "src/handlers/mod.rs"
check_file "src/handlers/auth.rs"
check_file "src/handlers/user.rs"
check_file "src/handlers/post.rs"
check_file "src/handlers/comment.rs"

# Middleware
check_file "src/middleware/mod.rs"
check_file "src/middleware/auth.rs"

# Models - CRITICAL!
echo ""
echo -e "${YELLOW}⚠ CRITICAL FILES:${NC}"
check_file "src/models/mod.rs"
check_file "src/models/common.rs"
check_file "src/models/user.rs"
check_file "src/models/post.rs"
check_file "src/models/comment.rs"

# Services
echo ""
echo -e "${BLUE}Checking services...${NC}"
check_file "src/services/mod.rs"
check_file "src/services/jwt.rs"
check_file "src/services/password.rs"

echo ""
echo -e "${BLUE}Checking configuration files...${NC}"
echo ""

check_file "Cargo.toml"
check_file ".env.example"
check_file ".gitignore"
check_file ".dockerignore"
check_file "config/default.toml"
check_file "config/production.toml"

echo ""
echo -e "${BLUE}Checking Docker files...${NC}"
echo ""

check_file "Dockerfile"
check_file "docker-compose.yml"
check_file "docker-compose.dev.yml"

echo ""
echo -e "${BLUE}Checking migrations...${NC}"
echo ""

check_file "migrations/20240101000001_create_users_table.sql"
check_file "migrations/20240101000002_create_posts_table.sql"
check_file "migrations/20240101000003_create_comments_table.sql"
check_file "migrations/20240101000004_create_likes_table.sql"

echo ""
echo -e "${BLUE}Checking test files...${NC}"
echo ""

check_file "tests/common/mod.rs"
check_file "tests/integration/auth_tests.rs"

echo ""
echo "=========================================="

if [ $missing -eq 0 ]; then
    echo -e "${GREEN}✓ All $total files present!${NC}"
    echo ""
    
    # Check critical file contents
    echo -e "${BLUE}Verifying critical file contents...${NC}"
    echo ""
    
    if grep -q "struct PaginationParams" src/models/common.rs 2>/dev/null; then
        echo -e "${GREEN}✓${NC} PaginationParams found in common.rs"
    else
        echo -e "${RED}✗${NC} PaginationParams NOT found in common.rs"
        echo "  This file may be empty or incomplete!"
        missing=$((missing + 1))
    fi
    
    if grep -q "struct PaginatedResponse" src/models/common.rs 2>/dev/null; then
        echo -e "${GREEN}✓${NC} PaginatedResponse found in common.rs"
    else
        echo -e "${RED}✗${NC} PaginatedResponse NOT found in common.rs"
        echo "  This file may be empty or incomplete!"
        missing=$((missing + 1))
    fi
    
    if grep -q "pub mod common" src/models/mod.rs 2>/dev/null; then
        echo -e "${GREEN}✓${NC} common module declared in models/mod.rs"
    else
        echo -e "${RED}✗${NC} common module NOT declared in models/mod.rs"
        missing=$((missing + 1))
    fi
    
    if grep -q "pub use common" src/models/mod.rs 2>/dev/null; then
        echo -e "${GREEN}✓${NC} common module exported in models/mod.rs"
    else
        echo -e "${RED}✗${NC} common module NOT exported in models/mod.rs"
        missing=$((missing + 1))
    fi
    
    echo ""
    
    if [ $missing -eq 0 ]; then
        echo -e "${GREEN}✓ All content verified!${NC}"
        echo ""
        echo "Running cargo check..."
        if cargo check 2>&1 | grep -q "error"; then
            echo -e "${RED}✗ Compilation errors found!${NC}"
            echo ""
            echo "Run: cargo check --message-format=short"
            echo "See COMPILATION_TROUBLESHOOTING.md for help"
        else
            echo -e "${GREEN}✓ Project compiles successfully!${NC}"
            echo ""
            echo -e "${GREEN}🎉 Setup complete!${NC}"
            echo ""
            echo "Next steps:"
            echo "1. Copy .env.example to .env: cp .env.example .env"
            echo "2. Start databases: make db-up (or docker-compose -f docker-compose.dev.yml up -d)"
            echo "3. Run migrations: make migrate (or sqlx migrate run)"
            echo "4. Start server: cargo run"
        fi
    else
        echo -e "${RED}✗ Content verification failed!${NC}"
        echo ""
        echo "Fix these issues:"
        echo "1. Ensure src/models/common.rs has complete content from artifacts"
        echo "2. Ensure src/models/mod.rs declares and exports common module"
        echo "3. Run: ./create-all-configs.sh to create missing config files"
    fi
else
    echo -e "${RED}✗ $missing of $total files missing!${NC}"
    echo ""
    echo "To create missing configuration files, run:"
    echo "  ./create-all-configs.sh"
    echo ""
    echo "To create all directories:"
    echo "  mkdir -p src/{config,db/repository,handlers,middleware,models,services}"
    echo "  mkdir -p config migrations tests/{common,integration} .github/workflows"
    echo ""
    echo "Then copy file contents from the artifacts in the conversation."
fi

echo "=========================================="