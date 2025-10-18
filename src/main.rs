mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http, middleware::Logger, web};
use config::AppConfig;
use db::{create_pool, run_migrations};
use routes::auth::configure_routes;
use services::jwt::JwtService;
use std::sync::Arc;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,actix_web=debug,sqlx=debug,blog_api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Blog API...");

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");

    tracing::info!("Configuration loaded - Environment: {}", config.environment);

    // Create database pool
    let pool = create_pool(&config.database)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    // Seed admin user (if needed)
    if let Err(e) = db::seed::seed_admin(&pool).await {
        tracing::warn!("Failed to seed admin user: {}", e);
    }

    // Create JWT service
    let jwt_service = Arc::new(JwtService::new(
        &config.jwt.secret,
        config.jwt.expiration_hours,
    ));

    let server_host = config.server.host.clone();
    let server_port = config.server.port;
    let is_production = config.is_production();

    tracing::info!("Server starting at {}:{}", server_host, server_port);

    HttpServer::new(move || {
        // Configure CORS
        let cors = if is_production {
            Cors::default()
                .allowed_origin("https://yourdomain.com")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT,
                    http::header::CONTENT_TYPE,
                ])
                .max_age(3600)
        } else {
            Cors::permissive()
        };

        App::new()
            // Add application data
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_service.clone()))
            .app_data(web::Data::new(config.clone()))
            // Add middleware
            .wrap(Logger::default())
            .wrap(cors)
            // Configure routes
            .configure(|cfg| configure_routes(cfg, jwt_service.clone()))
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}
