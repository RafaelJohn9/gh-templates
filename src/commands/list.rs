use crate::commands::{
    base::{Runnable, TemplateCategory},
    issue, license, pr,
};
use clap::Args;

#[derive(Args)]
pub struct List {
    #[arg(value_enum)]
    pub category: TemplateCategory,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub extra_args: Vec<String>,
}

impl Runnable for List {
    fn run(&self) -> anyhow::Result<()> {
        match self.category {
            TemplateCategory::Issue => issue::list(&self.extra_args),
            TemplateCategory::License => license::list(&self.extra_args),
            TemplateCategory::PR => pr::list(&self.extra_args),
        }
    }
}
