use clap::Args;

#[derive(Args)]
pub struct ResourceArgs {
    pub name: String,
}

pub async fn execute(args: ResourceArgs) -> anyhow::Result<()> {
    println!("📁 Generating resource: {}", args.name);
    // We'll implement this next
    Ok(())
}
