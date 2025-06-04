use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, Utc};

use crate::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,    // Subject (user ID)
    pub email: String,
    pub user_type: String,
    pub exp: i64,     // Expiration time
    pub iat: i64,     // Issued at
}

pub struct AuthService;

impl AuthService {
    /// Hash a password using Argon2
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::HashingFailed)?
            .to_string();
            
        Ok(password_hash)
    }
    
    /// Verify a password against its hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| AppError::HashingFailed)?;
            
        let argon2 = Argon2::default();
        
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    /// Generate a JWT token for a user
    pub fn generate_token(
        user_id: Uuid,
        email: String,
        user_type: String,
        secret: &str,
    ) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours
        
        let claims = Claims {
            sub: user_id,
            email,
            user_type,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|_| AppError::TokenGenerationFailed)?;
        
        Ok(token)
    }
    
    /// Validate and decode a JWT token
    pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AppError::InvalidToken)?;
        
        Ok(token_data.claims)
    }
    
    /// Extract token from Authorization header
    pub fn extract_token_from_header(auth_header: &str) -> Result<&str, AppError> {
        if auth_header.starts_with("Bearer ") {
            Ok(&auth_header[7..])
        } else {
            Err(AppError::InvalidToken)
        }
    }
}