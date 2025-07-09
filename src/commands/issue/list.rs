use crate::utils::get_comment;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::{GITHUB_API_BASE, GITHUB_RAW_BASE};

#[derive(clap::Args)]
pub struct ListArgs {
    // You can add options here if needed in the future
}

impl super::Runnable for ListArgs {
    fn run(&self) -> anyhow::Result<()> {
        list_all_templates()
    }
}

fn list_all_templates() -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    let pb = progress::spinner("Fetching issue templates...");
    pb.set_message("Fetching template list...");

    let url = format!("{}/issue-templates", GITHUB_API_BASE);
    let entries = fetcher.fetch_json(&url)?;
    let mut templates = Vec::new();

    if let Some(array) = entries.as_array() {
        for entry in array {
            if let Some(name) = entry.get("name").and_then(|n| n.as_str()) {
                let (name_without_ext, extension) = match name.rfind('.') {
                    Some(idx) => (&name[..idx], &name[idx + 1..]),
                    None => (name, ""),
                };

                pb.set_message(format!("Reading template: {}", name_without_ext));

                let file_url = format!("{}/issue-templates/{}", GITHUB_RAW_BASE, name);
                let comment = match fetcher.fetch_content(&file_url) {
                    Ok(text) => text
                        .lines()
                        .next()
                        .and_then(|line| get_comment::extract_comment(line, extension)),
                    _ => None,
                };

                templates.push((name_without_ext.to_string(), comment));
            }
        }
    }

    pb.finish_with_message("Successfully fetched templates");

    if templates.is_empty() {
        println!("No issue templates found.");
    } else {
        println!("\x1b[32m✓\x1b[0m Available issue templates:");
        for (name, description_opt) in templates {
            match description_opt {
                Some(description) => println!("  \x1b[32m>\x1b[0m {:<12} {}", name, description),
                None => println!("  {}", name),
            }
        }
    }
    Ok(())
}
