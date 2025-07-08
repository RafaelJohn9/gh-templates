use clap::ValueEnum;
use std::path::PathBuf;

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

#[derive(Debug, Clone)]
pub struct CommonAddArgs {
    pub dir: Option<PathBuf>,
    pub force: bool,
    pub all: bool,
}
