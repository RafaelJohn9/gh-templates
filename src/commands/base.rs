use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum TemplateCategory {
    Issue,
    License,
    PR, // pull request
}

pub trait Runnable {
    fn run(&self) -> anyhow::Result<()>;
}
