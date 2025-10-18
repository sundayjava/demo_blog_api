pub mod repository;
pub mod seed;

use crate::config::DatabaseConfig;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.url)
        .await?;

    tracing::info!("Database connection pool created successfully");
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    tracing::info!("Running database migrations...");

    // Check if migrations directory exists
    let migrations_path = std::path::Path::new("./migrations");
    if !migrations_path.exists() {
        tracing::error!("Migrations directory not found at {:?}", migrations_path);
        return Err(sqlx::migrate::MigrateError::VersionMissing(0));
    }

    tracing::info!("Migrations directory found");

    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            tracing::info!("Database migrations completed successfully");
            Ok(())
        }
        Err(e) => {
            tracing::error!("Migration failed: {:?}", e);
            Err(e)
        }
    }
}
