use std::path::Path;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use crate::utils::get_comment;
use crate::utils::pretty_print;
use crate::utils::remote::Fetcher;

const OUTPUT_BASE_PATH: &str = ".github";
const OUTPUT: &str = "ISSUE_TEMPLATE";
const GITHUB_RAW_BASE: &str =
    "https://raw.githubusercontent.com/rafaeljohn9/gh-templates/main/templates";
const GITHUB_API_BASE: &str =
    "https://api.github.com/repos/rafaeljohn9/gh-templates/contents/templates";

pub fn add(args: &[String]) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    if args.is_empty() {
        return Err(anyhow::anyhow!(
            "No issue template specified. Use --all to download all templates."
        ));
    }

    // Check if --all flag is provided
    if args.contains(&"--all".to_string()) {
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
    } else {
        // Download specified templates
        for template in args {
            let url = format!("{}/issue-templates/{}.yml", GITHUB_RAW_BASE, template);
            let dest_path = Path::new(OUTPUT_BASE_PATH)
                .join(OUTPUT)
                .join(format!("{}.yml", template));

            fetcher.fetch_to_file(&url, &dest_path)?;

            println!(
                "\x1b[32m✓\x1b[0m Downloaded and added issue template: {}",
                dest_path.display()
            );
        }
    }

    Ok(())
}

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

pub fn preview(template: &str, _extra_args: &[String]) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/issue-templates/{}.yml", GITHUB_RAW_BASE, template);

    println!("\x1b[32m✓\x1b[0m Previewing issue template: {}", template);

    let content = fetcher.fetch_content(&url)?;
    pretty_print::print_highlighted("yml", &content);
    Ok(())
}
