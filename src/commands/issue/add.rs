use std::path::Path;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use super::{GITHUB_API_BASE, GITHUB_RAW_BASE};
use crate::utils::remote::Fetcher;

const OUTPUT_BASE_PATH: &str = ".github";
const OUTPUT: &str = "ISSUE_TEMPLATE";

pub fn add(args: &[String]) -> anyhow::Result<()> {
    if args.is_empty() {
        return Err(anyhow::anyhow!(
            "No issue template specified. Use --all to download all templates."
        ));
    }

    for arg in args {
        match arg.as_str() {
            "--all" => download_all_templates()?,
            template_name => download_single_template(template_name)?,
        }
    }

    Ok(())
}

fn download_all_templates() -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Fetching all templates...");

    let url = format!("{}/issue-templates", GITHUB_API_BASE);
    let entries = fetcher.fetch_json(&url)?;
    let mut downloaded_templates = Vec::new();

    if let Some(array) = entries.as_array() {
        for entry in array {
            if let Some(name) = entry.get("name").and_then(|n| n.as_str()) {
                let template_name = match name.rfind('.') {
                    Some(idx) => &name[..idx],
                    None => name,
                };

                pb.set_message(format!("Downloading template: {}", template_name));

                let url = format!("{}/issue-templates/{}", GITHUB_RAW_BASE, name);
                let dest_path = Path::new(OUTPUT_BASE_PATH).join(OUTPUT).join(name);

                fetcher.fetch_to_file(&url, &dest_path)?;
                downloaded_templates.push(format!("{}.yml", template_name));
            }
        }
    }

    pb.finish_and_clear();
    println!(
        "\x1b[32m✓\x1b[0m Downloaded all issue templates to {}/{}",
        OUTPUT_BASE_PATH, OUTPUT
    );
    for template in downloaded_templates {
        println!("  \x1b[32m>\x1b[0m {}", template);
    }

    Ok(())
}

fn download_single_template(template_name: &str) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    let url = format!("{}/issue-templates/{}.yml", GITHUB_RAW_BASE, template_name);
    let dest_path = Path::new(OUTPUT_BASE_PATH)
        .join(OUTPUT)
        .join(format!("{}.yml", template_name));

    fetcher.fetch_to_file(&url, &dest_path)?;

    println!(
        "\x1b[32m✓\x1b[0m Downloaded and added issue template: {}",
        dest_path.display()
    );

    Ok(())
}
