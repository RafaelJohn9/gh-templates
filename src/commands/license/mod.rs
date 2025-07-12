use clap::Subcommand;

use crate::commands::base::Runnable;

mod add;
mod list;
mod preview;

// Global constants - these can stay in the main module file
pub const GITHUB_LICENSES_API: &str = "https://api.github.com/licenses";

#[derive(Subcommand)]
pub enum Command {
    /// Add one or more licenses to the repository
    Add(add::AddArgs),
    /// List available licenses
    List(list::ListArgs),
    /// Preview a specific license
    Preview(preview::PreviewArgs),
}

impl Command {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            Command::Add(args) => args.run(),
            Command::List(args) => args.run(),
            Command::Preview(args) => args.run(),
        }
    }
}
