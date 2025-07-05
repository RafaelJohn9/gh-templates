pub mod add;
pub mod base;
pub mod issue;
pub mod license;
pub mod list;
pub mod pr;
pub mod preview;

use add::Add;
use base::Runnable;
use clap::Subcommand;
use list::List;
use preview::Preview;

#[derive(Subcommand)]
pub enum Command {
    Add(Add),
    List(List),
    Preview(Preview),
}

impl Command {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            Command::Add(cmd) => cmd.run(),
            Command::List(cmd) => cmd.run(),
            Command::Preview(cmd) => cmd.run(),
        }
    }
}
