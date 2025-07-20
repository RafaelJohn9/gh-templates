use clap::CommandFactory;
use clap::Parser;

mod commands;
mod utils;

const BUILD_RS_CHECKSUM: &str = env!("BUILD_RS_CHECKSUM");

#[derive(Parser)]
#[command(name = "gh-templates")]
#[command(about = "📦 Scaffold GitHub templates easily", long_about = None)]
#[command(version = option_env!("APP_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")))]
struct Cli {
    #[command(subcommand)]
    category: Option<commands::CategoryCommand>,

    /// Show detailed version information
    #[arg(long = "build-info", help = "Display detailed build information")]
    build_info: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.build_info {
        print_build_info();
        return Ok(());
    }

    match cli.category {
        Some(category) => category.execute(),
        None => {
            // If no subcommand is provided, show help
            let mut cmd = Cli::command();
            cmd.print_help()?;
            Ok(())
        }
    }
}

fn print_build_info() {
    let version = option_env!("APP_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

    println!("Version: {}", version);
    println!("Build SHA256: {}", BUILD_RS_CHECKSUM);
    println!("Build Time: {}", env!("BUILD_TIME"));
}
