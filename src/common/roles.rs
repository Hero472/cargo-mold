use serde::{Serialize, Deserialize};

pub trait Role: for<'de> Deserialize<'de> + Serialize + PartialEq + Clone + Send + Sync + 'static {
    fn as_str(&self) -> &str;
}

pub trait RequiredRole {
    const ROLE: &'static str;
}

// Built-in role enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DefaultRole {
    Admin,
    User,
    Guest,
}

impl Role for DefaultRole {
    fn as_str(&self) -> &str {
        match self {
            Self::Admin => "admin",
            Self::User  => "user",
            Self::Guest => "guest",
        }
    }
}

// Built-in marker structs
pub struct Admin;
pub struct User;
pub struct Guest;

impl RequiredRole for Admin { const ROLE: &'static str = "admin"; }
impl RequiredRole for User  { const ROLE: &'static str = "user";  }
impl RequiredRole for Guest { const ROLE: &'static str = "guest"; }