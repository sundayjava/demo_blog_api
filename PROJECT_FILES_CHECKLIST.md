# Blog API - Complete File Checklist

Use this checklist to ensure all files are created correctly.

## Configuration Files

- [ ] `Cargo.toml` - Project dependencies
- [ ] `.env.example` - Environment variables template
- [ ] `.gitignore` - Git ignore rules
- [ ] `Makefile` - Development commands
- [ ] `config/default.toml` - Default configuration
- [ ] `config/production.toml` - Production configuration

## Database Migrations

- [ ] `migrations/20240101000001_create_users_table.sql`
- [ ] `migrations/20240101000002_create_posts_table.sql`
- [ ] `migrations/20240101000003_create_comments_table.sql`
- [ ] `migrations/20240101000004_create_likes_table.sql`

## Source Code - Core

- [ ] `src/main.rs` - Application entry point
- [ ] `src/error.rs` - Custom error types
- [ ] `src/routes.rs` - Route configuration

## Source Code - Configuration

- [ ] `src/config/mod.rs` - Configuration module

## Source Code - Database

- [ ] `src/db/mod.rs` - Database connection
- [ ] `src/db/repository/mod.rs` - Repository module
- [ ] `src/db/repository/user.rs` - User repository
- [ ] `src/db/repository/post.rs` - Post repository
- [ ] `src/db/repository/comment.rs` - Comment repository

## Source Code - Models

- [ ] `src/models/mod.rs` - Models module
- [ ] `src/models/user.rs` - User models and DTOs
- [ ] `src/models/post.rs` - Post models and DTOs
- [ ] `src/models/comment.rs` - Comment models and DTOs
- [ ] `src/models/common.rs` - Common models (pagination)

## Source Code - Services

- [ ] `src/services/mod.rs` - Services module
- [ ] `src/services/password.rs` - Password hashing service
- [ ] `src/services/jwt.rs` - JWT authentication service

## Source Code - Middleware

- [ ] `src/middleware/mod.rs` - Middleware module
- [ ] `src/middleware/auth.rs` - Authentication middleware

## Source Code - Handlers

- [ ] `src/handlers/mod.rs` - Handlers module
- [ ] `src/handlers/auth.rs` - Authentication handlers
- [ ] `src/handlers/user.rs` - User handlers
- [ ] `src/handlers/post.rs` - Post handlers
- [ ] `src/handlers/comment.rs` - Comment handlers

## Tests

- [ ] `tests/common/mod.rs` - Test utilities
- [ ] `tests/integration/auth_tests.rs` - Authentication tests

## Docker

- [ ] `Dockerfile` - Production Docker image
- [ ] `docker-compose.yml` - Production docker compose
- [ ] `docker-compose.dev.yml` - Development docker compose
- [ ] `.dockerignore` - Docker ignore rules

## CI/CD

- [ ] `.github/workflows/ci.yml` - GitHub Actions workflow

## Documentation

- [ ] `README.md` - Main documentation
- [ ] `QUICKSTART.md` - Quick start guide
- [ ] `PROJECT_FILES_CHECKLIST.md` - This file

## Quick Create Script

To verify all directories exist, run:

```bash
# Create all necessary directories
mkdir -p src/{config,db/repository,models,services,middleware,handlers}
mkdir -p migrations
mkdir -p tests/{common,integration}
mkdir -p config
mkdir -p .github/workflows
```

## File Content Summary

### Critical Files That Must Work Together

1. **src/main.rs** → Imports all modules and starts server
2. **src/routes.rs** → Configures all HTTP routes
3. **src/middleware/auth.rs** → Protects authenticated routes
4. **src/services/jwt.rs** → Generates and verifies JWT tokens
5. **src/handlers/*.rs** → Handle HTTP requests
6. **src/db/repository/*.rs** → Database operations

### Module Import Chain

```
main.rs
├── config (mod.rs)
├── db (mod.rs)
│   └── repository (mod.rs)
│       ├── user.rs
│       ├── post.rs
│       └── comment.rs
├── error.rs
├── handlers (mod.rs)
│   ├── auth.rs
│   ├── user.rs
│   ├── post.rs
│   └── comment.rs
├── middleware (mod.rs)
│   └── auth.rs
├── models (mod.rs)
│   ├── user.rs
│   ├── post.rs
│   ├── comment.rs
│   └── common.rs
├── routes.rs
└── services (mod.rs)
    ├── jwt.rs
    └── password.rs
```

## Verification Steps

### 1. Check Rust Compilation

```bash
cargo check
```

Should complete without errors.

### 2. Check Database Connection

```bash
# Start databases
make db-up

# Run migrations
make migrate
```

### 3. Run Tests

```bash
cargo test
```

### 4. Build Project

```bash
cargo build
```

### 5. Run Application

```bash
cargo run
```

Should start without panics.

## Common Issues and Fixes

### Issue: "cannot find module"

**Fix:** Ensure `mod.rs` files exist and declare submodules correctly.

Example `src/models/mod.rs`:
```rust
pub mod user;
pub mod post;
pub mod comment;
pub mod common;

pub use user::*;
pub use post::*;
pub use comment::*;
pub use common::*;
```

### Issue: "unused import"

**Fix:** Check if the import is actually needed or remove it.

### Issue: "cannot find type in this scope"

**Fix:** Add the correct import or use full path:
```rust
use crate::models::User;
// or
crate::models::User
```

### Issue: Migration fails

**Fix:**
```bash
# Reset database
make reset-db
```

### Issue: Port already in use

**Fix:**
```bash
# Stop existing process
lsof -ti:8080 | xargs kill -9

# Or change port in config
```

## Additional Files You May Want

### Optional Enhancement Files

- [ ] `src/utils/mod.rs` - Utility functions
- [ ] `src/utils/slug.rs` - Slug generation
- [ ] `src/middleware/rate_limit.rs` - Rate limiting
- [ ] `src/middleware/request_id.rs` - Request ID tracking
- [ ] `tests/integration/user_tests.rs` - User tests
- [ ] `tests/integration/post_tests.rs` - Post tests
- [ ] `tests/integration/comment_tests.rs` - Comment tests
- [ ] `LICENSE` - Project license
- [ ] `CONTRIBUTING.md` - Contribution guidelines
- [ ] `CHANGELOG.md` - Version changelog
- [ ] `.editorconfig` - Editor configuration
- [ ] `rustfmt.toml` - Rust formatter config
- [ ] `clippy.toml` - Clippy configuration

## Final Verification

Run this command to check all critical files exist:

```bash
ls -la src/main.rs \
  src/config/mod.rs \
  src/db/mod.rs \
  src/db/repository/mod.rs \
  src/error.rs \
  src/routes.rs \
  src/services/jwt.rs \
  src/services/password.rs \
  src/middleware/auth.rs \
  Cargo.toml
```

All files should exist. If any are missing, create them using the artifact code provided.