use anyhow::Result;
use clap::Parser;
use commands::*;

mod commands;
mod vt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

// List the names of your sub commands here.
register_commands! {
    GetPackages
    Download
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.commands.run()?;

    Ok(())
}
