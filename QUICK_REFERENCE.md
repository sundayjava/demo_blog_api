# Blog API - Quick Reference Card

## 🚨 Critical Files Checklist

**These 4 files cause 90% of errors:**

### ✓ `src/models/common.rs` - Contains:
```rust
PaginationParams
PaginatedResponse<T>
```

### ✓ `src/models/mod.rs` - Must have:
```rust
pub mod common;
pub use common::*;
```

### ✓ `src/services/jwt.rs` - Complete JWT implementation

### ✓ `src/models/user.rs` - Correct validation syntax:
```rust
#[validate(custom(function = "validate_password_strength"))]
```

---

## 🏃 Quick Commands

```bash
# Setup
make setup && make db-up && make migrate

# Run
cargo run

# Test
curl http://localhost:8080/health

# Build
cargo build

# Test
cargo test
```

---

## 🔍 Quick Troubleshooting

| Error | Fix |
|-------|-----|
| Cannot find `PaginationParams` | Check `src/models/common.rs` exists |
| Module `common` not found | Check `src/models/mod.rs` |
| Custom validation error | Use `custom(function = "...")` |
| JWT service not found | Check `src/services/jwt.rs` |

---

## 📊 File Count: 49 Total

- Source: 24 files
- Config: 6 files  
- Migrations: 4 files
- Docker: 4 files
- Tests: 2 files
- CI/CD: 1 file
- Docs: 8 files

---

## 🧪 Test Endpoints

```bash
# Health
curl localhost:8080/health

# Register
curl -X POST localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","email":"test@test.com","password":"Pass123"}'

# Login
curl -X POST localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@test.com","password":"Pass123"}'
```

---

## 📁 Must-Have Directory Structure

```
src/
├── models/
│   ├── mod.rs       ← exports common
│   ├── common.rs    ← PaginationParams
│   ├── user.rs
│   ├── post.rs
│   └── comment.rs
├── services/
│   ├── mod.rs
│   ├── jwt.rs       ← JWT service
│   └── password.rs
└── (other directories...)
```

---

## 🐛 Debug Commands

```bash
# Verify files
./verify-files.sh

# Check common.rs
cat src/models/common.rs | head -20

# Clean build
cargo clean && cargo build

# Check migrations
sqlx migrate info
```

---

## 📚 Key Documentation

1. `README.md` - Full docs
2. `QUICKSTART.md` - 5-min setup
3. `STEP_BY_STEP_SETUP.md` - Detailed steps
4. `COMPILATION_TROUBLESHOOTING.md` - Fix errors
5. This file - Quick ref

---

## ⚡ Speed Run (3 minutes)

```bash
# 1. Create structure (30 sec)
mkdir -p blog-api/{src/{models,services},migrations,config}
cd blog-api

# 2. Critical files (60 sec)
# Copy content for these 4 files first:
# - src/models/common.rs
# - src/models/mod.rs  
# - src/services/jwt.rs
# - Cargo.toml

# 3. Copy remaining files (60 sec)
# Copy all other files from artifacts

# 4. Setup & Run (30 sec)
./setup.sh && cargo run
```

---

**Save this card for quick reference! 🚀**