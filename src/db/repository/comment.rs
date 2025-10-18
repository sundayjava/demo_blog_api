use crate::error::AppError;
use crate::models::comment::{
    Comment, CommentCount, CommentQueryParams, CommentWithAuthor, CreateCommentRequest,
    UpdateCommentRequest,
};
use sqlx::{PgPool};
use uuid::Uuid;

/// Create a new comment
pub async fn create_comment(
    pool: &PgPool,
    post_id: Uuid,
    user_id: Uuid,
    req: &CreateCommentRequest,
) -> Result<Comment, AppError> {
    // Verify post exists
    let post_exists =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
            .bind(post_id)
            .fetch_one(pool)
            .await
            .map_err(AppError::DatabaseError)?;

    if !post_exists {
        return Err(AppError::NotFound("Post not found".to_string()));
    }

    // If parent_comment_id is provided, verify it exists and belongs to same post
    if let Some(parent_id) = req.parent_comment_id {
        let parent_valid = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM comments WHERE id = $1 AND post_id = $2)",
        )
        .bind(parent_id)
        .bind(post_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

        if !parent_valid {
            return Err(AppError::BadRequest(
                "Parent comment not found or doesn't belong to this post".to_string(),
            ));
        }
    }

    let comment = sqlx::query_as::<_, Comment>(
        r#"
        INSERT INTO comments (post_id, user_id, content, parent_comment_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, post_id, user_id, content, parent_comment_id, created_at, updated_at
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .bind(&req.content)
    .bind(req.parent_comment_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(comment)
}

/// Get comment by ID with author info
pub async fn get_comment_by_id(
    pool: &PgPool,
    comment_id: Uuid,
) -> Result<Option<CommentWithAuthor>, AppError> {
    let comment = sqlx::query_as::<_, CommentWithAuthor>(
        r#"
        SELECT 
            c.id, c.post_id, c.user_id, c.content, c.parent_comment_id,
            c.created_at, c.updated_at,
            u.id as author_id,
            u.username as author_username,
            u.avatar_url as author_avatar_url
        FROM comments c
        INNER JOIN users u ON c.user_id = u.id
        WHERE c.id = $1
        "#,
    )
    .bind(comment_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(comment)
}

/// Get comments for a post with pagination
pub async fn get_post_comments(
    pool: &PgPool,
    post_id: Uuid,
    params: &CommentQueryParams,
) -> Result<(Vec<CommentWithAuthor>, i64), AppError> {
    // Build the WHERE clause based on parent_id filter
    let (where_clause, bind_count) = if params.parent_id.is_some() {
        ("WHERE c.post_id = $1 AND c.parent_comment_id = $2", 2)
    } else {
        ("WHERE c.post_id = $1 AND c.parent_comment_id IS NULL", 1)
    };

    // Get total count
    let count_query = format!("SELECT COUNT(*) FROM comments c {}", where_clause);

    let total = if let Some(parent_id) = params.parent_id {
        sqlx::query_scalar(&count_query)
            .bind(post_id)
            .bind(parent_id)
            .fetch_one(pool)
            .await
    } else {
        sqlx::query_scalar(&count_query)
            .bind(post_id)
            .fetch_one(pool)
            .await
    }
    .map_err(AppError::DatabaseError)?;

    // Get comments with author info
    let query = format!(
        r#"
        SELECT 
            c.id, c.post_id, c.user_id, c.content, c.parent_comment_id,
            c.created_at, c.updated_at,
            u.id as author_id,
            u.username as author_username,
            u.avatar_url as author_avatar_url
        FROM comments c
        INNER JOIN users u ON c.user_id = u.id
        {}
        ORDER BY c.created_at DESC
        LIMIT ${} OFFSET ${}
        "#,
        where_clause,
        bind_count + 1,
        bind_count + 2
    );

    let comments = if let Some(parent_id) = params.parent_id {
        sqlx::query_as::<_, CommentWithAuthor>(&query)
            .bind(post_id)
            .bind(parent_id)
            .bind(params.page_size())
            .bind(params.offset())
            .fetch_all(pool)
            .await
    } else {
        sqlx::query_as::<_, CommentWithAuthor>(&query)
            .bind(post_id)
            .bind(params.page_size())
            .bind(params.offset())
            .fetch_all(pool)
            .await
    }
    .map_err(AppError::DatabaseError)?;

    Ok((comments, total))
}

/// Get replies count for multiple comments
pub async fn get_replies_counts(
    pool: &PgPool,
    comment_ids: &[Uuid],
) -> Result<Vec<CommentCount>, AppError> {
    let counts = sqlx::query_as::<_, CommentCount>(
        r#"
        SELECT parent_comment_id as comment_id, COUNT(*) as count
        FROM comments
        WHERE parent_comment_id = ANY($1)
        GROUP BY parent_comment_id
        "#,
    )
    .bind(comment_ids)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(counts)
}

/// Get comment count for a post
pub async fn get_post_comments_count(pool: &PgPool, post_id: Uuid) -> Result<i64, AppError> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM comments WHERE post_id = $1")
        .bind(post_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(count)
}

/// Get top-level comments count (comments without parent)
pub async fn get_top_level_comments_count(pool: &PgPool, post_id: Uuid) -> Result<i64, AppError> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM comments WHERE post_id = $1 AND parent_comment_id IS NULL",
    )
    .bind(post_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(count)
}

/// Update a comment
pub async fn update_comment(
    pool: &PgPool,
    comment_id: Uuid,
    user_id: Uuid,
    req: &UpdateCommentRequest,
) -> Result<Comment, AppError> {
    let comment = sqlx::query_as::<_, Comment>(
        r#"
        UPDATE comments
        SET content = $1, updated_at = NOW()
        WHERE id = $2 AND user_id = $3
        RETURNING id, post_id, user_id, content, parent_comment_id, created_at, updated_at
        "#,
    )
    .bind(&req.content)
    .bind(comment_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or_else(|| {
        AppError::NotFound("Comment not found or you don't have permission".to_string())
    })?;

    Ok(comment)
}

/// Delete a comment (and its replies due to CASCADE)
pub async fn delete_comment(
    pool: &PgPool,
    comment_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM comments WHERE id = $1 AND user_id = $2")
        .bind(comment_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Comment not found or you don't have permission".to_string(),
        ));
    }

    Ok(())
}

/// Get comments by user with pagination
pub async fn get_user_comments(
    pool: &PgPool,
    user_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<(Vec<CommentWithAuthor>, i64), AppError> {
    // Get total count
    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM comments WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    // Get comments
    let comments = sqlx::query_as::<_, CommentWithAuthor>(
        r#"
        SELECT 
            c.id, c.post_id, c.user_id, c.content, c.parent_comment_id,
            c.created_at, c.updated_at,
            u.id as author_id,
            u.username as author_username,
            u.avatar_url as author_avatar_url
        FROM comments c
        INNER JOIN users u ON c.user_id = u.id
        WHERE c.user_id = $1
        ORDER BY c.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok((comments, total))
}

// Get comment counts for multiple posts
// pub async fn get_posts_comments_count(
//     pool: &PgPool,
//     post_ids: &[Uuid],
// ) -> Result<Vec<CommentCount>, AppError> {
//     #[derive(FromRow)]
//     struct PostCommentCount {
//         post_id: Uuid,
//         count: i64,
//     }

//     let counts = sqlx::query_as::<_, PostCommentCount>(
//         r#"
//         SELECT post_id, COUNT(*) as count
//         FROM comments
//         WHERE post_id = ANY($1)
//         GROUP BY post_id
//         "#,
//     )
//     .bind(post_ids)
//     .fetch_all(pool)
//     .await
//     .map_err(AppError::DatabaseError)?;

//     Ok(counts
//         .into_iter()
//         .map(|c| CommentCount {
//             comment_id: c.post_id,
//             count: c.count,
//         })
//         .collect())
// }
