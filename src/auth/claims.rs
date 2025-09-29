use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims<T = serde_json::Value> {
    /// Subject (whom the token refers to)
    pub sub: String,
    /// Issued at (timestamp)
    pub iat: usize,
    /// Expiration time (timestamp) 
    pub exp: usize,
    /// Custom claims data
    pub data: T,
}

impl<T> Claims<T> {
    pub fn new(sub: String, iat: usize, exp: usize, data: T) -> Self {
        Self { sub, iat, exp, data }
    }
}