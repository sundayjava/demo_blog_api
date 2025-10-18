#!/bin/bash

# Blog API Setup Script
# This script automates the setup process for the Blog API

set -e  # Exit on error

echo "=========================================="
echo "Blog API - Automated Setup"
echo "=========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${YELLOW}ℹ${NC} $1"
}

# Check if Rust is installed
echo "Checking prerequisites..."
if ! command -v cargo &> /dev/null; then
    print_error "Rust is not installed"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi
print_success "Rust is installed ($(rustc --version))"

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed"
    echo "Please install Docker from https://docs.docker.com/get-docker/"
    exit 1
fi
print_success "Docker is installed"

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    print_error "Docker Compose is not installed"
    echo "Please install Docker Compose from https://docs.docker.com/compose/install/"
    exit 1
fi
print_success "Docker Compose is installed"

echo ""
echo "Creating project directories..."

# Create all necessary directories
mkdir -p src/{config,db/repository,models,services,middleware,handlers}
mkdir -p migrations
mkdir -p tests/{common,integration}
mkdir -p config
mkdir -p .github/workflows

print_success "Directories created"

echo ""
echo "Setting up environment configuration..."

# Copy .env.example to .env if it doesn't exist
if [ ! -f .env ]; then
    if [ -f .env.example ]; then
        cp .env.example .env
        print_success ".env file created from .env.example"
        print_info "Please edit .env file with your configuration"
    else
        print_error ".env.example not found"
    fi
else
    print_info ".env file already exists"
fi

echo ""
echo "Installing sqlx-cli..."

# Check if sqlx-cli is already installed
if command -v sqlx &> /dev/null; then
    print_info "sqlx-cli is already installed"
else
    cargo install sqlx-cli --no-default-features --features postgres
    print_success "sqlx-cli installed"
fi

echo ""
echo "Starting PostgreSQL databases..."

# Start Docker containers
docker-compose -f docker-compose.dev.yml up -d

print_success "PostgreSQL containers started"
print_info "Waiting for databases to be ready..."
sleep 10

echo ""
echo "Running database migrations..."

# Run migrations
if sqlx migrate run; then
    print_success "Database migrations completed"
else
    print_error "Migration failed. Please check your database connection"
    exit 1
fi

echo ""
echo "Building the project..."

# Build the project
if cargo build; then
    print_success "Project built successfully"
else
    print_error "Build failed. Please check the error messages above"
    exit 1
fi

echo ""
echo "=========================================="
echo "Setup Complete! 🎉"
echo "=========================================="
echo ""
echo "Next steps:"
echo "1. Edit .env file with your configuration (if not done already)"
echo "2. Run 'cargo run' to start the server"
echo "3. Visit http://localhost:8080/health to verify the server is running"
echo ""
echo "Useful commands:"
echo "  make dev          - Start development environment"
echo "  make test         - Run tests"
echo "  make docker-up    - Start with Docker"
echo "  make help         - Show all available commands"
echo ""
echo "For more information, see README.md or QUICKSTART.md"
echo ""