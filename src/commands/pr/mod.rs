use clap::Subcommand;

use crate::commands::base::Runnable;

mod add;
mod list;
mod preview;

// Global constants - these can stay in the main module file
const GITHUB_RAW_BASE: &str =
    "https://raw.githubusercontent.com/rafaeljohn9/gh-templates/main/templates";
const GITHUB_API_BASE: &str =
    "https://api.github.com/repos/rafaeljohn9/gh-templates/contents/templates";

#[derive(Subcommand)]
pub enum Command {
    Add(add::AddArgs),
    List(list::ListArgs),
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
