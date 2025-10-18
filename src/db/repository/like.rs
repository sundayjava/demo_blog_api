use crate::error::AppError;
use crate::models::likes::{Like, UserLikeInfo};
use sqlx::PgPool;
use uuid::Uuid;

/// Like a post
pub async fn like_post(pool: &PgPool, user_id: Uuid, post_id: Uuid) -> Result<Like, AppError> {
    let like = sqlx::query_as::<_, Like>(
        r#"
        INSERT INTO likes (user_id, post_id)
        VALUES ($1, $2)
        RETURNING id, user_id, post_id, created_at
        "#,
    )
    .bind(user_id)
    .bind(post_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e
            && db_err.is_unique_violation()
        {
            return AppError::BadRequest("You have already liked this post".to_string());
        }
        AppError::DatabaseError(e)
    })?;

    Ok(like)
}

/// Unlike a post
pub async fn unlike_post(pool: &PgPool, user_id: Uuid, post_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM likes
        WHERE user_id = $1 AND post_id = $2
        "#,
    )
    .bind(user_id)
    .bind(post_id)
    .execute(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Like not found".to_string()));
    }

    Ok(())
}

/// Check if user liked a post
pub async fn has_user_liked_post(
    pool: &PgPool,
    user_id: Uuid,
    post_id: Uuid,
) -> Result<bool, AppError> {
    let result = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM likes
            WHERE user_id = $1 AND post_id = $2
        )
        "#,
    )
    .bind(user_id)
    .bind(post_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(result)
}

/// Get like count for a post
pub async fn get_post_likes_count(pool: &PgPool, post_id: Uuid) -> Result<i64, AppError> {
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) FROM likes
        WHERE post_id = $1
        "#,
    )
    .bind(post_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(count)
}

/// Get users who liked a post
pub async fn get_post_likes(
    pool: &PgPool,
    post_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<(Vec<UserLikeInfo>, i64), AppError> {
    // Get total count
    let total = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) FROM likes
        WHERE post_id = $1
        "#,
    )
    .bind(post_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    // Get users with pagination
    let users = sqlx::query_as::<_, UserLikeInfo>(
        r#"
        SELECT 
            u.id,
            u.username,
            u.avatar_url,
            l.created_at as liked_at
        FROM likes l
        INNER JOIN users u ON l.user_id = u.id
        WHERE l.post_id = $1
        ORDER BY l.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(post_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok((users, total))
}

/// Toggle like (like if not liked, unlike if already liked)
pub async fn toggle_like(pool: &PgPool, user_id: Uuid, post_id: Uuid) -> Result<bool, AppError> {
    let liked = has_user_liked_post(pool, user_id, post_id).await?;

    if liked {
        unlike_post(pool, user_id, post_id).await?;
        Ok(false)
    } else {
        like_post(pool, user_id, post_id).await?;
        Ok(true)
    }
}
