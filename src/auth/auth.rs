use std::error::Error;
use serde::{Serialize, Deserialize};
use sha2::{digest::generic_array::GenericArray, Digest, Sha256};
use base64::Engine;
use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, KeyInit, Nonce};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use bcrypt::{hash as crypt_hash, DEFAULT_COST};

use crate::auth::claims::Claims;

fn derive_key_from_string(key_str: &str) -> [u8; 32] {
    let hasher = Sha256::new_with_prefix(key_str.as_bytes());
    hasher.finalize().into()
}

#[derive(Serialize, Deserialize)]
pub struct AuthService {
    secret_key: String,
    encryption_key: String,
}
impl AuthService {

    pub fn new(secret_key: String, encryption_key: String) -> Self {
        Self {
            secret_key,
            encryption_key,
        }
    }

    pub fn hash(input: &str) -> Result<String, Box<dyn Error>> {
        let mut hasher = Sha256::new_with_prefix(input.as_bytes());
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    pub fn hash_password(input: &str) -> Result<String, bcrypt::BcryptError> {
        crypt_hash(input, DEFAULT_COST)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(password, hash)
    }

    pub fn generate_token<T: Serialize>(&self, email: String, data: T, minutes: i64) -> String {

        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(minutes))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: email,
            exp: expiration,
            iat: Utc::now().timestamp() as usize,
            data: data,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_bytes()),
        ).unwrap()
    }

    pub fn verify_token<T>(&self, token: &str) -> bool
    where
        T: for<'de> Deserialize<'de> + Clone, 
    {
        let validation = Validation::default();
        let result = decode::<Claims<T>>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &validation,
        );
        result.is_ok()
    }

    pub fn is_token_expired<T>(&self, token: &str) -> bool 
    where
        T: for<'de> Deserialize<'de> + Clone,
    {
        let validation = Validation::default();
        if let Ok(data) = decode::<Claims<T>>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &validation,
        ) {
            let now = Utc::now().timestamp() as usize;
            data.claims.exp < now
        } else {
            true // Treat invalid token as expired
        }
    }

    pub fn encrypt(&self, input: &str) -> Result<String, Box<dyn Error>> {

        let key_bytes = derive_key_from_string(&self.encryption_key);
        let key = GenericArray::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let cipher_text = cipher.encrypt(&nonce, input.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&cipher_text);

        Ok(base64::engine::general_purpose::STANDARD.encode(encrypted_data))
    }

    pub fn decrypt(&self, input: &str) -> Result<String, Box<dyn Error>> {

        let key_bytes = derive_key_from_string(&self.encryption_key);
        let key = GenericArray::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        let encrypted_data = base64::engine::general_purpose::STANDARD.decode(input)
            .map_err(|e| format!("Base64 decode failed: {}", e))?;

        if encrypted_data.len() < 12 {
            return Err("Invalid encrypted data: too short".into());
        }
        
        let (nonce_bytes, cipher_text) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = cipher.decrypt(nonce, cipher_text)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| format!("Invalid UTF-8: {}", e).into())
    }
}