use clap::Subcommand;

pub mod base;
pub mod gitignore;
pub mod issue;
pub mod license;
pub mod pr;

#[derive(Subcommand)]
pub enum CategoryCommand {
    #[command(subcommand)]
    Issue(issue::Command),
    #[command(subcommand)]
    License(license::Command),
    #[command(subcommand)]
    PR(pr::Command),
    #[command(subcommand)]
    Gitignore(gitignore::Command),
}

impl CategoryCommand {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            Self::Issue(cmd) => cmd.execute(),
            Self::License(cmd) => cmd.execute(),
            Self::PR(cmd) => cmd.execute(),
            Self::Gitignore(cmd) => cmd.execute(),
        }
    }
}
