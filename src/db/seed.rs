use crate::db::repository::user as repository;
use crate::error::AppError;
use crate::models::CreateUserDto;
use crate::services::password::PasswordService;
use sqlx::PgPool;

pub async fn seed_admin(pool: &PgPool) -> Result<(), AppError> {
    // Check if admin exists
    let admin_email =
        std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());

    if repository::get_user_by_email(pool, &admin_email)
        .await?
        .is_some()
    {
        tracing::info!("Admin user already exists");
        return Ok(());
    }

    // Create admin user
    let admin_password = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "Admin123".to_string());

    let password_hash = PasswordService::hash_password(&admin_password)?;

    let admin_dto = CreateUserDto {
        username: "admin".to_string(),
        email: admin_email.clone(),
        password: admin_password,
    };

    let mut user = repository::create_user(pool, admin_dto, password_hash).await?;

    // Update role to admin
    user = repository::update_user_role(pool, user.id, "admin").await?;

    tracing::info!("Admin user created: {}, {}", admin_email, user.role);

    Ok(())
}
