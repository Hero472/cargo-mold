// Route configuration module
// Defines all public API routes and their handlers
use actix_web::web;
use cargo_smith::auth::JwtMiddleware;

use crate::handlers::handlers;

/// Configures all public routes for the application
pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/hello", web::get().to(handlers::hello))
    );
}

/// Configures all private routes for the application
pub fn private_routes(cfg: &mut web::ServiceConfig) {

    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in environment");
    let jwt_middleware = JwtMiddleware::new(jwt_secret);

    cfg.service(
        web::scope("/private-api")
            .wrap(jwt_middleware)
            .route("/", web::get().to(handlers::hello))
    );
}