#!/bin/bash

set -e

echo "=========================================="
echo "Creating All Configuration Files"
echo "=========================================="
echo ""

# Create directories
echo "Creating directories..."
mkdir -p config
mkdir -p .github/workflows

# Create config/default.toml
echo "Creating config/default.toml..."
cat > config/default.toml << 'EOF'
environment = "development"

[server]
host = "127.0.0.1"
port = 8080

[database]
url = "postgres://postgres:postgres@localhost:5432/blog_api_dev"
max_connections = 5

[jwt]
secret = "dev_secret_key_change_in_production"
expiration_hours = 24
EOF

# Create config/production.toml
echo "Creating config/production.toml..."
cat > config/production.toml << 'EOF'
environment = "production"

[server]
host = "0.0.0.0"
port = 8080

[database]
max_connections = 20

[jwt]
expiration_hours = 24
EOF

# Create .env.example
echo "Creating .env.example..."
cat > .env.example << 'EOF'
# Application Environment
RUN_MODE=development

# Server Configuration
APP__SERVER__HOST=127.0.0.1
APP__SERVER__PORT=8080

# Database Configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/blog_api_dev
APP__DATABASE__URL=postgres://postgres:postgres@localhost:5432/blog_api_dev
APP__DATABASE__MAX_CONNECTIONS=5

# JWT Configuration
APP__JWT__SECRET=your-secret-key-change-this-in-production
APP__JWT__EXPIRATION_HOURS=24

# Test Database (for running tests)
TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/blog_api_test
EOF

# Create .gitignore
echo "Creating .gitignore..."
cat > .gitignore << 'EOF'
# Rust
/target/
**/*.rs.bk
*.pdb

# Cargo
Cargo.lock

# Environment variables
.env
.env.local
.env.*.local

# IDE
.vscode/
.idea/
*.swp
*.swo
*~
.DS_Store

# Database
*.db
*.sqlite
*.sqlite3

# Logs
*.log

# Test coverage
cobertura.xml
tarpaulin-report.html

# Docker
docker-compose.override.yml

# Documentation
/doc/

# Build artifacts
*.exe
*.dll
*.so
*.dylib
EOF

# Create .dockerignore
echo "Creating .dockerignore..."
cat > .dockerignore << 'EOF'
# Rust build artifacts
target/
**/*.rs.bk
*.pdb

# Git
.git/
.gitignore
.gitattributes

# Environment files
.env
.env.local
.env.*.local

# Documentation
*.md
!README.md
doc/

# Docker files
Dockerfile
docker-compose*.yml
.dockerignore

# CI/CD
.github/

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# Test files
tests/

# Logs
*.log

# OS files
.DS_Store
Thumbs.db
EOF

echo ""
echo "=========================================="
echo "✅ All configuration files created!"
echo "=========================================="
echo ""
echo "Files created:"
echo "  ✓ config/default.toml"
echo "  ✓ config/production.toml"
echo "  ✓ .env.example"
echo "  ✓ .gitignore"
echo "  ✓ .dockerignore"
echo ""
echo "Next steps:"
echo "1. Copy .env.example to .env: cp .env.example .env"
echo "2. Edit .env with your configuration"
echo "3. Continue with project setup"
echo ""