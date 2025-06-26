use clap::{Parser, Subcommand};
mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "gh-templates")]
#[command(about = "📦 Scaffold GitHub templates easily", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        /// Template category (e.g., issue, pr, ci, license, gitignore)
        category: String,
        /// Template name (e.g., bug, feature-request)
        template: String,
    },

    /// List available templates in a category
    List {
        /// Category to list (e.g., issue, pr, ci, license, gitignore)
        category: String,
    },

    /// Preview a template in a category
    Preview {
        /// Category of the template (e.g., issue, pr, ci, license, gitignore)
        category: String,
        /// Template name to preview (e.g., bug, feature-request)
        template: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { category, template } => {
            commands::dispatch_add(&category, &template)?;
        }

        Commands::List { category } => {
            commands::dispatch_list(&category)?;
        }

        Commands::Preview { category, template } => {
            commands::dispatch_preview(&category, &template)?;
        }
    }

    Ok(())
}
