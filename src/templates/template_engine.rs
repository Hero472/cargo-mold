use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use chrono::Utc;

use crate::templates::contents::common;
use crate::templates::types::CargoToml;
use crate::templates::TemplateType;
use crate::utils::to_pascal_case;

const CARGO_TOML: &str = include_str!("../../Cargo.toml");

pub struct TemplateEngine;

impl TemplateEngine {

    /// Create a project folder structure based on a list of directory paths.
    ///
    /// Example:
    /// create_project_structure("myapp", &["src/routes", "src/models"]);
    pub async fn create_project_structure(
        project_name: &str,
        dirs: &[&str],
    ) -> Result<()> {
        let base = if project_name.is_empty() {
            Path::new(".")
        } else {
            Path::new(project_name)
        };

        for dir in dirs {
            let full_path: PathBuf = base.join(dir);
            fs::create_dir_all(&full_path).await?;
        }

        Ok(())
    }

    pub async fn generate_common_files(project_name: &str, template_type: &TemplateType) -> Result<()> {
        let common_files = [
            ("Cargo.toml", common::CARGO_TOML),
            ("src/main.rs", common::MAIN),
            (".cargo-smith", common::CARGO_SMITH),
            (".env.example", common::ENV_EXAMPLE),
        ];

        for (output_path, file_content) in common_files {
            TemplateEngine::generate_from_template(
                project_name,
                output_path,
                file_content,
                template_type,
                true
            ).await?;
        }
        Ok(())
    }

    pub async fn generate_from_template(
        name: &str,
        output_path: &str,
        template_content: &str,
        template_type: &TemplateType,
        is_new_project: bool
    ) -> Result<()> {

        let config: CargoToml = toml::from_str(CARGO_TOML)?;

        let content = template_content
            .replace("{{name}}", name)
            .replace("{{name_snake_case}}", &name.replace("-", "_"))
            .replace("{{name_pascal_case}}", &to_pascal_case(&name))
            .replace("{{now}}",  &Utc::now().format("%Y-%m-%d").to_string())
            .replace("{{template_type}}", &template_type.to_string())
            .replace("{{cargo_version}}", &config.package.version);

        let full_output_path = if is_new_project {
            Path::new(name).join(output_path)
        } else {
            PathBuf::from(output_path)
        };
        
        if let Some(parent) = Path::new(&full_output_path).parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let mut file = fs::File::create(full_output_path).await?;
        file.write_all(content.as_bytes()).await?;
        
        Ok(())
    }

    // Create the mod generated files in the
    pub async fn generate_mod_files(
        project_name: &str,
        files: &[(&str, &str)],
        template: &TemplateType
    ) -> Result<()> {
        for (path, content) in files {
            TemplateEngine::generate_from_template(
                project_name,
                path,
                content,
                template,
                true
            ).await?;
        }
        Ok(())
    }

}