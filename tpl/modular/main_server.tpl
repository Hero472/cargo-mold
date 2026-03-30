// Main entry point for the Actix Web application
use {{name_snake_case}}::server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run().await
}