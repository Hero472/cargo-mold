use crate::templates::{TemplateType, contents::modular, template_engine::TemplateEngine};
use anyhow::Result;

pub async fn run(project_name: &str) -> Result<()> {
    println!("Generating Modular Rust Template...");
    
    let folders = ["src/features", "src/server", "src/utils"];
    TemplateEngine::create_project_structure(project_name, &folders).await?;
    
    TemplateEngine::generate_common_files(project_name, &TemplateType::Modular).await?;

        let files = [
            ("src/main.rs", modular::MAIN),
            ("src/lib.rs", modular::LIB),
            ("src/server/server.rs", modular::SERVER),
        ];

        for (output_path, file_content) in files {
            TemplateEngine::generate_from_template(
                project_name,
                output_path,
                file_content,
                &TemplateType::Modular,
                true
            ).await?;
        }

        let mod_files = [
            ("src/features/mod.rs", modular::FEATURE_MOD),
            ("src/server/mod.rs", "//feature server entry\npub mod server;"),
            ("src/utils/mod.rs", "// feature utils entry"),
        ];

        TemplateEngine::generate_mod_files(project_name, &mod_files, &TemplateType::Modular).await?;

    Ok(())
}