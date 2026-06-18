use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use serde_json::Value;
use std::marker::PhantomData;
use futures::future::{ready, Ready};
use crate::auth::claims::Claims;
use crate::common::roles::RequiredRole;

pub struct RoleGuard<R: RequiredRole> {
    _marker: PhantomData<R>
}

impl<R: RequiredRole + 'static> FromRequest for RoleGuard<R> {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        
        let claims = match req.extensions().get::<Claims<Value>>().cloned() {
            Some(c) => c,
            None => {
                return ready(Err(actix_web::error::ErrorUnauthorized(
                    "missing or invalid token",
                )));
            }
        };

        let role_matches = claims.data.get("role")
            .and_then(|v| v.as_str())
            .map(|r| r == R::ROLE)
            .unwrap_or(false);

        if role_matches {
            ready(Ok(RoleGuard { _marker: PhantomData }))
        } else {
            ready(Err(actix_web::error::ErrorForbidden(
                format!("required role: {}", R::ROLE),
            )))
        }

    }
}