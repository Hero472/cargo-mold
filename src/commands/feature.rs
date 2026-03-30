use clap::Args;
use anyhow::{Result, bail};
use std::path::Path;
use tokio::fs;
use anyhow::Context;

use crate::templates::{TemplateType, types::CargoSmith};

#[derive(Args)]
pub struct FeatureArgs {
    /// Name of the feature (e.g., "users")
    pub name: String,
}

pub async fn execute(args: FeatureArgs) -> Result<()> {
    let config_path = ".cargo-smith";
    
    // 1. Load Config
    if !Path::new(config_path).exists() {
        bail!("Not a cargo-smith project. Run `cargo-smith new` first.");
    }
    let config_content = fs::read_to_string(config_path).await?;
    let mut config: CargoSmith = toml::from_str(&config_content)?;

    // 2. Check if feature exists in config
    let feature_name = args.name.to_lowercase();
    if config.generated.features.contains(&feature_name) {
        bail!("Feature '{}' already exists in .cargo-smith", feature_name);
    }

    let template_str = config.metadata.template.clone();

    let template_type = template_str.parse()
        .context(format!("Failed to parse template type: {}", template_str))?;

    TemplateType::create(&template_type)
        .add_feature(&feature_name)
        .await?;

    config.generated.features.push(feature_name);
    fs::write(config_path, toml::to_string_pretty(&config)?).await?;

    println!("Feature created and registered successfully!");
    Ok(())
}


// Someday I will do some routing framework os something to make it really well and useful