use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::repository::user as repository,
    error::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
    models::{PaginationParams, UpdateUserDto, UserResponse},
};

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRoleDto {
    #[validate(length(min = 1, message = "Role is required"))]
    pub role: String,
}

#[tracing::instrument(skip(pool))]
pub async fn list_users(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    mut pagination: web::Query<PaginationParams>,
) -> AppResult<HttpResponse> {
    // Only admins and moderators can list all users
    if !user.is_moderator_or_above() {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }

    pagination.validate();

    tracing::info!(
        "Listing users with pagination: page={}, limit={}",
        pagination.page,
        pagination.limit
    );

    let result = repository::list_users(&pool, pagination.into_inner()).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[tracing::instrument(skip(pool))]
pub async fn get_current_user(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    tracing::info!("Getting current user: {}", user.user_id);

    let user_data = repository::get_user_by_id(&pool, user.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user_data)))
}

#[tracing::instrument(skip(pool))]
pub async fn get_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    // Anyone can view user profiles (even unauthenticated in this version)
    // If you want to require auth, add `user: AuthenticatedUser` parameter
    tracing::info!("Getting user: {}", user_id);

    let user_data = repository::get_user_by_id(&pool, *user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user_data)))
}

#[tracing::instrument(skip(pool, dto))]
pub async fn update_current_user(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    dto: web::Json<UpdateUserDto>,
) -> AppResult<HttpResponse> {
    dto.validate()?;

    tracing::info!("Updating current user: {}", user.user_id);

    // Check if new username is taken (if provided)
    if let Some(ref username) = dto.username {
        if let Some(existing_user) = repository::get_user_by_username(&pool, username).await? {
            if existing_user.id != user.user_id {
                return Err(AppError::Conflict("Username already taken".to_string()));
            }
        }
    }

    // Check if new email is taken (if provided)
    if let Some(ref email) = dto.email {
        if let Some(existing_user) = repository::get_user_by_email(&pool, email).await? {
            if existing_user.id != user.user_id {
                return Err(AppError::Conflict("Email already registered".to_string()));
            }
        }
    }

    let updated_user = repository::update_user(&pool, user.user_id, dto.into_inner()).await?;

    tracing::info!("User updated successfully: {}", user.user_id);

    Ok(HttpResponse::Ok().json(UserResponse::from(updated_user)))
}

#[tracing::instrument(skip(pool, dto))]
pub async fn update_user(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    user_id: web::Path<Uuid>,
    dto: web::Json<UpdateUserDto>,
) -> AppResult<HttpResponse> {
    // Only admins can update other users
    if !user.is_admin() {
        return Err(AppError::Forbidden(
            "Only admins can update other users".to_string(),
        ));
    }

    dto.validate()?;

    tracing::info!("Admin {} updating user: {}", user.user_id, user_id);

    // Check if new username is taken (if provided)
    if let Some(ref username) = dto.username {
        if let Some(existing_user) = repository::get_user_by_username(&pool, username).await? {
            if existing_user.id != *user_id {
                return Err(AppError::Conflict("Username already taken".to_string()));
            }
        }
    }

    // Check if new email is taken (if provided)
    if let Some(ref email) = dto.email {
        if let Some(existing_user) = repository::get_user_by_email(&pool, email).await? {
            if existing_user.id != *user_id {
                return Err(AppError::Conflict("Email already registered".to_string()));
            }
        }
    }

    let updated_user = repository::update_user(&pool, *user_id, dto.into_inner()).await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(updated_user)))
}

#[tracing::instrument(skip(pool))]
pub async fn delete_user(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    user_id: web::Path<Uuid>,
) -> AppResult<HttpResponse> {
    // Users can delete themselves, or admins can delete anyone
    if user.user_id != *user_id && !user.is_admin() {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }

    tracing::info!("Deleting user: {}", user_id);

    repository::delete_user(&pool, *user_id).await?;

    tracing::info!("User deleted successfully: {}", user_id);

    Ok(HttpResponse::NoContent().finish())
}

#[tracing::instrument(skip(pool))]
pub async fn update_user_role(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    user_id: web::Path<Uuid>,
    dto: web::Json<UpdateRoleDto>,
) -> AppResult<HttpResponse> {
    // Only admins can change roles
    if !user.is_admin() {
        return Err(AppError::Forbidden(
            "Only admins can change user roles".to_string(),
        ));
    }

    dto.validate()?;

    // Validate role value
    let new_role = match dto.role.to_lowercase().as_str() {
        "admin" | "moderator" | "user" => dto.role.to_lowercase(),
        _ => {
            return Err(AppError::BadRequest(
                "Invalid role. Must be: admin, moderator, or user".to_string(),
            ));
        }
    };

    // Prevent users from demoting themselves
    if user.user_id == *user_id && new_role != "admin" {
        return Err(AppError::BadRequest(
            "You cannot change your own admin role".to_string(),
        ));
    }

    tracing::info!(
        "Admin {} updating role for user {} to {}",
        user.user_id,
        user_id,
        new_role
    );

    let updated_user = repository::update_user_role(&pool, *user_id, &new_role).await?;

    tracing::info!("User role updated successfully: {}", user_id);

    Ok(HttpResponse::Ok().json(UserResponse::from(updated_user)))
}
