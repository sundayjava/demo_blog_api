use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    db::repository::post as repository,
    error::AppError,
    middleware::auth::AuthenticatedUser,
    models::post::{CreatePostRequest, PostListResponse, PostQueryParams},
};

pub async fn create_post(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    dto: web::Json<CreatePostRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    dto.validate()?;

    // Create post
    let post = repository::create_post(&pool, user.user_id, &dto).await?;

    // Get author info
    let author = repository::get_author_info(&pool, post.user_id).await?;

    // Convert to response
    let response = post.to_response(author);

    Ok(HttpResponse::Created().json(response))
}

pub async fn get_posts(
    pool: web::Data<PgPool>,
    query: web::Query<PostQueryParams>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Get posts with stats
    let (posts, total) =
        repository::get_posts_with_stats(&pool, &query, Some(user.user_id)).await?;

    let total_pages = (total as f64 / query.page_size() as f64).ceil() as i64;

    let response = PostListResponse {
        posts,
        total,
        page: query.page(),
        page_size: query.page_size(),
        total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}
