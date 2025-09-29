use anyhow::Result;
use clap::Args;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Args)]
pub struct NewArgs {
    /// Name of the project
    pub project_name: String,
}

/// Creates a new Actix Web project with proper structure and boilerplate code
pub async fn execute(args: NewArgs) -> Result<()> {
    println!("🚀 Creating new project: {}", args.project_name);

    // Create project structure and generate all necessary files
    create_project_structure(&args.project_name).await?;
    generate_cargo_toml(&args.project_name).await?;
    generate_main_rs(&args.project_name).await?;
    generate_lib_rs(&args.project_name).await?;
    generate_route_files(&args.project_name).await?;
    generate_handler_files(&args.project_name).await?;
    generate_server_files(&args.project_name).await?;
    generate_mod_files(&args.project_name).await?;

    println!("✅ Project '{}' created successfully!", args.project_name);
    println!("📂 Next steps:");
    println!("   cd {}", args.project_name);
    println!("   cargo run");

    Ok(())
}

/// Creates the directory structure for the Actix Web project
async fn create_project_structure(project_name: &str) -> Result<()> {
    let project_path = Path::new(project_name);
    fs::create_dir_all(project_path.join("src/routes")).await?;
    fs::create_dir_all(project_path.join("src/models")).await?;
    fs::create_dir_all(project_path.join("src/handlers")).await?;
    fs::create_dir_all(project_path.join("src/server")).await?;
    fs::create_dir_all(project_path.join("src/utils")).await?;
    Ok(())
}

/// Generates the Cargo.toml file with necessary dependencies
async fn generate_cargo_toml(project_name: &str) -> Result<()> {
    let content = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.4"
tokio = {{ version = "1.0", features = ["full"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"

[lib]
name = "{}"
path = "src/lib.rs"
"#, 
        project_name, project_name
    );

    let mut file = fs::File::create(format!("{}/Cargo.toml", project_name)).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

/// Generates the main.rs file with server initialization
async fn generate_main_rs(project_name: &str) -> Result<()> {
    let content = format!(
        r#"// Main entry point for the Actix Web application
use {}::server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {{
    server::run().await
}}"#,
        project_name.replace("-", "_")
    );

    let mut file = fs::File::create(format!("{}/src/main.rs", project_name)).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

/// Generates the lib.rs file with module declarations
async fn generate_lib_rs(project_name: &str) -> Result<()> {
    let content = r#"// Library crate root module declarations
pub mod server;
pub mod routes;
pub mod models;
pub mod utils;
pub mod handlers;"#;

    let mut file = fs::File::create(format!("{}/src/lib.rs", project_name)).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

/// Generates route-related files
async fn generate_route_files(project_name: &str) -> Result<()> {
    // routes/routes.rs
    let routes_file = r#"// Route configuration module
// Defines all public API routes and their handlers
use actix_web::web;

use crate::handlers::handlers;

/// Configures all public routes for the application
pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/hello", web::get().to(handlers::hello))
    );
}"#;

    let mut file = fs::File::create(format!("{}/src/routes/routes.rs", project_name)).await?;
    file.write_all(routes_file.as_bytes()).await?;

    Ok(())
}

/// Generates handler files with example handlers
async fn generate_handler_files(project_name: &str) -> Result<()> {
    // handlers/handlers.rs
    let handlers_file = r#"// Request handlers for the Actix Web application
use actix_web::{HttpResponse, Responder};

/// Simple hello world endpoint
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World! from Actix Web")
}"#;
    let mut file = fs::File::create(format!("{}/src/handlers/handlers.rs", project_name)).await?;
    file.write_all(handlers_file.as_bytes()).await?;

    Ok(())
}

/// Generates server configuration files
async fn generate_server_files(project_name: &str) -> Result<()> {
    // server/server.rs
    let server_file = r#"// Server configuration and startup
use actix_web::{App, HttpServer};
use crate::routes;

/// Starts the HTTP server and begins listening for requests
pub async fn run() -> std::io::Result<()> {
    println!("🚀 Starting Actix Web server on http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .configure(routes::routes::public_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}"#;

    let mut file = fs::File::create(format!("{}/src/server/server.rs", project_name)).await?;
    file.write_all(server_file.as_bytes()).await?;

    Ok(())
}

async fn generate_mod_files(project_name: &str) -> Result<()> {
    // models/mod.rs
    let models_mod = r#"// Data models and structures for the application
// Define your database models, request/response DTOs, and domain models here"#;

    let mut file = fs::File::create(format!("{}/src/models/mod.rs", project_name)).await?;
    file.write_all(models_mod.as_bytes()).await?;

    // utils/mod.rs
    let utils_mod = r#"// Utility functions and helpers
// Common utilities, helpers, and shared functionality across the application"#;

    let mut file = fs::File::create(format!("{}/src/utils/mod.rs", project_name)).await?;
    file.write_all(utils_mod.as_bytes()).await?;

    // handlers/mod.rs (if not already created)
    let handlers_mod = r#"// Request handlers for the Actix Web application
pub mod handlers;"#;

    let mut file = fs::File::create(format!("{}/src/handlers/mod.rs", project_name)).await?;
    file.write_all(handlers_mod.as_bytes()).await?;

    // routes/mod.rs (if not already created)
    let routes_mod = r#"// Route configuration module
pub mod routes;"#;

    let mut file = fs::File::create(format!("{}/src/routes/mod.rs", project_name)).await?;
    file.write_all(routes_mod.as_bytes()).await?;

    // server/mod.rs (if not already created)
    let server_mod = r#"// Server configuration and startup logic
pub mod server;"#;

    let mut file = fs::File::create(format!("{}/src/server/mod.rs", project_name)).await?;
    file.write_all(server_mod.as_bytes()).await?;

    Ok(())
}
