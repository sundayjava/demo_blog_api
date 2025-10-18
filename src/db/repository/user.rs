use crate::error::AppResult;
use crate::models::common::{PaginatedResponse, PaginationParams};
use crate::models::user::{CreateUserDto, UpdateUserDto, User};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(
    pool: &PgPool,
    dto: CreateUserDto,
    password_hash: String,
) -> AppResult<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'user', NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(dto.username)
    .bind(dto.email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn get_user_by_username(pool: &PgPool, username: &str) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn list_users(
    pool: &PgPool,
    pagination: PaginationParams,
) -> AppResult<PaginatedResponse<User>> {
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(pagination.limit)
    .bind(pagination.offset())
    .fetch_all(pool)
    .await?;

    Ok(PaginatedResponse::new(
        users,
        pagination.page,
        pagination.limit,
        total,
    ))
}

pub async fn update_user(pool: &PgPool, id: Uuid, dto: UpdateUserDto) -> AppResult<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users 
        SET 
            username = COALESCE($1, username),
            email = COALESCE($2, email),
            bio = COALESCE($3, bio),
            avatar_url = COALESCE($4, avatar_url),
            updated_at = NOW()
        WHERE id = $5
        RETURNING *
        "#,
    )
    .bind(dto.username)
    .bind(dto.email)
    .bind(dto.bio)
    .bind(dto.avatar_url)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> AppResult<()> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_user_role(pool: &PgPool, id: Uuid, role: &str) -> AppResult<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users 
        SET role = $1, updated_at = NOW()
        WHERE id = $2
        RETURNING *
        "#,
    )
    .bind(role)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}
