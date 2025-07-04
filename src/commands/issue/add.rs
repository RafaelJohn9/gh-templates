use std::path::Path;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use super::{GITHUB_API_BASE, GITHUB_RAW_BASE};
use crate::utils::remote::Fetcher;
use crate::utils::template_args_parser::{TemplateArgs, parse_template_args};

const OUTPUT_BASE_PATH: &str = ".github";
const OUTPUT: &str = "ISSUE_TEMPLATE";

pub fn add(args: &[String]) -> anyhow::Result<()> {
    let parsed: TemplateArgs = parse_template_args(args)?;

    if parsed.all {
        download_all_templates(parsed.dir.as_deref())?;
    } else if parsed.names.is_empty() {
        return Err(anyhow::anyhow!(
            "No issue template specified. Use `--all` or pass template names."
        ));
    } else {
        for template_name in parsed.names {
            download_single_template(&template_name, parsed.dir.as_deref())?;
        }
    }

    Ok(())
}
fn download_all_templates(dir_path: Option<&str>) -> anyhow::Result<()> {
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
                let default_path = Path::new(OUTPUT_BASE_PATH).join(OUTPUT);
                let base_path = match dir_path {
                    Some(path) => Path::new(path),
                    None => &default_path,
                };
                let dest_path = base_path.join(name);

                fetcher.fetch_to_file(&url, &dest_path)?;
                downloaded_templates.push(format!("{}.yml", template_name));
            }
        }
    }

    pb.finish_and_clear();
    let default_output = format!("{}/{}", OUTPUT_BASE_PATH, OUTPUT);
    let output_location = dir_path.unwrap_or(&default_output);
    println!(
        "\x1b[32m✓\x1b[0m Downloaded all issue templates to {}",
        output_location
    );
    for template in downloaded_templates {
        println!("  \x1b[32m>\x1b[0m {}", template);
    }

    Ok(())
}

fn download_single_template(template_name: &str, dir_path: Option<&str>) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    let url = format!("{}/issue-templates/{}.yml", GITHUB_RAW_BASE, template_name);
    let default_path = Path::new(OUTPUT_BASE_PATH).join(OUTPUT);
    let base_path = match dir_path {
        Some(path) => Path::new(path),
        None => &default_path,
    };
    let dest_path = base_path.join(format!("{}.yml", template_name));

    fetcher.fetch_to_file(&url, &dest_path)?;

    println!(
        "\x1b[32m✓\x1b[0m Downloaded and added issue template: {}",
        dest_path.display()
    );

    Ok(())
}
