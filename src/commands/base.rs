use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum TemplateCategory {
    Issue,
    License,
}

pub trait Runnable {
    fn run(&self) -> anyhow::Result<()>;
}
