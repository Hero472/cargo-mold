use anyhow::Result;
use clap::Args;

use crate::templates::TemplateType;

#[derive(Args)]
pub struct NewArgs {
    pub project_name: Option<String>,
    pub template_type: Option<TemplateType>,
}

pub async fn execute(args: NewArgs) -> Result<()> {

    let project_name = args.project_name
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("Project name is required"))?;
        
    let template_type = args.template_type
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Template type is required"))?;

    println!("Creating new project: {}", project_name);

    TemplateType::create(&template_type)
        .generate(&project_name)
        .await?;

    println!("Project '{}' created successfully!", project_name);
    println!("Next steps:");
    println!("   cd {}", project_name);
    println!("   cargo run");

    Ok(())
}