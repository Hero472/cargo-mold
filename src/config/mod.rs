use std::env;
use dotenvy::dotenv;

pub struct Config;

impl Config {
    /// Load .env file (Call this at the very top of main)
    pub fn init() {
        if let Err(e) = dotenv() {
            // We don't panic here because in Production (Docker), 
            // .env files often don't exist—real env vars are used instead.
            eprintln!("Note: No .env file loaded: {}", e);
        }
    }

    /// Required: Get a string or panic
    pub fn get(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| panic!("Environment variable [{}] is missing!", key))
    }

    /// Optional: Get a string or return a default
    pub fn get_or(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    /// Typed: Get and parse (e.g., Config::get_as::<u16>("PORT"))
    pub fn get_as<T>(key: &str) -> T 
    where 
        T: std::str::FromStr, 
        <T as std::str::FromStr>::Err: std::fmt::Debug 
    {
        Self::get(key)
            .parse::<T>()
            .expect(&format!("Environment variable [{}] is not the correct type!", key))
    }

    /// Check if a variable exists (useful for feature flags)
    pub fn has(key: &str) -> bool {
        env::var(key).is_ok()
    }
}