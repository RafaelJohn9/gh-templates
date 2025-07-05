use clap::Parser;
mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "gh-templates")]
#[command(about = "📦 Scaffold GitHub templates easily", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: commands::Command,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.command.execute()
}
