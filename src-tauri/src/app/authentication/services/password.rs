use argon2::password_hash::SaltString;
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

pub fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
    let secret_key = std::env::var("PASSWORD_SECRET_KEY").unwrap_or("secret_key_12345".to_string());
    let selt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new_with_secret(
        secret_key.as_bytes(),
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::default(),
    );

    Ok(argon2
        .unwrap()
        .hash_password(password.as_bytes(), &selt)?
        .to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let secret_key = std::env::var("PASSWORD_SECRET_KEY").unwrap_or("secret_key_12345".to_string());
    let hash_password = PasswordHash::new(&hash);
    let argon2 = Argon2::new_with_secret(
        secret_key.as_bytes(),
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::default(),
    );

    Ok(argon2
        .unwrap()
        .verify_password(password.as_bytes(), &hash_password.unwrap())
        .is_ok())
}
