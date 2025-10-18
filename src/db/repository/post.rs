use crate::error::AppError;
use crate::models::post::{
    AuthorInfo, CreatePostRequest, Post, PostQueryParams, PostWithStatsResponse, UpdatePostRequest,
};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

/// Create a new post
pub async fn create_post(
    pool: &PgPool,
    user_id: Uuid,
    dto: &CreatePostRequest,
) -> Result<Post, AppError> {
    let status = dto.status.as_deref().unwrap_or("draft");

    let post = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (id, user_id, title, content, status, featured_image_url, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&dto.title)
    .bind(&dto.content)
    .bind(status)
    .bind(&dto.featured_image_url)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    Ok(post)
}

/// Get post by ID
pub async fn get_post_by_id(pool: &PgPool, post_id: Uuid) -> Result<Option<Post>, AppError> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(post_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    Ok(post)
}

/// Get post by slug
pub async fn get_post_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Post>, AppError> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE slug = $1")
        .bind(slug)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    Ok(post)
}

/// Get author info for a user
pub async fn get_author_info(pool: &PgPool, user_id: Uuid) -> Result<AuthorInfo, AppError> {
    let author = sqlx::query_as::<_, AuthorInfo>(
        r#"
        SELECT id, username, avatar_url
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    Ok(author)
}

pub async fn get_posts(
    pool: &PgPool,
    params: &PostQueryParams,
) -> Result<(Vec<Post>, i64), AppError> {
    // PRE-CALCULATE all patterns that need to live long enough
    let search_pattern = params.search.as_ref().map(|s| format!("%{}%", s));
    let default_status = "published".to_string();
    let status = params.status.as_ref().unwrap_or(&default_status);

    // Build dynamic query based on filters
    let mut conditions = vec!["1=1".to_string()];
    let mut bind_count = 0;

    // Filter by status
    bind_count += 1;
    conditions.push(format!("status = ${}", bind_count));

    // Filter by user_id
    if params.user_id.is_some() {
        bind_count += 1;
        conditions.push(format!("user_id = ${}", bind_count));
    }

    // Search in title and content
    if search_pattern.is_some() {
        bind_count += 1;
        conditions.push(format!(
            "(title ILIKE ${} OR content ILIKE ${})",
            bind_count, bind_count
        ));
    }

    let where_clause = conditions.join(" AND ");

    // Get total count
    let count_query = format!("SELECT COUNT(*) FROM posts WHERE {}", where_clause);

    let mut count_query_builder = sqlx::query_scalar::<_, i64>(&count_query);

    // Bind status
    count_query_builder = count_query_builder.bind(status);

    // Bind user_id
    if let Some(user_id) = params.user_id {
        count_query_builder = count_query_builder.bind(user_id);
    }

    // Bind search pattern - NOW IT LIVES LONG ENOUGH!
    if let Some(ref pattern) = search_pattern {
        count_query_builder = count_query_builder.bind(pattern);
    }

    let total = count_query_builder
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    // Get posts
    let posts_query = format!(
        r#"
        SELECT id, user_id, title, content, slug, status, featured_image_url,
               views_count, created_at, updated_at, published_at
        FROM posts
        WHERE {}
        ORDER BY created_at DESC
        LIMIT ${} OFFSET ${}
        "#,
        where_clause,
        bind_count + 1,
        bind_count + 2
    );

    let mut posts_query_builder = sqlx::query_as::<_, Post>(&posts_query);

    // Bind status
    posts_query_builder = posts_query_builder.bind(status);

    // Bind user_id
    if let Some(user_id) = params.user_id {
        posts_query_builder = posts_query_builder.bind(user_id);
    }

    // Bind search pattern
    if let Some(ref pattern) = search_pattern {
        posts_query_builder = posts_query_builder.bind(pattern);
    }

    // Bind pagination
    posts_query_builder = posts_query_builder
        .bind(params.page_size())
        .bind(params.offset());

    let posts = posts_query_builder
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    Ok((posts, total))
}

/// Get posts with stats (likes, comments, user interaction)
pub async fn get_posts_with_stats(
    pool: &PgPool,
    params: &PostQueryParams,
    current_user_id: Option<Uuid>,
) -> Result<(Vec<PostWithStatsResponse>, i64), AppError> {
    // Get posts
    let (posts, total) = get_posts(pool, params).await?;

    if posts.is_empty() {
        return Ok((vec![], total));
    }

    let post_ids: Vec<Uuid> = posts.iter().map(|p| p.id).collect();

    // Get likes count for all posts
    let likes_counts = get_posts_likes_count(pool, &post_ids).await?;
    let likes_map: HashMap<Uuid, i64> = likes_counts
        .into_iter()
        .map(|lc| (lc.post_id, lc.count))
        .collect();

    // Get comments count for all posts
    let comments_counts = get_posts_comments_count(pool, &post_ids).await?;
    let comments_map: HashMap<Uuid, i64> = comments_counts
        .into_iter()
        .map(|cc| (cc.post_id, cc.count))
        .collect();

    // Get user likes status if authenticated
    let user_likes: Vec<Uuid> = if let Some(user_id) = current_user_id {
        get_user_likes_for_posts(pool, user_id, &post_ids).await?
    } else {
        vec![]
    };

    // Get user bookmarks if authenticated
    // let user_bookmarks: Vec<Uuid> = if let Some(user_id) = current_user_id {
    //     get_user_bookmarks_for_posts(pool, user_id, &post_ids).await?
    // } else {
    //     vec![]
    // };

    // Get author info for all posts
    let author_ids: Vec<Uuid> = posts.iter().map(|p| p.user_id).collect();
    let authors = get_authors_info(pool, &author_ids).await?;
    let authors_map: HashMap<Uuid, AuthorInfo> = authors.into_iter().map(|a| (a.id, a)).collect();

    // Combine everything
    let posts_with_stats: Vec<PostWithStatsResponse> = posts
        .into_iter()
        .map(|post| {
            let likes_count = *likes_map.get(&post.id).unwrap_or(&0);
            let comments_count = *comments_map.get(&post.id).unwrap_or(&0);
            let user_liked = user_likes.contains(&post.id);
            // let user_bookmarked = user_bookmarks.contains(&post.id);

            let author = authors_map
                .get(&post.user_id)
                .cloned()
                .unwrap_or_else(|| AuthorInfo {
                    id: post.user_id,
                    username: "Unknown".to_string(),
                    avatar_url: None,
                });

            let post_response = post.to_response(author);
            post_response.with_stats(likes_count, comments_count, user_liked)
        })
        .collect();

    Ok((posts_with_stats, total))
}

/// Get single post with stats
pub async fn get_post_with_stats(
    pool: &PgPool,
    post_id: Uuid,
    current_user_id: Option<Uuid>,
) -> Result<Option<PostWithStatsResponse>, AppError> {
    let post = get_post_by_id(pool, post_id).await?;

    if let Some(post) = post {
        let likes_count = get_post_likes_count(pool, post_id).await?;
        let comments_count = get_post_comments_count(pool, post_id).await?;

        let user_liked = if let Some(user_id) = current_user_id {
            has_user_liked_post(pool, user_id, post_id).await?
        } else {
            false
        };

        // let user_bookmarked = if let Some(user_id) = current_user_id {
        //     has_user_bookmarked_post(pool, user_id, post_id).await?
        // } else {
        //     false
        // };

        let author = get_author_info(pool, post.user_id).await?;
        let post_response = post.to_response(author);
        let post_with_stats = post_response.with_stats(
            likes_count,
            comments_count,
            user_liked,
            // user_bookmarked,
        );

        Ok(Some(post_with_stats))
    } else {
        Ok(None)
    }
}

/// Update post
pub async fn update_post(
    pool: &PgPool,
    post_id: Uuid,
    user_id: Uuid,
    req: &UpdatePostRequest,
) -> Result<Post, AppError> {
    // Build dynamic update query
    let mut updates = vec![];
    let mut bind_count = 0;

    if req.title.is_some() {
        bind_count += 1;
        updates.push(format!("title = ${}", bind_count));
    }
    if req.content.is_some() {
        bind_count += 1;
        updates.push(format!("content = ${}", bind_count));
    }
    if req.status.is_some() {
        bind_count += 1;
        updates.push(format!("status = ${}", bind_count));
    }
    if req.featured_image_url.is_some() {
        bind_count += 1;
        updates.push(format!("featured_image_url = ${}", bind_count));
    }

    if updates.is_empty() {
        return Err(AppError::BadRequest("No fields to update".to_string()));
    }

    updates.push("updated_at = NOW()".to_string());
    let set_clause = updates.join(", ");

    let query = format!(
        r#"
        UPDATE posts
        SET {}
        WHERE id = ${} AND user_id = ${}
        RETURNING id, user_id, title, content, slug, status, featured_image_url,
                  views_count, created_at, updated_at, published_at
        "#,
        set_clause,
        bind_count + 1,
        bind_count + 2
    );

    let mut query_builder = sqlx::query_as::<_, Post>(&query);

    if let Some(ref title) = req.title {
        query_builder = query_builder.bind(title);
    }
    if let Some(ref content) = req.content {
        query_builder = query_builder.bind(content);
    }
    if let Some(ref status) = req.status {
        query_builder = query_builder.bind(status);
    }
    if let Some(ref featured_image_url) = req.featured_image_url {
        query_builder = query_builder.bind(featured_image_url);
    }

    query_builder = query_builder.bind(post_id).bind(user_id);

    let post = query_builder
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?
        .ok_or_else(|| {
            AppError::NotFound("Post not found or you don't have permission".to_string())
        })?;

    Ok(post)
}

/// Delete post
pub async fn delete_post(pool: &PgPool, post_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1 AND user_id = $2")
        .bind(post_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Post not found or you don't have permission".to_string(),
        ));
    }

    Ok(())
}

