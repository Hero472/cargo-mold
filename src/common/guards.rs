use actix_web::{FromRequest, HttpRequest, HttpMessage, dev::Payload};
use futures::future::{ready, Ready};
use serde_json::Value;
use std::marker::PhantomData;
use crate::auth::Claims;
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
    _role: PhantomData<R>,
    _perm: PhantomData<P>,
}

// --- GuardClaims impls ---

impl GuardClaims for AuthGuard { fn claims(&self) -> &Claims<Value> { &self.0 } }
impl GuardClaims for RegisteredGuard { fn claims(&self) -> &Claims<Value> { &self.0 } }
impl GuardClaims for GuestGuard { fn claims(&self) -> &Claims<Value> { &self.0 } }

impl<R: Role, P: RequiredRole> GuardClaims for RequireRole<R, P> {
    fn claims(&self) -> &Claims<Value> { &self.claims }
}

// --- FromRequest impls ---

impl FromRequest for AuthGuard {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let result = req.extensions().get::<Claims<Value>>().cloned()
            .map(AuthGuard)
            .ok_or(AppError::Unauthorized("missing or invalid token"));
        ready(result)
    }
}

impl FromRequest for RegisteredGuard {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let result = match req.extensions().get::<Claims<Value>>().cloned() {
            Some(c) if c.data["kind"] == "registered" => Ok(RegisteredGuard(c)),
            Some(_) => Err(AppError::Forbidden("registered users only")),
            None    => Err(AppError::Unauthorized("missing token")),
        };
        ready(result)
    }
}

impl FromRequest for GuestGuard {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let result = match req.extensions().get::<Claims<Value>>().cloned() {
            Some(c) if c.data["kind"] == "guest" => Ok(GuestGuard(c)),
            Some(_) => Err(AppError::Forbidden("guests only")),
            None    => Err(AppError::Unauthorized("missing token")),
        };
        ready(result)
    }
}

impl<R: Role, P: RequiredRole> FromRequest for RequireRole<R, P> {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let result = match req.extensions().get::<Claims<Value>>().cloned() {
            Some(c) => {
                let role_matches = c.data
                    .get("role")
                    .and_then(|r| serde_json::from_value::<R>(r.clone()).ok())
                    .map(|r| r.as_str() == P::ROLE)
                    .unwrap_or(false);

                if role_matches {
                    Ok(RequireRole { claims: c, _role: PhantomData, _perm: PhantomData })
                } else {
                    Err(AppError::Forbidden("insufficient role"))
                }
            }
            None => Err(AppError::Unauthorized("missing token")),
        };
        ready(result)
    }
}