# Testing Guide

Complete guide for testing the Blog API.

## Test Structure

```
tests/
├── common/
│   └── mod.rs           # Shared test utilities
└── integration/
    └── auth_tests.rs    # Authentication integration tests
```

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Specific Test Suite

```bash
# Integration tests only
cargo test --test auth_tests

# Unit tests only
cargo test --lib
```

### Run Specific Test

```bash
cargo test test_user_registration_success
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

### Run Tests in Parallel

```bash
cargo test -- --test-threads=4
```

## Test Database Setup

The tests use a separate test database to avoid affecting development data.

### Configure Test Database

In `.env`:
```env
TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/blog_api_test
```

### Create Test Database

```bash
# Using sqlx-cli
sqlx database create --database-url $TEST_DATABASE_URL

# Or manually
createdb blog_api_test
```

### Run Migrations on Test Database

```bash
DATABASE_URL=$TEST_DATABASE_URL sqlx migrate run
```

## Writing Tests

### Integration Test Example

```rust
use actix_web::{test, web, App};
use blog_api::handlers::auth;
use blog_api::models::CreateUserDto;

mod common;

#[actix_web::test]
async fn test_user_registration() {
    // Setup
    let pool = common::setup_test_db().await;
    let jwt_service = common::create_test_jwt_service();

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_service))
            .route("/auth/register", web::post().to(auth::register)),
    )
    .await;

    // Create test data
    let create_user_dto = CreateUserDto {
        username: common::generate_test_username(),
        email: common::generate_test_email(),
        password: "Password123".to_string(),
    };

    // Make request
    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&create_user_dto)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());

    // Cleanup
    common::cleanup_database(&pool).await;
}
```

### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "SecurePassword123";
        let hash = PasswordService::hash_password(password).unwrap();
        
        assert!(PasswordService::verify_password(password, &hash).unwrap());
        assert!(!PasswordService::verify_password("wrong", &hash).unwrap());
    }
}
```

## Test Utilities

### Setup Test Database

```rust
pub async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
```

### Cleanup Database

```rust
pub async fn cleanup_database(pool: &PgPool) {
    sqlx::query("TRUNCATE users, posts, comments, likes CASCADE")
        .execute(pool)
        .await
        .expect("Failed to cleanup database");
}
```

### Generate Test Data

```rust
pub fn generate_test_email() -> String {
    format!("test-{}@example.com", Uuid::new_v4())
}

pub fn generate_test_username() -> String {
    format!("testuser-{}", Uuid::new_v4().to_string()[..8].to_string())
}
```

## Test Coverage

### Install tarpaulin

```bash
cargo install cargo-tarpaulin
```

### Generate Coverage Report

```bash
# HTML report
cargo tarpaulin --out Html

# XML report (for CI)
cargo tarpaulin --out Xml

# Open HTML report
open tarpaulin-report.html  # macOS
xdg-open tarpaulin-report.html  # Linux
```

### Coverage in CI

The GitHub Actions workflow automatically generates coverage reports and uploads them to Codecov.

## Testing Best Practices

### 1. Use Descriptive Test Names

```rust
#[test]
fn test_user_registration_success() { }  // Good

#[test]
fn test1() { }  // Bad
```

### 2. Follow AAA Pattern

```rust
#[actix_web::test]
async fn test_login() {
    // Arrange
    let pool = setup_test_db().await;
    let user = create_test_user(&pool).await;
    
    // Act
    let result = login(&pool, user.email, "password").await;
    
    // Assert
    assert!(result.is_ok());
}
```

### 3. Clean Up After Tests

```rust
#[actix_web::test]
async fn test_something() {
    let pool = setup_test_db().await;
    
    // Test code here
    
    cleanup_database(&pool).await;  // Always cleanup
}
```

### 4. Test Edge Cases

```rust
#[test]
fn test_password_validation() {
    // Test minimum length
    assert!(validate_password("short").is_err());
    
    // Test maximum length
    assert!(validate_password(&"a".repeat(1000)).is_err());
    
    // Test special characters
    assert!(validate_password("Valid123!").is_ok());
    
    // Test empty string
    assert!(validate_password("").is_err());
}
```

### 5. Use Test Fixtures

```rust
// Create reusable test data
async fn create_test_user(pool: &PgPool) -> User {
    let dto = CreateUserDto {
        username: generate_test_username(),
        email: generate_test_email(),
        password: "Password123".to_string(),
    };
    
    repository::create_user(
        pool,
        dto,
        PasswordService::hash_password("Password123").unwrap()
    ).await.unwrap()
}
```

## Common Testing Scenarios

### Testing Protected Endpoints

```rust
#[actix_web::test]
async fn test_protected_endpoint() {
    let pool = setup_test_db().await;
    let jwt_service = create_test_jwt_service();
    
    // Create user and get token
    let user = create_test_user(&pool).await;
    let token = jwt_service
        .generate_token(user.id, user.email, user.role)
        .unwrap();
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(JwtAuth::new(jwt_service.clone()))
            .route("/protected", web::get().to(protected_handler))
    ).await;
    
    // Test with token
    let req = test::TestRequest::get()
        .uri("/protected")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    cleanup_database(&pool).await;
}
```

### Testing Validation Errors

```rust
#[actix_web::test]
async fn test_invalid_email() {
    let pool = setup_test_db().await;
    let jwt_service = create_test_jwt_service();
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_service))
            .route("/auth/register", web::post().to(auth::register))
    ).await;
    
    let dto = CreateUserDto {
        username: "testuser".to_string(),
        email: "invalid-email".to_string(),  // Invalid format
        password: "Password123".to_string(),
    };
    
    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&dto)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
    cleanup_database(&pool).await;
}
```

### Testing Database Transactions

```rust
#[sqlx::test]
async fn test_transaction_rollback(pool: PgPool) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    
    // Insert test data
    let user = create_user_in_transaction(&mut tx).await?;
    
    // Don't commit - rollback
    tx.rollback().await?;
    
    // Verify data was rolled back
    let result = get_user_by_id(&pool, user.id).await?;
    assert!(result.is_none());
    
    Ok(())
}
```

## Troubleshooting Tests

### Test Database Connection Issues

```bash
# Check if test database exists
psql -l | grep blog_api_test

# Recreate test database
dropdb blog_api_test
createdb blog_api_test
DATABASE_URL=$TEST_DATABASE_URL sqlx migrate run
```

### Tests Hanging

```bash
# Run with timeout
cargo test -- --test-threads=1 --nocapture

# Check for open connections
psql blog_api_test -c "SELECT * FROM pg_stat_activity"
```

### Flaky Tests

Common causes and solutions:

1. **Database state not cleaned**
   - Always cleanup after tests
   - Use transactions in tests

2. **Timing issues**
   - Add explicit waits
   - Use proper async/await

3. **Shared resources**
   - Use separate test databases
   - Run tests serially if needed

## Continuous Integration

Tests run automatically on:
- Every push to main/develop
- Every pull request

View test results in GitHub Actions.

## Manual Testing with curl

See [README.md](README.md#example-api-usage) for curl examples.

## Testing Checklist

Before committing:

- [ ] All tests pass locally
- [ ] New features have tests
- [ ] Edge cases are covered
- [ ] Tests clean up after themselves
- [ ] Test names are descriptive
- [ ] Code coverage is maintained or improved

## Additional Resources

- [Actix Web Testing Documentation](https://actix.rs/docs/testing/)
- [SQLx Testing Guide](https://github.com/launchbadge/sqlx#testing)
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)