# Step-by-Step Setup Guide

Follow these exact steps to set up the Blog API project without errors.

## Prerequisites Check

```bash
# Check Rust version (need 1.75+)
rustc --version

# Check Cargo
cargo --version

# Check Docker
docker --version

# Check Docker Compose
docker-compose --version
```

All should return version numbers. If not, install the missing tools.

## Step 1: Create Project Structure

```bash
# Create project directory
mkdir blog-api
cd blog-api

# Create all directories at once
mkdir -p .github/workflows
mkdir -p config
mkdir -p migrations
mkdir -p src/{config,db/repository,handlers,middleware,models,services}
mkdir -p tests/{common,integration}
```

Verify:
```bash
tree -d -L 3
```

Should show all directories created.

## Step 2: Create Critical Model Files First

These files must be created in this order to avoid import errors.

### 2.1 Create `src/models/common.rs`

This file contains `PaginationParams` and `PaginatedResponse`.

```bash
cat > src/models/common.rs << 'EOF'
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.limit
    }

    pub fn validate(&mut self) {
        if self.page < 1 {
            self.page = 1;
        }
        if self.limit < 1 {
            self.limit = 20;
        }
        if self.limit > 100 {
            self.limit = 100;
        }
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
        
        Self {
            data,
            page,
            limit,
            total,
            total_pages,
        }
    }
}
EOF
```

### 2.2 Verify the file was created

```bash
cat src/models/common.rs | head -n 20
```

You should see the PaginationParams struct definition.

### 2.3 Create `src/models/mod.rs`

```bash
cat > src/models/mod.rs << 'EOF'
pub mod user;
pub mod post;
pub mod comment;
pub mod common;

// Re-export all types
pub use user::*;
pub use post::*;
pub use comment::*;
pub use common::*;
EOF
```

### 2.4 Create other model files

Now copy the content from artifacts for:
- `src/models/user.rs`
- `src/models/post.rs`
- `src/models/comment.rs`

## Step 3: Create All mod.rs Files

These must exist before creating the main files:

```bash
# Create all mod.rs files
touch src/config/mod.rs
touch src/db/mod.rs
touch src/db/repository/mod.rs
touch src/handlers/mod.rs
touch src/middleware/mod.rs
touch src/services/mod.rs
```

### 3.1 Fill in `src/services/mod.rs`

```bash
cat > src/services/mod.rs << 'EOF'
pub mod password;
pub mod jwt;

pub use password::PasswordService;
pub use jwt::{JwtService, Claims};
EOF
```

### 3.2 Fill in `src/db/repository/mod.rs`

```bash
cat > src/db/repository/mod.rs << 'EOF'
pub mod user;
pub mod post;
pub mod comment;

pub use user::*;
pub use post::*;
pub use comment::*;
EOF
```

### 3.3 Fill in `src/handlers/mod.rs`

```bash
cat > src/handlers/mod.rs << 'EOF'
pub mod auth;
pub mod user;
pub mod post;
pub mod comment;
EOF
```

### 3.4 Fill in `src/middleware/mod.rs`

```bash
cat > src/middleware/mod.rs << 'EOF'
pub mod auth;

pub use auth::{AuthenticatedUser, JwtAuth};
EOF
```

## Step 4: Create Cargo.toml

```bash
cat > Cargo.toml << 'EOF'
[package]
name = "blog-api"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.4"
actix-cors = "0.7"
actix-multipart = "0.6"
actix-files = "0.6"
tokio = { version = "1.35", features = ["full"] }
sqlx = { version = "0.7", features = [
    "runtime-tokio-native-tls",
    "postgres",
    "uuid",
    "chrono",
    "migrate"
] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
argon2 = "0.5"
jsonwebtoken = "9"
validator = { version = "0.16", features = ["derive"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
rand = "0.8"
dotenv = "0.15"
config = "0.13"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-actix-web = "0.7"
thiserror = "1.0"
anyhow = "1.0"

[dev-dependencies]
actix-rt = "2.9"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
EOF
```

## Step 5: Copy Remaining Files

Now copy content from artifacts for all remaining files. Use the artifact IDs provided.

Critical files to copy:
1. All files in `src/` directory
2. All migration files in `migrations/`
3. Configuration files (`config/*.toml`)
4. `.env.example`
5. Docker files

## Step 6: Verification Script

```bash
# Make verification script executable
chmod +x verify-files.sh

# Run it
./verify-files.sh
```

This will check all files and attempt compilation.

## Step 7: Verify models/common.rs Content

This is the most common source of errors:

```bash
# Check if PaginationParams exists
grep -n "struct PaginationParams" src/models/common.rs

# Check if PaginatedResponse exists
grep -n "struct PaginatedResponse" src/models/common.rs

# Check file size (should be around 1.5KB)
ls -lh src/models/common.rs
```

Expected output:
```
-rw-r--r-- 1 user user 1.5K Jan 1 12:00 src/models/common.rs
```

## Step 8: Test Compilation

```bash
# Clean any previous builds
cargo clean

# Check for errors (faster than build)
cargo check

# If check passes, try build
cargo build
```

### Common Errors and Fixes

#### Error: "cannot find type `PaginationParams`"

**Fix:**
```bash
# Verify the file exists and has content
cat src/models/common.rs

# If empty or missing, recreate it (see Step 2.1)

# Ensure mod.rs exports it
grep "pub use common" src/models/mod.rs
```

#### Error: "module `common` not found"

**Fix:**
```bash
# Check src/models/mod.rs declares common
grep "pub mod common" src/models/mod.rs

# If missing, add it
echo "pub mod common;" >> src/models/mod.rs
```