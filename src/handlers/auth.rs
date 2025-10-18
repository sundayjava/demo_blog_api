use crate::db::repository::user as repository;
use crate::error::{AppError, AppResult};
use crate::models::user::{AuthResponse, CreateUserDto, LoginDto, UserResponse};
use crate::services::{jwt::JwtService, password::PasswordService};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use std::sync::Arc;
use validator::Validate;

#[tracing::instrument(skip(pool, jwt_service, dto))]
pub async fn register(
    pool: web::Data<PgPool>,
    jwt_service: web::Data<Arc<JwtService>>,
    dto: web::Json<CreateUserDto>,
) -> AppResult<HttpResponse> {
    // Validate input
    dto.validate()?;

    tracing::info!("Attempting to register user with email: {}", dto.email);

    // Check if username exists
    if repository::get_user_by_username(&pool, &dto.username)
        .await?
        .is_some()
    {
        tracing::warn!(
            "Registration failed: username '{}' already taken",
            dto.username
        );
        return Err(AppError::Conflict("Username already taken".to_string()));
    }

    // Check if email exists
    if repository::get_user_by_email(&pool, &dto.email)
        .await?
        .is_some()
    {
        tracing::warn!(
            "Registration failed: email '{}' already registered",
            dto.email
        );
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Hash password
    let password_hash = PasswordService::hash_password(&dto.password)?;

    // Create user
    let user = repository::create_user(&pool, dto.into_inner(), password_hash).await?;

    // Generate JWT token
    let token = jwt_service.generate_token(user.id, user.email.clone(), user.role.clone())?;

    tracing::info!("User registered successfully: {}", user.id);

    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user: UserResponse::from(user),
    }))
}

#[tracing::instrument(skip(pool, jwt_service, dto))]
pub async fn login(
    pool: web::Data<PgPool>,
    jwt_service: web::Data<Arc<JwtService>>,
    dto: web::Json<LoginDto>,
) -> AppResult<HttpResponse> {
    // Validate input
    dto.validate()?;

    tracing::info!("Login attempt for email: {}", dto.email);

    // Get user by email
    let user = repository::get_user_by_email(&pool, &dto.email)
        .await?
        .ok_or_else(|| {
            tracing::warn!("Login failed: user not found for email {}", dto.email);
            AppError::Unauthorized("Invalid email or password".to_string())
        })?;

    // Verify password
    let is_valid = PasswordService::verify_password(&dto.password, &user.password_hash)?;

    if !is_valid {
        tracing::warn!("Login failed: invalid password for user {}", user.id);
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    // Generate JWT token
    let token = jwt_service.generate_token(user.id, user.email.clone(), user.role.clone())?;

    tracing::info!("User logged in successfully: {}", user.id);

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse::from(user),
    }))
}
