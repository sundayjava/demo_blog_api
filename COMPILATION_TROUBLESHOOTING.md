# Compilation Troubleshooting Guide

This guide helps you fix common compilation errors when setting up the Blog API project.

## Common Error 1: Cannot Find Type

### Error Message
```
error[E0412]: cannot find type `PaginationParams` in this scope
error[E0412]: cannot find type `PaginatedResponse` in this scope
```

### Cause
The types are defined in `src/models/common.rs` but not properly imported.

### Solution 1: Check File Structure

Ensure these files exist:
```
src/models/
├── mod.rs          ← Must export common module
├── common.rs       ← Contains PaginationParams and PaginatedResponse
├── user.rs
├── post.rs
└── comment.rs
```

### Solution 2: Verify `src/models/mod.rs`

Must contain:
```rust
pub mod user;
pub mod post;
pub mod comment;
pub mod common;

// Re-export all types
pub use user::*;
pub use post::*;
pub use comment::*;
pub use common::*;
```

### Solution 3: Verify `src/models/common.rs`

Must contain:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 20 }

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.limit
    }

    pub fn validate(&mut self) {
        if self.page < 1 { self.page = 1; }
        if self.limit < 1 { self.limit = 20; }
        if self.limit > 100 { self.limit = 100; }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: i64, limit: i64, total: i64) -> Self {
        let total_pages = if total > 0 {
            (total as f64 / limit as f64).ceil() as i64
        } else {
            0
        };
        
        Self { data, page, limit, total, total_pages }
    }
}
```

### Solution 4: Import in Handlers

In your handler files (e.g., `src/handlers/user.rs`):
```rust
use crate::models::{PaginationParams, PaginatedResponse};
// or
use crate::models::*;
```

## Common Error 2: Module Not Found

### Error Message
```
error[E0583]: file not found for module `common`
```

### Solution

1. Create the file `src/models/common.rs`
2. Ensure it's in the correct location
3. Check file permissions (must be readable)

```bash
# Verify file exists
ls -la src/models/common.rs

# If missing, create it
touch src/models/common.rs
# Then add the content
```

## Common Error 3: Trait Not Implemented

### Error Message
```
error[E0277]: the trait `Deserialize` is not implemented for `PaginationParams`
```

### Solution

Ensure the derive macro is present:
```rust
#[derive(Debug, Deserialize, Clone)]  // ← Must have Deserialize
pub struct PaginationParams {
    // ...
}
```

Also check `Cargo.toml` has:
```toml
serde = { version = "1.0", features = ["derive"] }
```

## Common Error 4: Custom Validation Error

### Error Message
```
error: custom validation syntax error
```

### Solution

Use correct syntax for custom validation:
```rust
// WRONG
#[validate(custom = "validate_password_strength")]

// CORRECT
#[validate(custom(function = "validate_password_strength"))]
```

## Common Error 5: Cannot Find Module in Repository

### Error Message
```
error[E0583]: file not found for module `user`
   --> src/db/repository/mod.rs
```

### Solution

Check `src/db/repository/mod.rs`:
```rust
pub mod user;
pub mod post;
pub mod comment;

pub use user::*;
pub use post::*;
pub use comment::*;
```

Ensure these files exist:
- `src/db/repository/user.rs`
- `src/db/repository/post.rs`
- `src/db/repository/comment.rs`

## Common Error 6: Type Mismatch in Web Extractors

### Error Message
```
error[E0308]: mismatched types
expected struct `actix_web::web::Query<PaginationParams>`
found struct `PaginationParams`
```

### Solution

Use the correct extractor:
```rust
// For query parameters
pub async fn list_users(
    pagination: web::Query<PaginationParams>,  // ← Use web::Query
) -> AppResult<HttpResponse> {
    pagination.validate();  // Error - Query doesn't have validate
}

