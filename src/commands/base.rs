use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum TemplateCategory {
    Issue,
    License,
    PR, // pull request
    Gitignore,
}

pub trait Runnable {
    fn run(&self) -> anyhow::Result<()>;
}
