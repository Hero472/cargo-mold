use clap::Args;
use anyhow::Result;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Args)]
pub struct ResourceArgs {
    pub name: String,
}

pub async fn execute(args: ResourceArgs) -> anyhow::Result<()> {
    println!("📁 Generating resource: {}", args.name);
    
    if !Path::new(".cargo-mold").exists() {
        anyhow::bail!(
            "❌ Not a cargo-mold project.\n\
             Run this command in a project created with `cargo mold new`\n\
             Or create a new project with: `cargo mold new {}`",
            args.name
        );
    }
    
    generate_model(&args.name).await?;
    generate_handler(&args.name).await?;
    generate_routes(&args.name).await?;
    update_modules(&args.name).await?;
    
    println!("✅ Resource '{}' created successfully!", args.name);
    println!("📝 Generated files:");
    println!("   - src/models/{}.rs", args.name);
    println!("   - src/handlers/{}_handlers.rs", args.name);
    println!("   - src/routes/{}_routes.rs", args.name);
    
    Ok(())
}

async fn generate_model(resource_name: &str) -> Result<()> {
    let pascal_case = to_pascal_case(resource_name);
    let content = format!(
        r#"use serde::{{Deserialize, Serialize}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct {} {{
}}

impl {} {{
    pub fn new() -> Self {{
        Self {{
        }}
    }}
}}
"#, 
        pascal_case, pascal_case
    );

    let file_path = format!("src/models/{}.rs", resource_name);
    let mut file = fs::File::create(&file_path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

async fn generate_handler(resource_name: &str) -> Result<()> {
    let pascal_case = to_pascal_case(resource_name);
    let content = format!(
        r#"use actix_web::{{web, HttpResponse}};
use crate::models::{}::{};

pub async fn create_{}({}_data: web::Json<{}>) -> HttpResponse {{
    HttpResponse::Created().json({}_data)
}}

pub async fn get_{}() -> HttpResponse {{
    HttpResponse::Ok().finish()
}}

pub async fn update_{}(path: web::Path<String>, {}_data: web::Json<{}>) -> HttpResponse {{
    HttpResponse::Ok().json({}_data.clone())
}}

pub async fn delete_{}(path: web::Path<String>) -> HttpResponse {{
    HttpResponse::NoContent().finish()
}}
"#,
        resource_name, pascal_case,  // use statements
        resource_name, resource_name, pascal_case,
        resource_name,
        resource_name,
        resource_name, resource_name, pascal_case,
        resource_name,
        resource_name
    );

    let file_path = format!("src/handlers/{}_handlers.rs", resource_name);
    let mut file = fs::File::create(&file_path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

async fn generate_routes(resource_name: &str) -> Result<()> {
    let content = format!(
        r#"use actix_web::web;
use crate::handlers::{}_handlers;

pub fn {}_routes(cfg: &mut web::ServiceConfig) {{
    cfg.service(
        web::scope("/{}")
            .route("", web::get().to({}_handlers::get_{}))
            .route("", web::post().to({}_handlers::create_{}))
            .route("/{{id}}", web::get().to({}_handlers::get_{}))
            .route("/{{id}}", web::put().to({}_handlers::update_{}))
            .route("/{{id}}", web::delete().to({}_handlers::delete_{}))
    );
}}
"#,
        resource_name,
        resource_name,
        resource_name,
        resource_name, resource_name,
        resource_name, resource_name,
        resource_name, resource_name,
        resource_name, resource_name,
        resource_name, resource_name
    );

    let file_path = format!("src/routes/{}_routes.rs", resource_name);
    let mut file = fs::File::create(&file_path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

async fn update_modules(resource_name: &str) -> Result<()> {
    // Update models/mod.rs
    let models_mod_path = "src/models/mod.rs";
    if Path::new(models_mod_path).exists() {
        let mut models_mod = fs::read_to_string(models_mod_path).await?;
        if !models_mod.contains(&format!("pub mod {};", resource_name)) {
            models_mod.push_str(&format!("\npub mod {};", resource_name));
            fs::write(models_mod_path, models_mod).await?;
        }
    }
    
    // Update handlers/mod.rs
    let handlers_mod_path = "src/handlers/mod.rs";
    if Path::new(handlers_mod_path).exists() {
        let mut handlers_mod = fs::read_to_string(handlers_mod_path).await?;
        if !handlers_mod.contains(&format!("pub mod {}_handlers;", resource_name)) {
            handlers_mod.push_str(&format!("\npub mod {}_handlers;", resource_name));
            fs::write(handlers_mod_path, handlers_mod).await?;
        }
    }
    
    // Update routes/mod.rs
    let routes_mod_path = "src/routes/mod.rs";
    if Path::new(routes_mod_path).exists() {
        let mut routes_mod = fs::read_to_string(routes_mod_path).await?;
        if !routes_mod.contains(&format!("pub mod {}_routes;", resource_name)) {
            routes_mod.push_str(&format!("\npub mod {}_routes;", resource_name));
            fs::write(routes_mod_path, routes_mod).await?;
        }
    }
    
    // Update main routes.rs to include the new routes
    let routes_file_path = "src/routes/routes.rs";
    if Path::new(routes_file_path).exists() {
        let mut routes_file = fs::read_to_string(routes_file_path).await?;
        
        if routes_file.contains("pub fn public_routes") && 
        !routes_file.contains(&format!("{}_routes::{}_routes", resource_name, resource_name)) {
            
            // 1. Add the use statement at the top with other use statements
            let use_statement = format!("use crate::routes::{}_routes;\n", resource_name);
            
            // Find a good place to insert the use statement (after the last existing use)
            if let Some(last_use_pos) = routes_file.rfind("use ") {
                if let Some(next_newline) = routes_file[last_use_pos..].find('\n') {
                    let insert_pos = last_use_pos + next_newline + 1;
                    routes_file.insert_str(insert_pos, &use_statement);
                }
            } else {
                // If no use statements found, add after the module comments
                if let Some(mod_end_pos) = routes_file.find("use actix_web::web;") {
                    let insert_pos = mod_end_pos + "use actix_web::web;".len();
                    routes_file.insert_str(insert_pos, &format!("\n{}", use_statement));
                }
            }
            
            // 2. Add the route configuration inside public_routes scope
            if let Some(scope_pos) = routes_file.find("web::scope(\"/api\")") {
                // Find the closing parenthesis of the scope
                if let Some(scope_end_pos) = find_matching_parenthesis(&routes_file, scope_pos) {
                    // Look for the closing brace of the service configuration
                    if let Some(service_end_pos) = routes_file[scope_end_pos..].find(')') {
                        let insert_pos = scope_end_pos + service_end_pos;
                        
                        // Insert before the closing parenthesis of the service call
                        routes_file.insert_str(insert_pos, 
                            &format!("\n            .configure({}_routes::{}_routes)", resource_name, resource_name));
                    }
                }
            }
        }
        fs::write(routes_file_path, routes_file).await?;
    }
    
    Ok(())
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for c in s.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

fn find_matching_parenthesis(content: &str, start_pos: usize) -> Option<usize> {
    let mut count = 1;
    let chars: Vec<char> = content[start_pos..].chars().collect();
    
    for (i, c) in chars.iter().enumerate().skip(1) {
        match c {
            '(' => count += 1,
            ')' => {
                count -= 1;
                if count == 0 {
                    return Some(start_pos + i);
                }
            }
            _ => {}
        }
    }
    None
}