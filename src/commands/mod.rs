use clap::Subcommand;

pub mod base;
pub mod gitignore;
pub mod issue;
pub mod license;
pub mod pr;

#[derive(Subcommand)]
pub enum CategoryCommand {
    #[command(subcommand)]
    /// The `Issue` subcommand provides functionality related to managing issue templates.
    Issue(issue::Command),

    #[command(subcommand)]
    /// The `License` subcommand provides functionality related to managing license templates.
    License(license::Command),

    #[command(subcommand)]
    /// The `PR` subcommand provides functionality related to managing pull request templates.
    PR(pr::Command),

    #[command(subcommand)]
    /// The `Gitignore` subcommand provides functionality related to managing `.gitignore` templates.
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
