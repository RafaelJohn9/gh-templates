use crate::commands::{
    base::{Runnable, TemplateCategory},
    issue, license, pr,
};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct Add {
    #[arg(value_enum)]
    pub category: TemplateCategory,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,

    /// Directory to output template(s) to
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// Overwrite existing files
    #[arg(short, long)]
    pub force: bool,

    /// Add all templates in the category
    #[arg(long)]
    pub all: bool,
}

impl Runnable for Add {
    fn run(&self) -> anyhow::Result<()> {
        let request = AddTemplateRequest {
            dir: self.dir.clone(),
            args: self.args.clone(),
            force: self.force,
            all: self.all,
        };

        match self.category {
            TemplateCategory::Issue => issue::add(request),
            TemplateCategory::License => license::add(request),
            TemplateCategory::PR => pr::add(request),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddTemplateRequest {
    pub dir: Option<PathBuf>,
    pub args: Vec<String>,
    pub force: bool,
    pub all: bool,
}
