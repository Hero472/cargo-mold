use actix_web::{HttpMessage, dev::{ServiceRequest, ServiceResponse}};
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
        let secret_key = self.secret_key.clone();

        // 1. Try Authorization: Bearer header first
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|t| t.to_string());

        // 2. Fall back to ?token= query param (needed for WebSocket upgrades)
        let token = token.or_else(|| {
            url::form_urlencoded::parse(req.query_string().as_bytes())
                .find(|(k, _)| k == "token")
                .map(|(_, v)| v.into_owned())
        });

        let token = match token {
            Some(t) if !t.is_empty() => t,
            _ => return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("missing token"))
            }),
        };

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data = decode::<Claims<serde_json::Value>>(
            &token,
            &DecodingKey::from_secret(secret_key.as_bytes()),
            &validation,
        );

        match token_data {
            Ok(data) => {
                // Inject claims into request extensions so handlers can extract them
                req.extensions_mut().insert(data.claims);
                Box::pin(service.call(req))
            }
            Err(err) => {
                let msg = match err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature  => "token expired",
                    jsonwebtoken::errors::ErrorKind::InvalidToken      => "invalid token",
                    jsonwebtoken::errors::ErrorKind::InvalidSignature  => "invalid token signature",
                    jsonwebtoken::errors::ErrorKind::ImmatureSignature => "token not yet valid",
                    _                                                  => "invalid token",
                };
                Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized(msg))
                })
            }
        }
    }
}
