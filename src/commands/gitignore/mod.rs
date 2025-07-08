use crate::commands::base::Runnable;
use clap::Subcommand;

mod add;
mod list;
mod preview;

// Global constants - these can stay in the main module file
const GITHUB_API_BASE: &str = "https://api.github.com/repos/github/gitignore/contents";
const GITHUB_RAW_BASE: &str = "https://raw.githubusercontent.com/github/gitignore/main";
const OUTPUT_BASE_PATH: &str = ".";
const OUTPUT: &str = "gitignore_templates";
const GITIGNORE_CACHE_NAME: &str = "gitignore_templates";
const CACHE_MAX_AGE_SECONDS: u64 = 60 * 60 * 24 * 30; // 30 days

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
