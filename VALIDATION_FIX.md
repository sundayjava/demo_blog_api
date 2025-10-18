# Custom Validation Fix

## Issue

The custom validation syntax in `src/models/user.rs` was incorrect.

## ❌ Incorrect Syntax

```rust
#[validate(custom = "validate_password_strength")]
pub password: String,
```

## ✅ Correct Syntax

```rust
#[validate(custom(function = "validate_password_strength"))]
pub password: String,
```

## Complete Corrected Code

### In `src/models/user.rs`

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(custom(function = "validate_password_strength"))]
    pub password: String,
}

fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());

    if has_uppercase && has_lowercase && has_digit {
        Ok(())
    } else {
        let mut error = validator::ValidationError::new("password_strength");
        error.message = Some(std::borrow::Cow::from(
            "Password must contain at least one uppercase letter, one lowercase letter, and one digit"
        ));
        Err(error)
    }
}
```

## Key Changes

1. **Validation Attribute**: Changed from `custom = "..."` to `custom(function = "...")`
2. **Error Creation**: Proper `ValidationError` creation with message
3. **Error Message**: User-friendly error message using `Cow` for string ownership

## Alternative: Without Custom Validation

If you want to avoid custom validation complexity, you can use regex instead:

```rust
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(
        length(min = 8),
        regex(path = "PASSWORD_REGEX", message = "Password must contain uppercase, lowercase, and digit")
    )]
    pub password: String,
}

// Add this at the module level
use once_cell::sync::Lazy;
use regex::Regex;

static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).+$").unwrap()
});
```

### If using regex approach, add to Cargo.toml:

```toml
[dependencies]
regex = "1.10"
once_cell = "1.19"
validator = { version = "0.16", features = ["derive"] }
```

## Testing the Validation

### Valid Password

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "Password123"
  }'
```

**Expected**: Success (201 Created)

### Invalid Password - No Uppercase

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }'
```

**Expected**: Error (400 Bad Request) with message about password strength

### Invalid Password - Too Short

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "Pass1"
  }'
```

**Expected**: Error (400 Bad Request) with message about minimum length

## Validation Error Response Format

When validation fails, the API returns:

```json
{
  "error": "Validation Error",
  "message": "One or more validation errors occurred",
  "details": [
    "password: password_strength"
  ]
}
```

## Additional Validation Examples

### Email Validation

```rust
#[validate(email(message = "Please provide a valid email address"))]
pub email: String,
```

### Length Validation

```rust
#[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
pub username: String,
```

### URL Validation

```rust
#[validate(url(message = "Please provide a valid URL"))]
pub avatar_url: Option<String>,
```

### Range Validation

```rust
#[validate(range(min = 18, max = 120, message = "Age must be between 18 and 120"))]
pub age: i32,
```

### Multiple Validations

```rust
#[validate(
    length(min = 8, max = 100),
    custom(function = "validate_password_strength")
)]
pub password: String,
```

## Common Validator Crate Issues

### Issue 1: "custom validation must be a function"

**Cause**: Incorrect syntax for custom validation

**Fix**: Use `custom(function = "function_name")`

### Issue 2: Custom function not found

**Cause**: Function not in scope or misnamed

**Fix**: Ensure function is defined in the same module or imported

### Issue 3: Validation not running

**Cause**: Forgot to call `.validate()` in handler

**Fix**: Always call `dto.validate()?` in handlers

## Best Practices

1. **Always validate input**: Call `.validate()?` on all DTOs
2. **Provide clear messages**: Use custom error messages for better UX
3. **Test validation**: Write tests for both valid and invalid inputs
4. **Document requirements**: List validation rules in API docs
5. **Consistent error format**: Use the custom error handler for uniform responses

## Summary

The validator crate requires the `custom(function = "...")` syntax for custom validation functions. The corrected `src/models/user.rs` file now includes:

- ✅ Correct custom validation syntax
- ✅ Proper error message formatting
- ✅ User-friendly error messages

Make sure to update your `src/models/user.rs` file with the corrected version!