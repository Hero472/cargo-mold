use crate::templates::Template;
use anyhow::Result;
use async_trait::async_trait;

// Import the logic from the sub-files
mod generate;
mod feature;

pub struct ModularTemplate;

#[async_trait]
impl Template for ModularTemplate {
    fn name(&self) -> &str { "Modular" }

    fn description(&self) -> &str {
        "Modular Rust/Actix Web structure organized by features"
    }

    // Delegate to the specialized file
    async fn generate(&self, project_name: &str) -> Result<()> {
        generate::run(project_name).await
    }

    // Delegate to the specialized file
    async fn add_feature(&self, feature_name: &str) -> Result<()> {
        feature::run(feature_name).await
    }
}