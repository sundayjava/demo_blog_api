use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::repository::comment as repository,
    error::AppError,
    middleware::auth::AuthenticatedUser as JwtClaims,
    models::comment::{
        CommentQueryParams, CommentResponse, CommentStatsResponse, CommentsListResponse,
        CreateCommentRequest, UpdateCommentRequest,
    },
};

/// POST /api/posts/:post_id/comments
pub async fn create_comment(
    pool: web::Data<PgPool>,
    claims: JwtClaims,
    post_id: web::Path<Uuid>,
    req: web::Json<CreateCommentRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.validate()?;

    tracing::info!(
        "User {} is creating a comment on post {}",
        claims.user_id,
        *post_id
    );

    // Create comment
    let comment = repository::create_comment(&pool, *post_id, claims.user_id, &req).await?;

    // Get comment with author info
    let comment_with_author = repository::get_comment_by_id(&pool, comment.id)
        .await?
        .ok_or_else(|| AppError::InternalError("Failed to retrieve created comment".to_string()))?;

    let response = comment_with_author.to_response(0);

    Ok(HttpResponse::Created().json(response))
}

/// GET /api/posts/:post_id/comments
pub async fn get_post_comments(
    pool: web::Data<PgPool>,
    post_id: web::Path<Uuid>,
    query: web::Query<CommentQueryParams>,
) -> Result<HttpResponse, AppError> {
    // Get comments with pagination
    let (comments, total) = repository::get_post_comments(&pool, *post_id, &query).await?;

    // Get replies count for each comment
    let comment_ids: Vec<Uuid> = comments.iter().map(|c| c.id).collect();
    let replies_counts = repository::get_replies_counts(&pool, &comment_ids).await?;

    // Create a map for quick lookup
    let replies_map: HashMap<Uuid, i64> = replies_counts
        .into_iter()
        .map(|rc| (rc.comment_id, rc.count))
        .collect();

    // Convert to responses
    let comment_responses: Vec<CommentResponse> = comments
        .into_iter()
        .map(|c| {
            let replies_count = *replies_map.get(&c.id).unwrap_or(&0);
            c.to_response(replies_count)
        })
        .collect();

    let total_pages = (total as f64 / query.page_size() as f64).ceil() as i64;

    let response = CommentsListResponse {
        comments: comment_responses,
        total,
        page: query.page(),
        page_size: query.page_size(),
        total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/comments/:comment_id
pub async fn get_comment(
    pool: web::Data<PgPool>,
    comment_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let comment = repository::get_comment_by_id(&pool, *comment_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    // Get replies count
    let replies_counts = repository::get_replies_counts(&pool, &[comment.id]).await?;
    let replies_count = replies_counts.first().map(|rc| rc.count).unwrap_or(0);

    let response = comment.to_response(replies_count);

    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/comments/:comment_id/replies
pub async fn get_comment_replies(
    pool: web::Data<PgPool>,
    comment_id: web::Path<Uuid>,
    query: web::Query<CommentQueryParams>,
) -> Result<HttpResponse, AppError> {
    // Create params with parent_id set
    let mut params = query.into_inner();
    params.parent_id = Some(*comment_id);

    // Get parent comment to extract post_id
    let parent_comment = repository::get_comment_by_id(&pool, *comment_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Parent comment not found".to_string()))?;

    // Get replies
    let (replies, total) =
        repository::get_post_comments(&pool, parent_comment.post_id, &params).await?;

    // Get replies count for each reply (nested replies)
    let reply_ids: Vec<Uuid> = replies.iter().map(|c| c.id).collect();
    let replies_counts = repository::get_replies_counts(&pool, &reply_ids).await?;

    let replies_map: HashMap<Uuid, i64> = replies_counts
        .into_iter()
        .map(|rc| (rc.comment_id, rc.count))
        .collect();

    let reply_responses: Vec<CommentResponse> = replies
        .into_iter()
        .map(|c| {
            let nested_replies_count = *replies_map.get(&c.id).unwrap_or(&0);
            c.to_response(nested_replies_count)
        })
        .collect();

    let total_pages = (total as f64 / params.page_size() as f64).ceil() as i64;

    let response = CommentsListResponse {
        comments: reply_responses,
        total,
        page: params.page(),
        page_size: params.page_size(),
        total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// PUT /api/comments/:comment_id
pub async fn update_comment(
    pool: web::Data<PgPool>,
    claims: JwtClaims,
    comment_id: web::Path<Uuid>,
    req: web::Json<UpdateCommentRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    req.validate()?;

    // Update comment
    let comment = repository::update_comment(&pool, *comment_id, claims.user_id, &req).await?;

    // Get updated comment with author info
    let comment_with_author = repository::get_comment_by_id(&pool, comment.id)
        .await?
        .ok_or_else(|| AppError::InternalError("Failed to retrieve updated comment".to_string()))?;

    // Get replies count
    let replies_counts = repository::get_replies_counts(&pool, &[comment.id]).await?;
    let replies_count = replies_counts.first().map(|rc| rc.count).unwrap_or(0);

    let response = comment_with_author.to_response(replies_count);

    Ok(HttpResponse::Ok().json(response))
}

/// DELETE /api/comments/:comment_id
pub async fn delete_comment(
    pool: web::Data<PgPool>,
    claims: JwtClaims,
    comment_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    repository::delete_comment(&pool, *comment_id, claims.user_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// GET /api/posts/:post_id/comments/stats
pub async fn get_comment_stats(
    pool: web::Data<PgPool>,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let total_comments = repository::get_post_comments_count(&pool, *post_id).await?;
    let top_level_comments = repository::get_top_level_comments_count(&pool, *post_id).await?;

    let response = CommentStatsResponse {
        post_id: *post_id,
        total_comments,
        top_level_comments,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/users/:user_id/comments
pub async fn get_user_comments(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    query: web::Query<CommentQueryParams>,
) -> Result<HttpResponse, AppError> {
    let (comments, total) =
        repository::get_user_comments(&pool, *user_id, query.page_size(), query.offset()).await?;

    // Get replies count for each comment
    let comment_ids: Vec<Uuid> = comments.iter().map(|c| c.id).collect();
    let replies_counts = repository::get_replies_counts(&pool, &comment_ids).await?;

    let replies_map: HashMap<Uuid, i64> = replies_counts
        .into_iter()
        .map(|rc| (rc.comment_id, rc.count))
        .collect();

    let comment_responses: Vec<CommentResponse> = comments
        .into_iter()
        .map(|c| {
            let replies_count = *replies_map.get(&c.id).unwrap_or(&0);
            c.to_response(replies_count)
        })
        .collect();

    let total_pages = (total as f64 / query.page_size() as f64).ceil() as i64;

    let response = CommentsListResponse {
        comments: comment_responses,
        total,
        page: query.page(),
        page_size: query.page_size(),
        total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}
