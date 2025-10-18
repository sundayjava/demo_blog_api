use role_base_auth::config::AppConfig;
use role_base_auth::db::{create_pool, repository::user as repository};
use role_base_auth::error::AppError;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: manage-roles <user-id|email> <role> [--by-email]");
        eprintln!("Roles: admin, moderator, user");
        eprintln!("_");
        eprintln!("Examples:");
        eprintln!("  cargo run --bin manage-roles 550e8400-e29b-41d4-a716-446655440000 admin");
        eprintln!("  cargo run --bin manage-roles user@example.com moderator --by-email");
        std::process::exit(1);
    }

    let identifier = &args[1];
    let role = &args[2];
    let by_email = args.len() > 3 && args[3] == "--by-email";

    // Validate role
    if !["admin", "moderator", "user"].contains(&role.as_str()) {
        eprintln!("Error: Invalid role. Must be: admin, moderator, or user");
        std::process::exit(1);
    }

    // Setup
    let config = AppConfig::from_env()
        .map_err(|e| AppError::InternalError(format!("Configuration error: {}", e)))?;
    let pool = create_pool(&config.database).await?;

    // Get user
    let user = if by_email {
        repository::get_user_by_email(&pool, identifier)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("User not found with email: {}", identifier))
            })?
    } else {
        let user_id = Uuid::parse_str(identifier)
            .map_err(|e| AppError::BadRequest(format!("Invalid UUID: {}", e)))?;
        repository::get_user_by_id(&pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User not found with ID: {}", user_id)))?
    };

    println!("Found user: {} ({})", user.username, user.email);
    println!("Current role: {}", user.role);

    // Update role
    let updated_user = repository::update_user_role(&pool, user.id, role).await?;

    println!("✓ Role updated to: {}", updated_user.role);

    Ok(())
}