/// Increment post views
pub async fn increment_views(pool: &PgPool, post_id: Uuid) -> Result<(), AppError> {
    sqlx::query("UPDATE posts SET views_count = views_count + 1 WHERE id = $1")
        .bind(post_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    Ok(())
}

/// Publish post
pub async fn publish_post(pool: &PgPool, post_id: Uuid, user_id: Uuid) -> Result<Post, AppError> {
    let post = sqlx::query_as::<_, Post>(
        r#"
        UPDATE posts
        SET status = 'published',
            published_at = COALESCE(published_at, NOW()),
            updated_at = NOW()
        WHERE id = $1 AND user_id = $2
        RETURNING id, user_id, title, content, slug, status, featured_image_url,
                  views_count, created_at, updated_at, published_at
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?
    .ok_or_else(|| AppError::NotFound("Post not found or you don't have permission".to_string()))?;

    Ok(post)
}

/// Archive post
pub async fn archive_post(pool: &PgPool, post_id: Uuid, user_id: Uuid) -> Result<Post, AppError> {
    let post = sqlx::query_as::<_, Post>(
        r#"
        UPDATE posts
        SET status = 'archived', updated_at = NOW()
        WHERE id = $1 AND user_id = $2
        RETURNING id, user_id, title, content, slug, status, featured_image_url,
                  views_count, created_at, updated_at, published_at
        "#,
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?
    .ok_or_else(|| AppError::NotFound("Post not found or you don't have permission".to_string()))?;

    Ok(post)
}

// ====================================
// HELPER FUNCTIONS
// ====================================

#[derive(Debug, sqlx::FromRow)]
struct PostLikeCount {
    post_id: Uuid,
    count: i64,
}

async fn get_posts_likes_count(
    pool: &PgPool,
    post_ids: &[Uuid],
) -> Result<Vec<PostLikeCount>, AppError> {
    let counts = sqlx::query_as::<_, PostLikeCount>(
        r#"
        SELECT post_id, COUNT(*) as count
        FROM likes
        WHERE post_id = ANY($1)
        GROUP BY post_id
        "#,
    )
    .bind(post_ids)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    Ok(counts)
}

async fn get_post_likes_count(pool: &PgPool, post_id: Uuid) -> Result<i64, AppError> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM likes WHERE post_id = $1")
        .bind(post_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    Ok(count)
}

#[derive(Debug, sqlx::FromRow)]
struct PostCommentCount {
    post_id: Uuid,
    count: i64,
}

async fn get_posts_comments_count(
    pool: &PgPool,
    post_ids: &[Uuid],
) -> Result<Vec<PostCommentCount>, AppError> {
    let counts = sqlx::query_as::<_, PostCommentCount>(
        r#"
        SELECT post_id, COUNT(*) as count
        FROM comments
        WHERE post_id = ANY($1)
        GROUP BY post_id
        "#,
    )
    .bind(post_ids)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    Ok(counts)
}

async fn get_post_comments_count(pool: &PgPool, post_id: Uuid) -> Result<i64, AppError> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM comments WHERE post_id = $1")
        .bind(post_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    Ok(count)
}

async fn get_user_likes_for_posts(
    pool: &PgPool,
    user_id: Uuid,
    post_ids: &[Uuid],
) -> Result<Vec<Uuid>, AppError> {
    let liked_post_ids = sqlx::query_scalar::<_, Uuid>(
        "SELECT post_id FROM likes WHERE user_id = $1 AND post_id = ANY($2)",
    )
    .bind(user_id)
    .bind(post_ids)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    Ok(liked_post_ids)
}

async fn has_user_liked_post(
    pool: &PgPool,
    user_id: Uuid,
    post_id: Uuid,
) -> Result<bool, AppError> {
    let liked = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM likes WHERE user_id = $1 AND post_id = $2)",
    )
    .bind(user_id)
    .bind(post_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    Ok(liked)
}

async fn get_user_bookmarks_for_posts(
    pool: &PgPool,
    user_id: Uuid,
    post_ids: &[Uuid],
) -> Result<Vec<Uuid>, AppError> {
    let bookmarked_post_ids = sqlx::query_scalar::<_, Uuid>(
        "SELECT post_id FROM bookmarks WHERE user_id = $1 AND post_id = ANY($2)",
    )
    .bind(user_id)
    .bind(post_ids)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    Ok(bookmarked_post_ids)
}

async fn has_user_bookmarked_post(
    pool: &PgPool,
    user_id: Uuid,
    post_id: Uuid,
) -> Result<bool, AppError> {
    let bookmarked = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM bookmarks WHERE user_id = $1 AND post_id = $2)",
    )
    .bind(user_id)
    .bind(post_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    Ok(bookmarked)
}

async fn get_authors_info(pool: &PgPool, user_ids: &[Uuid]) -> Result<Vec<AuthorInfo>, AppError> {
    let authors = sqlx::query_as::<_, AuthorInfo>(
        "SELECT id, username, avatar_url FROM users WHERE id = ANY($1)",
    )
    .bind(user_ids)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    Ok(authors)
}
