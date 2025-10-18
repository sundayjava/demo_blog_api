use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::repository::like as repository,
    error::AppError,
    middleware::auth::AuthenticatedUser as JwtClaims,
    models::likes::{LikeStatusResponse, LikesQueryParams, PostLikesResponse},
};

/// POST /api/posts/:post_id/like
pub async fn like_post(
    pool: web::Data<PgPool>,
    claims: JwtClaims,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    repository::like_post(&pool, claims.user_id, *post_id).await?;

    let likes_count = repository::get_post_likes_count(&pool, *post_id).await?;

    let response = LikeStatusResponse {
        post_id: *post_id,
        liked: true,
        likes_count,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// DELETE /api/posts/:post_id/like
pub async fn unlike_post(
    pool: web::Data<PgPool>,
    claims: JwtClaims,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    repository::unlike_post(&pool, claims.user_id, *post_id).await?;

    let likes_count = repository::get_post_likes_count(&pool, *post_id).await?;

    let response = LikeStatusResponse {
        post_id: *post_id,
        liked: false,
        likes_count,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/posts/:post_id/toggle-like
pub async fn toggle_like(
    pool: web::Data<PgPool>,
    claims: JwtClaims,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let liked = repository::toggle_like(&pool, claims.user_id, *post_id).await?;
    let likes_count = repository::get_post_likes_count(&pool, *post_id).await?;

    let response = LikeStatusResponse {
        post_id: *post_id,
        liked,
        likes_count,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/posts/:post_id/likes
pub async fn get_post_likes(
    pool: web::Data<PgPool>,
    post_id: web::Path<Uuid>,
    query: web::Query<LikesQueryParams>,
) -> Result<HttpResponse, AppError> {
    let (users, _total) =
        repository::get_post_likes(&pool, *post_id, query.page_size(), query.offset()).await?;

    let likes_count = repository::get_post_likes_count(&pool, *post_id).await?;

    let response = PostLikesResponse {
        post_id: *post_id,
        likes_count,
        users,
    };

    Ok(HttpResponse::Ok().json(response))
}
