use crate::handlers::{auth, comment, like, post, user};
use crate::middleware::auth::JwtAuth;
use crate::services::jwt::JwtService;
use actix_web::web;
use std::sync::Arc;

pub fn configure_routes(cfg: &mut web::ServiceConfig, jwt_service: Arc<JwtService>) {
    // Health check
    cfg.route("/health", web::get().to(health_check));

    // Public routes - no authentication required
    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(auth::register))
            .route("/login", web::post().to(auth::login)),
    );

    cfg.service(
        web::scope("/api")
            .wrap(JwtAuth::new(jwt_service))
            //User routes
            .service(
                web::scope("/users")
                    .route("", web::get().to(user::list_users))
                    .route("/me", web::get().to(user::get_current_user))
                    .route("/me", web::put().to(user::update_current_user))
                    .route("/{id}", web::get().to(user::get_user))
                    .route("/{id}/comments", web::get().to(comment::get_user_comments))
                    .route("/{id}", web::put().to(user::update_user))
                    .route("/{id}", web::delete().to(user::delete_user))
                    .route("/{id}/role", web::put().to(user::update_user_role)),
            )
            // Post routes
            .service(
                web::scope("/posts")
                    .route("", web::post().to(post::create_post))
                    .route("", web::get().to(post::get_posts)),
            )
            .service(
                web::scope("/comments")
                    .route("/{post_id}", web::get().to(comment::get_post_comments))
                    .route("/{post_id}", web::post().to(comment::create_comment))
                    .route("/{comment_id}", web::get().to(comment::get_comment))
                    .route("/replies", web::get().to(comment::get_comment_replies))
                    .route("/stats", web::get().to(comment::get_comment_stats))
                    .route("/{id}", web::put().to(comment::update_comment))
                    .route("/{id}", web::delete().to(comment::delete_comment)),
            )
            .service(
                web::scope("/post/like")
                    .route("/{post_id}", web::post().to(like::like_post))
                    .route("/{post_id}", web::delete().to(like::unlike_post))
                    .route("/{post_id}/toggle", web::post().to(like::toggle_like))
                    .route("/{post_id}/likes", web::get().to(like::get_post_likes)),
            ),
    );
}

async fn health_check() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    })))
}
