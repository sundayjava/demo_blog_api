.PHONY: help setup db-up db-down migrate test run build clean docker-up docker-down fmt lint check

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

setup: ## Install dependencies and set up the project
	@echo "Installing sqlx-cli..."
	cargo install sqlx-cli --no-default-features --features postgres
	@echo "Copying .env.example to .env..."
	cp -n .env.example .env || true
	@echo "Setup complete! Edit .env with your configuration."

db-up: ## Start PostgreSQL databases (dev and test)
	docker-compose -f docker-compose.dev.yml up -d
	@echo "Waiting for databases to be ready..."
	@sleep 5

db-down: ## Stop PostgreSQL databases
	docker-compose -f docker-compose.dev.yml down

migrate: ## Run database migrations
	sqlx migrate run

migrate-revert: ## Revert last migration
	sqlx migrate revert

test: ## Run all tests
	cargo test --verbose

test-coverage: ## Run tests with coverage
	cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Html

run: ## Run the application in development mode
	cargo run

build: ## Build the application in release mode
	cargo build --release

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/

docker-up: ## Start all services with Docker Compose
	docker-compose up --build -d

docker-down: ## Stop all Docker services
	docker-compose down

docker-logs: ## View Docker logs
	docker-compose logs -f app

fmt: ## Format code
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt -- --check

lint: ## Run clippy
	cargo clippy -- -D warnings

check: fmt-check lint test ## Run all checks (format, lint, test)

dev: db-up migrate run ## Start development environment

reset-db: ## Reset development database
	sqlx database drop -y || true
	sqlx database create
	sqlx migrate run

watch: ## Run with auto-reload
	cargo install cargo-watch
	cargo watch -x run