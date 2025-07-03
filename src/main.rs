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
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },

    /// List available templates in a category
    List {
        /// Category to list (e.g., issue, pr, ci, license, gitignore)
        category: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },

    /// Preview a template in a category
    Preview {
        /// Category of the template (e.g., issue, pr, ci, license, gitignore)
        category: String,
        /// Template name to preview (e.g., bug, feature-request)
        template: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            category,
            template,
            extra_args,
        } => {
            commands::dispatch_add(&category, &template, &extra_args)?;
        }

        Commands::List {
            category,
            extra_args,
        } => {
            commands::dispatch_list(&category, &extra_args)?;
        }

        Commands::Preview {
            category,
            template,
            extra_args,
        } => {
            commands::dispatch_preview(&category, &template, &extra_args)?;
        }
    }

    Ok(())
}
