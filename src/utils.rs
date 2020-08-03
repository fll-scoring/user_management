use argonautica::{input::Salt, Hasher, Verifier};
use fll_scoring::{config::get_service_config_value, errors::ServiceError};

/// Takes in a password and returns a Hash
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let secret_key = get_service_config_value("user_management", "password-key", true)?;
    let result = Hasher::default()
        .with_password(password)
        .with_secret_key(secret_key.as_str())
        .hash();

    match result {
        Ok(hash) => Ok(hash),
        Err(_err) => Err(ServiceError::InternalServerError),
    }
}

/// Verifies a password against a hash
pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    let secret_key = get_service_config_value("user_management", "password-key", true)?;
    let result = Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(secret_key.as_str())
        .verify();

    match result {
        Ok(b) => Ok(b),
        Err(_) => Err(ServiceError::InternalServerError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        let pass = "test";
        let hash = hash_password(&pass).unwrap();
        assert_eq!(verify(&hash, pass).unwrap(), true);
    }
}
