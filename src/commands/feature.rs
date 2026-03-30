use clap::Args;
use anyhow::{Context, Result, bail};
use std::path::Path;
use tokio::fs;

use crate::templates::{TemplateType, contents::modular, template_engine::TemplateEngine, types::CargoSmith};

#[derive(Args)]
pub struct FeatureArgs {
    /// Name of the feature (e.g., "users")
    pub name: String,
}

pub async fn execute(args: FeatureArgs) -> anyhow::Result<()> {

    let config_path = ".cargo-smith";

    if !Path::new(".cargo-smith").exists() {
        bail!(
            "Not a cargo-smith project. Run this in a project created with `cargo-smith new`."
        );
    }

    let config_content = fs::read_to_string(config_path).await?;

    let config: CargoSmith = toml::from_str(&config_content)
        .context("Failed to parse .cargo-smith config")?;

    let feature_name = args.name.to_lowercase();

    if config.generated.features.contains(&feature_name) {
        bail!("Feature '{}' is already registered in .cargo-smith", feature_name);
    }

    let base_path = format!("src/features/{}/", feature_name);
    if Path::new(&base_path).exists() {
        bail!("Folder '{}' already exists, but isn't in .cargo-smith. Manual cleanup required.", base_path);
    }

    println!("Generating feature: {}", feature_name);
    fs::create_dir_all(&base_path).await?;

    let files = [
        (format!("{}/mod.rs", base_path), modular::feature::MOD),
        (format!("{}/controller.rs", base_path), modular::feature::CONTROLLER),
        (format!("{}/service.rs", base_path), modular::feature::SERVICE),
        (format!("{}/model.rs", base_path), modular::feature::MODEL),
        (format!("{}/routes.rs", base_path), modular::feature::ROUTES),
    ];

    for (path, content) in files {
        // Here we pass the feature_name so the engine can replace {{name}} in the templates
        TemplateEngine::generate_from_template(
            &feature_name, 
            &path,
            content,
            &TemplateType::Modular,
            false
        ).await?;
    }

    update_resource_cargo_smith(feature_name.clone()).await?;

    register_in_parent_mod(&feature_name).await?;
    
    println!("Resource '{}' created successfully!", feature_name);
    println!("Generated files:");
    println!("   - src/{}", feature_name);
    println!("   - src/{}/mod.rs", feature_name);
    println!("   - src/{}/model.rs", feature_name);
    println!("   - src/{}/controller.rs", feature_name);
    println!("   - src/{}/service.rs", feature_name);
    println!("   - src/{}/routes.rs", feature_name);

    Ok(())
}

async fn update_resource_cargo_smith(name: String) -> Result<()> {
    
    let content = fs::read_to_string(".cargo-smith").await?;

    let mut config: CargoSmith = toml::from_str(&content)?;

    if !config.generated.features.contains(&name) {
        config.generated.features.push(name);
        let updated_content = toml::to_string_pretty(&config)?;
        fs::write(".cargo-smith", updated_content).await?;
    }
    Ok(())
}

async fn register_in_parent_mod(name: &str) -> anyhow::Result<()> {
    let mod_file = "src/features/mod.rs";
    let mod_path = std::path::Path::new(mod_file);

    // 1. Get current content or use a default template if empty/missing
    let mut content = if mod_path.exists() {
        tokio::fs::read_to_string(mod_file).await?
    } else {
        String::new()
    };

    // 2. "Upgrade" the file if it's the old simple version or empty
    if !content.contains("// [SMITH-MOD]") {
        content = format!(
            "use actix_web::web;\n\n// [SMITH-MOD]\n\npub fn init(cfg: &mut web::ServiceConfig) {{\n    // [SMITH-INIT]\n}}\n"
        );
    }

    let mod_declaration = format!("pub mod {};", name);
    let init_call = format!("    {}::init(cfg);", name);

    // 3. Inject 'pub mod' declaration
    if !content.contains(&mod_declaration) {
        content = content.replace(
            "// [SMITH-MOD]",
            &format!("// [SMITH-MOD]\n{}", mod_declaration)
        );
    }

    // 4. Inject 'init' call
    if !content.contains(&init_call) {
        content = content.replace(
            "// [SMITH-INIT]",
            &format!("// [SMITH-INIT]\n{}", init_call)
        );
    }

    tokio::fs::write(mod_file, content).await?;
    Ok(())
}

// Someday I will do some routing framework os something to make it really well and useful