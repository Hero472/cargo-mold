use clap::{Parser, Subcommand};

mod commands;
mod generators;
mod templates;

#[derive(Parser)]
#[command(name = "cargo-mold")]
#[command(about = "NestJS-inspired code generator for Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    New(commands::new::NewArgs),
    /// Generate code components (shortcut: g)
    #[command(name = "g")]
    Generate(GenerateArgs),
}

// Wrapper struct for generate subcommands
#[derive(Parser)]
pub struct GenerateArgs {
    #[command(subcommand)]
    command: GenerateCommands,
}

#[derive(Subcommand)]
enum GenerateCommands {
    /// Generate a resource module
    Resource(commands::resource::ResourceArgs),
    /// Generate a service
    Service(commands::service::ServiceArgs),
    /// Generate a controller  
    Controller(commands::controller::ControllerArgs),
    /// Generate a module
    Module(commands::module::ModuleArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::New(args) => commands::new::execute(args).await,
        Commands::Generate(args) => match args.command {
            GenerateCommands::Resource(args) => commands::resource::execute(args).await,
            GenerateCommands::Service(args) => commands::service::execute(args).await,
            GenerateCommands::Controller(args) => commands::controller::execute(args).await,
            GenerateCommands::Module(args) => commands::module::execute(args).await,
        },
    }
}
