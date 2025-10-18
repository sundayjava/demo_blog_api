use blog_api::config::{AppConfig, DatabaseConfig, JwtConfig, ServerConfig};
use blog_api::services::JwtService;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn setup_test_db() -> PgPool {
    dotenv::dotenv().ok();
    
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/blog_api_test".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub async fn cleanup_database(pool: &PgPool) {
    sqlx::query("TRUNCATE users, posts, comments, likes CASCADE")
        .execute(pool)
        .await
        .expect("Failed to cleanup database");
}

pub fn create_test_jwt_service() -> Arc<JwtService> {
    Arc::new(JwtService::new("test_secret_key", 24))
}

pub fn create_test_config() -> AppConfig {
    AppConfig {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        },
        database: DatabaseConfig {
            url: "postgres://localhost/test".to_string(),
            max_connections: 5,
        },
        jwt: JwtConfig {
            secret: "test_secret".to_string(),
            expiration_hours: 24,
        },
        environment: "test".to_string(),
    }
}

pub fn generate_test_email() -> String {
    format!("test-{}@example.com", Uuid::new_v4())
}

pub fn generate_test_username() -> String {
    format!("testuser-{}", Uuid::new_v4().to_string()[..8].to_string())
}