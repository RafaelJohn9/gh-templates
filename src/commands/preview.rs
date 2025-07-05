use crate::commands::{
    base::{Runnable, TemplateCategory},
    issue, license, pr,
};
use clap::Args;

#[derive(Args)]
pub struct Preview {
    #[arg(value_enum)]
    pub category: TemplateCategory,

    pub template: String,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub extra_args: Vec<String>,
}

impl Runnable for Preview {
    fn run(&self) -> anyhow::Result<()> {
        match self.category {
            TemplateCategory::Issue => issue::preview(&self.template, &self.extra_args),
            TemplateCategory::License => license::preview(&self.template, &self.extra_args),
            TemplateCategory::PR => pr::preview(&self.template, &self.extra_args),
        }
    }
}
