use actix_web::{FromRequest, HttpRequest, dev::Payload, HttpMessage};
use futures::future::{ready, Ready};
use std::marker::PhantomData;
use serde_json::Value;
use crate::auth::Claims;
use crate::common::DefaultRoles;
use crate::common::errors::AppError;
use crate::common::roles::{Role, RequiredRole};

pub trait GuardClaims {
    fn claims(&self) -> &Claims<Value>;
    fn user_id(&self) -> &str { &self.claims().sub }
    fn username(&self) -> &str {
        self.claims().data["username"].as_str().unwrap_or("unknown")
    }
    fn role_str(&self) -> Option<&str> {
        self.claims().data["role"].as_str()
    }
}

// --- Guard structs ---

pub struct AuthGuard(pub Claims<Value>);
pub struct RegisteredGuard(pub Claims<Value>);
pub struct GuestGuard(pub Claims<Value>);

pub struct RequireRole<R: Role, P: RequiredRole> {
    pub claims: Claims<Value>,
    pub role: R, 
    _marker: PhantomData<P>,
}

// --- GuardClaims impls ---

impl GuardClaims for AuthGuard { fn claims(&self) -> &Claims<Value> { &self.0 } }
impl GuardClaims for RegisteredGuard { fn claims(&self) -> &Claims<Value> { &self.0 } }
impl GuardClaims for GuestGuard { fn claims(&self) -> &Claims<Value> { &self.0 } }

impl<R: Role, P: RequiredRole> GuardClaims for RequireRole<R, P> {
    fn claims(&self) -> &Claims<Value> { &self.claims }
}

// --- FromRequest impls ---

impl<R: Role, P: RequiredRole> FromRequest for RequireRole<R, P> {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let result = match req.extensions().get::<Claims<Value>>().cloned() {
            Some(c) => {
                // 1. Try to extract the "role" from the claims JSON
                let user_role = c.data.get("role")
                    .and_then(|v| serde_json::from_value::<R>(v.clone()).ok());

                match user_role {
                    // 2. Compare the runtime string with the compile-time constant
                    Some(r) if r.as_str() == P::ROLE => {
                        Ok(RequireRole { 
                            claims: c, 
                            role: r, 
                            _marker: PhantomData 
                        })
                    }
                    Some(_) => Err(AppError::Forbidden("Insufficient permissions")),
                    None => Err(AppError::Forbidden("Invalid or missing role in token")),
                }
            }
            None => Err(AppError::Unauthorized("Missing authentication claims")),
        };
        ready(result)
    }
}

/// Built-in alias for Admin-only access using the DefaultRole enum.
pub type RequireAdmin = RequireRole<DefaultRoles, crate::common::roles::Admin>;

/// Built-in alias for Registered User access using the DefaultRole enum.
pub type RequireUser = RequireRole<DefaultRoles, crate::common::roles::User>;

/// Built-in alias for Guest access using the DefaultRole enum.
pub type RequireGuest = RequireRole<DefaultRoles, crate::common::roles::Guest>;

#[macro_export]
macro_rules! create_auth_aliases {
    ($enum_ty:ty, $($marker:ty => $alias:ident),* $(,)?) => {
        $(
            pub type $alias = $crate::guards::RequireRole<$enum_ty, $marker>;
        )*
    };
}

