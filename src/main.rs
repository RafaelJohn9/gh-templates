use clap::Parser;

mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "gh-templates")]
#[command(about = "📦 Scaffold GitHub templates easily", long_about = None)]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    category: commands::CategoryCommand,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.category.execute()
}
