use role_base_auth::services::jwt::JwtService;
use uuid::Uuid;

#[test]
fn test_jwt_generation_and_verification() {
    let jwt_service = JwtService::new("test_secret", 24);
    let user_id = Uuid::new_v4();
    let email = "test@example.com".to_string();
    let role = "user".to_string();

    let token = jwt_service
        .generate_token(user_id, email.clone(), role.clone())
        .unwrap();

    let claims = jwt_service.verify_token(&token).unwrap();

    assert_eq!(claims.sub, user_id.to_string());
    assert_eq!(claims.email, email);
    assert_eq!(claims.role, role);
}

#[test]
fn test_invalid_token() {
    let jwt_service = JwtService::new("test_secret", 24);
    let result = jwt_service.verify_token("invalid_token");
    assert!(result.is_err());
}

#[test]
fn test_token_from_different_secret() {
    let jwt_service1 = JwtService::new("secret1", 24);
    let jwt_service2 = JwtService::new("secret2", 24);

    let user_id = Uuid::new_v4();
    let token = jwt_service1
        .generate_token(user_id, "test@example.com".to_string(), "user".to_string())
        .unwrap();

    let result = jwt_service2.verify_token(&token);
    assert!(result.is_err());
}
