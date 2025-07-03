use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use super::{GITHUB_API_BASE, GITHUB_RAW_BASE};
use crate::utils::get_comment;
use crate::utils::remote::Fetcher;

pub fn list(_extra_args: &[String]) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
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

                // Fetch the template file to read the first line comment
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
