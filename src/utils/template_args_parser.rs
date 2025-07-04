// src/utils/template_args_parser.rs
use std::collections::VecDeque;

#[derive(Debug)]
pub struct TemplateArgs {
    pub names: Vec<String>,
    pub dir: Option<String>,
    pub all: bool,
    pub force: bool,
}

pub fn parse_template_args(args: &[String]) -> anyhow::Result<TemplateArgs> {
    let mut queue: VecDeque<String> = args.iter().cloned().collect();
    let mut names = vec![];
    let mut dir = None;
    let mut all = false;
    let mut force = false;

    while let Some(token) = queue.pop_front() {
        match token.as_str() {
            "--dir" => {
                if let Some(value) = queue.pop_front() {
                    dir = Some(value);
                } else {
                    return Err(anyhow::anyhow!("Expected a directory path after `--dir`"));
                }
            }
            "--all" => {
                all = true;
            }
            "--force" | "-f" => {
                force = true;
            }
            value => {
                names.push(value.to_string());
            }
        }
    }

    Ok(TemplateArgs {
        names,
        dir,
        all,
        force,
    })
}
