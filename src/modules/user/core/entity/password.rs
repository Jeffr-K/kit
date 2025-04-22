use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        Error as HashError
    },
    Pbkdf2
};
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct PasswordError(HashError);

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Password error: {}", self.0)
    }
}

impl StdError for PasswordError {}

impl From<HashError> for PasswordError {
    fn from(error: HashError) -> Self {
        PasswordError(error)
    }
}

#[derive(Debug)]
pub struct PasswordEncrypter {
    pub(crate) hash: String,
    pub(crate) salt: String
}

impl PasswordEncrypter {
    pub fn new() -> Self {
        Self {
            hash: String::new(),
            salt: String::new()
        }
    }

    pub fn hash(&self, password: &str) -> Result<Self, PasswordError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2
            .hash_password(password.as_bytes(), &salt)?;

        Ok(Self {
            hash: password_hash.to_string(),
            salt: salt.to_string()
        })
    }

    pub fn verify(
        &self,
        password: &str,
        hash: &str,
        salt: &str
    ) -> Result<bool, PasswordError> {
        let password_hash = PasswordHash::new(hash)?;
        let _salt_string = SaltString::from_b64(salt)?;

        Pbkdf2
            .verify_password(password.as_bytes(), &password_hash)?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash() {
        let password = "password123";
        let password_encrypter = PasswordEncrypter::new();
        let hashed_password = password_encrypter.hash(password).unwrap();
        assert_ne!(hashed_password.hash, password);
        assert_ne!(hashed_password.salt, password);
    }

    #[test]
    fn test_password_verify() {
        let password = "password123";
        let password_encrypter = PasswordEncrypter::new();
        let hashed_password = password_encrypter.hash(password).unwrap();
        let is_valid = password_encrypter.verify(
            password,
            &hashed_password.hash,
            &hashed_password.salt
        ).unwrap();
        assert!(is_valid);
    }
}