// Correct way
pub async fn list_users(
    mut pagination: web::Query<PaginationParams>,
) -> AppResult<HttpResponse> {
    pagination.validate();  // Call on inner value
    let params = pagination.into_inner();  // Extract inner value
}
```

## Common Error 7: Circular Dependencies

### Error Message
```
error[E0369]: cyclic dependency detected
```

### Solution

Check your module structure. Don't import parent modules from child modules.

**Bad:**
```rust
// In src/handlers/user.rs
use crate::handlers::*;  // Don't import parent
```

**Good:**
```rust
// In src/handlers/user.rs
use crate::models::*;
use crate::db::repository;
use crate::error::*;
```

## Common Error 8: Missing Async Runtime

### Error Message
```
error: async functions cannot be used in tests without a runtime
```

### Solution

Use `#[actix_web::test]` or `#[tokio::test]`:
```rust
// For actix-web tests
#[actix_web::test]
async fn test_something() {
    // test code
}

// For tokio tests
#[tokio::test]
async fn test_something() {
    // test code
}
```

## Complete Verification Checklist

Run these commands in order to diagnose issues:

```bash
# 1. Check all files exist
find src -name "*.rs" -type f

# 2. Verify module structure
cargo check --all

# 3. Check for syntax errors
cargo clippy

# 4. Format code
cargo fmt

# 5. Try building
cargo build

# 6. Run tests
cargo test
```

## Module Structure Reference

Your complete module structure should be:

```
src/
├── main.rs
├── error.rs
├── routes.rs
├── config/
│   └── mod.rs
├── db/
│   ├── mod.rs
│   └── repository/
│       ├── mod.rs
│       ├── user.rs
│       ├── post.rs
│       └── comment.rs
├── handlers/
│   ├── mod.rs
│   ├── auth.rs
│   ├── user.rs
│   ├── post.rs
│   └── comment.rs
├── middleware/
│   ├── mod.rs
│   └── auth.rs
├── models/
│   ├── mod.rs          ← MUST export common
│   ├── common.rs       ← Contains Pagination types
│   ├── user.rs
│   ├── post.rs
│   └── comment.rs
└── services/
    ├── mod.rs
    ├── jwt.rs
    └── password.rs
```

## Quick Fix Script

Create a file `check-structure.sh`:

```bash
#!/bin/bash

echo "Checking project structure..."

files=(
    "src/main.rs"
    "src/error.rs"
    "src/routes.rs"
    "src/config/mod.rs"
    "src/db/mod.rs"
    "src/db/repository/mod.rs"
    "src/db/repository/user.rs"
    "src/db/repository/post.rs"
    "src/db/repository/comment.rs"
    "src/handlers/mod.rs"
    "src/handlers/auth.rs"
    "src/handlers/user.rs"
    "src/handlers/post.rs"
    "src/handlers/comment.rs"
    "src/middleware/mod.rs"
    "src/middleware/auth.rs"
    "src/models/mod.rs"
    "src/models/common.rs"
    "src/models/user.rs"
    "src/models/post.rs"
    "src/models/comment.rs"
    "src/services/mod.rs"
    "src/services/jwt.rs"
    "src/services/password.rs"
)

missing=0
for file in "${files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ Missing: $file"
        missing=$((missing + 1))
    else
        echo "✓ Found: $file"
    fi
done

if [ $missing -eq 0 ]; then
    echo ""
    echo "✅ All files present!"
    echo "Running cargo check..."
    cargo check
else
    echo ""
    echo "❌ $missing files missing!"
    echo "Create missing files before building."
fi
```

Run it:
```bash
chmod +x check-structure.sh
./check-structure.sh
```

## Still Having Issues?

### Debug Import Chain

Add this to your `src/main.rs` temporarily:
```rust
// At the top of main.rs
mod models {
    pub mod common;
    pub mod user;
    pub mod post;
    pub mod comment;
}

// Then try to use the types
use models::common::{PaginationParams, PaginatedResponse};

fn main() {
    println!("PaginationParams type exists!");
}
```

If this works, the issue is with your module declarations.

### Check Cargo.toml Dependencies

Ensure you have:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

### Clean and Rebuild

```bash
cargo clean
rm -rf target/
cargo build
```

## Get Help

If you're still stuck:

1. Run `cargo --version` and `rustc --version`
2. Share the complete error message
3. Verify all files from the artifacts are copied correctly
4. Check that `src/models/common.rs` has the exact content provided

The types `PaginationParams` and `PaginatedResponse` **ARE** defined in the project - they're in `src/models/common.rs` and re-exported in `src/models/mod.rs`.