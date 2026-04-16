use clap::{Parser, Subcommand, Args};
use dialoguer::{theme::ColorfulTheme, Select, Input};

use crate::{commands::new::NewArgs, templates::TemplateType};

mod commands;
mod templates;
mod utils;

#[derive(Parser)]
#[command(name = "cargo-smith")]
#[command(version, about = "NestJS-inspired code generator for Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Run in interactive mode
    #[arg(short, long)]
    interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    New(commands::new::NewArgs),

    /// Add a new component to the project
    Add(AddActionArgs),

    /// Alias for 'add' (shortcut: g)
    #[command(name = "g", alias = "generate")]
    Generate(AddActionArgs),
}

#[derive(Args)]
pub struct AddActionArgs {
    #[command(subcommand)]
    pub command: AddCommands,
}

#[derive(Args)]
pub struct AddArgs {
    #[command(subcommand)]
    pub command: AddCommands,
}

#[derive(Subcommand)]
pub enum AddCommands {
    /// Add a new feature module (controller, service, model)
    #[command(alias = "f")]
    Feature(commands::feature::FeatureArgs),
    
    // Future expansion:
    // #[command(alias = "mw")]
    // Middleware(commands::middleware::Args),
    
    // #[command(alias = "grd")]
    // Guard(commands::guard::Args),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        // Handle the 'new' command
        Some(Commands::New(args)) => {
            // If EITHER project_name or template_type is missing, run interactive
            if args.project_name.is_none() || args.template_type.is_none() {
                create_new_project_interactive(args).await?;
            } else {
                // Both exist, run immediately
                crate::commands::new::execute(args).await?;
            }
        }

        Some(Commands::Add(args)) | Some(Commands::Generate(args)) => {
            match args.command {
                AddCommands::Feature(f_args) => {
                    crate::commands::feature::execute(f_args).await?;
                }
            }
        }

        None => {
            run_interactive().await?;
        }
    }

    Ok(())
}

async fn run_interactive() -> anyhow::Result<()> {
    let theme = ColorfulTheme::default();
    
    println!("Welcome to cargo-smith interactive mode!");
    println!("Let's generate some code step by step...\n");
    
    // Step 1: Choose action
    let actions = &["Create new project"];
    let action_choice = Select::with_theme(&theme)
        .with_prompt("What would you like to do?")
        .items(actions)
        .default(0)
        .interact()?;
    
    match action_choice {
        0 => {
            let empty_args = crate::commands::new::NewArgs {
                project_name: None,
                template_type: None,
            };
            create_new_project_interactive(empty_args).await
        },
        _ => unreachable!(),
    }
}

async fn create_new_project_interactive(args: NewArgs) -> anyhow::Result<()> {
    let theme = ColorfulTheme::default();
    
    println!("\nCreating a new project...");
    
    let project_name = match args.project_name {
        Some(name) => name,
        None => Input::with_theme(&theme)
            .with_prompt("Project name")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.is_empty() { Err("Project name cannot be empty") }
                else if input.contains(' ') { Err("No spaces allowed") }
                else { Ok(()) }
            })
            .interact_text()?
    };

    let templates = &[
        TemplateType::Modular,
        // TemplateType::Nestjs,
        // "web-api",
        // "cli-tool",
        // "library",
        // "microservice"
        ];

    let template = match args.template_type {
        Some(t) => t,
        None => {
            let choice = Select::with_theme(&theme)
                .with_prompt("Choose template type")
                .items(&*templates)
                .default(0)
                .interact()?;
            templates[choice].clone()
        }
    };
    
    println!("\nSummary:");
    println!("  Project: {}", project_name);
    println!("  Template: {}", template);
    // println!("  Features: {}", selected_features.join(", "));
    
    // Step 4: Confirm
    let confirm = dialoguer::Confirm::with_theme(&theme)
        .with_prompt("Create project with these settings?")
        .default(true)
        .interact()?;
    
    if confirm {
        let final_args = commands::new::NewArgs { 
            project_name: Some(project_name),
            template_type: Some(template),
        };
        commands::new::execute(final_args).await
    } else {
        println!("Project creation cancelled.");
        Ok(())
    }
}