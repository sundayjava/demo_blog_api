# Missing Files Fix Guide

This guide helps you create the configuration files that are missing.

## Quick Fix: Run the Script

```bash
# Make script executable
chmod +x create-all-configs.sh

# Run it
./create-all-configs.sh
```

This will create:
- ✓ `config/default.toml`
- ✓ `config/production.toml`
- ✓ `.env.example`
- ✓ `.gitignore`
- ✓ `.dockerignore`

## Manual Creation (If Script Doesn't Work)

### 1. Create `config/default.toml`

```bash
mkdir -p config

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
```

### 2. Create `config/production.toml`

```bash
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
```

**Note**: Production config doesn't include sensitive values (database URL, JWT secret). These should be provided via environment variables.

## Verify Configuration Files

```bash
# Check if files exist
ls -lh config/

# View contents
cat config/default.toml
cat config/production.toml
```

Expected output:
```
-rw-r--r-- 1 user user  193 Jan 1 12:00 default.toml
-rw-r--r-- 1 user user  103 Jan 1 12:00 production.toml
```

## Configuration File Explanation

### `config/default.toml`

Default configuration used in development. Contains:
- **environment**: "development" - sets the running mode
- **server.host**: "127.0.0.1" - localhost only
- **server.port**: 8080 - API port
- **database.url**: Connection string for dev database
- **database.max_connections**: 5 - pool size for dev
- **jwt.secret**: Default secret (CHANGE IN PRODUCTION!)
- **jwt.expiration_hours**: 24 - token validity period

### `config/production.toml`

Production overrides. Only includes values that differ from defaults:
- **environment**: "production"
- **server.host**: "0.0.0.0" - listen on all interfaces
- **database.max_connections**: 20 - larger pool for production

Missing values (database URL, JWT secret) come from environment variables for security.

## Environment Variables Override

You can override any config value with environment variables:

```bash
# Format: APP__SECTION__KEY
export APP__SERVER__PORT=9000
export APP__DATABASE__URL=postgres://user:pass@prod-db:5432/blog_api
export APP__JWT__SECRET=super-secure-production-secret
```

Priority (highest to lowest):
1. Environment variables (APP__*)
2. Environment-specific config (production.toml)
3. Default config (default.toml)

## Complete Setup Checklist

After creating config files:

```bash
# 1. Verify configuration files
./verify-files.sh

# 2. Create .env for local development
cp .env.example .env

# 3. Edit .env with your settings
nano .env

# 4. Start databases
docker-compose -f docker-compose.dev.yml up -d

# 5. Run migrations
sqlx migrate run

# 6. Start server
cargo run
```

## Configuration for Different Environments

### Development (Local)
```bash
# Uses config/default.toml
cargo run
```

### Production
```bash
# Set environment
export RUN_MODE=production

# Set secrets via environment variables
export APP__DATABASE__URL=postgres://prod-user:prod-pass@prod-host:5432/blog_api
export APP__JWT__SECRET=your-production-secret-key

# Run
cargo run
```

### Testing
```bash
# Uses TEST_DATABASE_URL from .env
cargo test
```

## Troubleshooting

### Error: "Failed to load configuration"

**Cause**: Config files missing or malformed

**Fix**:
```bash
# Recreate config files
./create-all-configs.sh

# Verify TOML syntax
cargo install taplo-cli
taplo check config/*.toml
```

### Error: "No such file or directory: config/default.toml"

**Cause**: Config directory doesn't exist or files not created

**Fix**:
```bash
# Create directory
mkdir -p config

# Create files manually (see sections above)
```

### Error: "Invalid TOML"

**Cause**: Syntax error in TOML file

**Fix**:
```bash
# Check for syntax errors
cat config/default.toml

# Look for:
# - Missing quotes around strings
# - Incorrect section headers
# - Typos in key names
```

## Additional Configuration Files

### `.env.example` (If Missing)

```bash
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

# Test Database
TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/blog_api_test
EOF
```

### `.gitignore` (If Missing)

```bash
cat > .gitignore << 'EOF'
/target/
**/*.rs.bk
Cargo.lock
.env
.env.local
.env.*.local
.vscode/
.idea/
*.swp
*.swo
*~
.DS_Store
*.db
*.sqlite
*.sqlite3
*.log
cobertura.xml
tarpaulin-report.html
docker-compose.override.yml
/doc/
*.exe
*.dll
*.so
*.dylib
EOF
```

## Configuration Best Practices

1. **Never commit secrets**: Use `.env` files (git-ignored) for secrets
2. **Use environment variables in production**: Don't hardcode production credentials
3. **Separate configs per environment**: Use `RUN_MODE` to switch configs
4. **Validate on startup**: App will fail early if config is invalid
5. **Document required variables**: Keep `.env.example` up to date

## Next Steps

After creating configuration files:

1. ✅ Verify all files exist: `./verify-files.sh`
2. ✅ Create `.env`: `cp .env.example .env`
3. ✅ Edit `.env` with your settings
4. ✅ Continue with database setup

See `STEP_BY_STEP_SETUP.md` for complete setup instructions.