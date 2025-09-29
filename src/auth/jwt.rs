use actix_web::dev::{ServiceRequest, ServiceResponse};
use std::{rc::Rc, task::{Context, Poll}};
use actix_web::Error;
use actix_service::{Service, Transform};
use futures::{future::{ok, LocalBoxFuture, Ready}};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::auth::claims::Claims;

pub struct JwtMiddleware {
    secret_key: String
}

impl JwtMiddleware{
    pub fn new(secret_key: String) -> Self {
        Self {
            secret_key
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService {
            service: Rc::new(service),
            secret_key: self.secret_key.clone()
        })
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
    secret_key: String,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        
        let auth_header = match req.headers().get("Authorization") {
            Some(header) => header,
            None => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Authorization header missing"))
                })
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Invalid Authorization header encoding"))
                })
            }
        };

        if !auth_str.starts_with("Bearer ") {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("Authorization header must start with 'Bearer '"))
            })
        }

        let token = &auth_str[7..];

        if token.is_empty() {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("Empty token"))
            });
        }

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data = decode::<Claims<serde_json::Value>>( // Concrete type
                token,
                &DecodingKey::from_secret(self.secret_key.as_bytes()),
                &validation
            );

        match token_data {
            Ok(_data) => {
                return Box::pin(service.call(req))
            },
            Err(err) => {
                let error_msg = match err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expired",
                    jsonwebtoken::errors::ErrorKind::InvalidToken => "Invalid token",
                    jsonwebtoken::errors::ErrorKind::InvalidSignature => "Invalid token signature",
                    jsonwebtoken::errors::ErrorKind::InvalidEcdsaKey => "Invalid key",
                    jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => "Invalid algorithm",
                    jsonwebtoken::errors::ErrorKind::InvalidIssuer => "Invalid issuer",
                    jsonwebtoken::errors::ErrorKind::InvalidAudience => "Invalid audience",
                    jsonwebtoken::errors::ErrorKind::InvalidSubject => "Invalid subject",
                    jsonwebtoken::errors::ErrorKind::ImmatureSignature => "Token not yet valid",
                    _ => "Invalid token", // Handles malformed_jwt_structure and other cases
                };

                Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized(error_msg))
                })
            }
        }
    }
}
