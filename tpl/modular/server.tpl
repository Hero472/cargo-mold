// Server configuration and startup
use actix_web::{App, HttpServer};

use crate::features::init;

/// Starts the HTTP server and begins listening for requests
pub async fn run() -> std::io::Result<()> {
    println!("Starting Actix Web server on http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}