use role_base_auth::services::password::PasswordService;

#[test]
fn test_password_hashing() {
    let password = "SecurePassword123";
    let hash = PasswordService::hash_password(password).unwrap();

    assert!(PasswordService::verify_password(password, &hash).unwrap());
    assert!(!PasswordService::verify_password("WrongPassword", &hash).unwrap());
}
