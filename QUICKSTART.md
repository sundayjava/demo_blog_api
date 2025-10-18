# Quick Start Guide

Get the Blog API up and running in 5 minutes!

## Prerequisites

- Rust 1.75+ installed ([rustup.rs](https://rustup.rs/))
- Docker and Docker Compose installed

## Step-by-Step Setup

### 1. Clone and Enter Directory

```bash
git clone <your-repo-url>
cd blog-api
```

### 2. Quick Setup with Make

```bash
make setup
```

This will:
- Install `sqlx-cli`
- Copy `.env.example` to `.env`

### 3. Start Databases

```bash
make db-up
```

### 4. Run Migrations

```bash
make migrate
```

### 5. Start the API

```bash
make run
```

The API is now running at `http://localhost:8080` 🎉

## Test the API

### 1. Check Health

```bash
curl http://localhost:8080/health
```

### 2. Register a User

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "Password123"
  }'
```

Save the token from the response!

### 3. Create a Post

```bash
curl -X POST http://localhost:8080/api/posts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -d '{
    "title": "My First Post",
    "content": "Hello from the Blog API!",
    "published": true,
    "tags": ["rust", "actix-web"]
  }'
```

### 4. List Posts

```bash
curl http://localhost:8080/api/public/posts
```

## Common Commands

```bash
# Start development environment
make dev

# Run tests
make test

# Format code
make fmt

# Run linter
make lint

# Run all checks
make check

# Reset database
make reset-db

# View Docker logs
make docker-logs

# Stop databases
make db-down
```

## Using Docker Compose (Alternative)

If you prefer to run everything in Docker:

```bash
# Start all services (database + API)
make docker-up

# View logs
make docker-logs

# Stop all services
make docker-down
```

## Troubleshooting

### Port 5432 Already in Use

If you have PostgreSQL running locally:

```bash
# Stop local PostgreSQL
sudo systemctl stop postgresql  # Linux
brew services stop postgresql   # macOS

# Or change the port in docker-compose.dev.yml
```

### Database Connection Error

```bash
# Check if containers are running
docker ps

# Restart databases
make db-down
make db-up

# Wait a few seconds, then run migrations
make migrate
```

### Migration Errors

```bash
# Reset database
make reset-db
```

## Next Steps

- Check out the full [README.md](README.md) for detailed documentation
- Explore the [API endpoints](#) in the README
- Read about [authentication](#) and [authorization](#)
- Set up your IDE with Rust analyzer

## Development Tips

1. **Auto-reload during development:**
   ```bash
   make watch
   ```

2. **Check code before committing:**
   ```bash
   make check
   ```

3. **Generate test coverage:**
   ```bash
   make test-coverage
   ```

4. **Clean build artifacts:**
   ```bash
   make clean
   ```

## Getting Help

- Read the [README.md](README.md) for comprehensive documentation
- Check [GitHub Issues](issues-url) for common problems
- Review the [API documentation](#) for endpoint details

Happy coding! 🦀