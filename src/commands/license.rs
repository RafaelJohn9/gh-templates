use crate::utils::remote::Fetcher;
use anyhow::anyhow;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Duration;

const GITHUB_LICENSES_API: &str = "https://api.github.com/licenses";

pub fn add(id: &str, _extra_args: &[String]) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/{}", GITHUB_LICENSES_API, id.to_lowercase());

    let license_data = fetcher.fetch_json(&url)?;

    let body = license_data
        .get("body")
        .and_then(|b| b.as_str())
        .ok_or_else(|| anyhow!("License body not found"))?;

    let filename = format!("LICENSE.{}", id.to_uppercase());
    let dest_path = Path::new(&filename);

    std::fs::write(dest_path, body)?;

    println!(
        "\x1b[32m✓\x1b[0m Downloaded and added license: {}",
        dest_path.display()
    );
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
    pb.set_message("Fetching license list...");

    let licenses = fetcher.fetch_json(GITHUB_LICENSES_API)?;

    pb.finish_with_message("Successfully fetched licenses");

    if let Some(array) = licenses.as_array() {
        println!("\x1b[32m✓\x1b[0m Available licenses:");
        for license in array {
            if let (Some(key), Some(name)) = (
                license.get("key").and_then(|k| k.as_str()),
                license.get("name").and_then(|n| n.as_str()),
            ) {
                println!("  \x1b[32m>\x1b[0m {:<15} {}", key, name);
            }
        }
    } else {
        println!("No licenses found.");
    }

    Ok(())
}

pub fn preview(id: &str, extra_args: &[String]) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/{}", GITHUB_LICENSES_API, id.to_lowercase());

    let license_data = fetcher.fetch_json(&url)?;

    let name = license_data
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("Unknown License");

    println!("\x1b[36mLicense Name:\x1b[0m {}\n", name);

    // Parse extra arguments
    let mut description = false;
    let mut permissions = false;
    let mut limitations = false;
    let mut conditions = false;
    let mut details = false;

    for arg in extra_args {
        let arg_str = arg.as_str();
        if arg_str.starts_with('-') && !arg_str.starts_with("--") {
            // Handle combined short flags like -pdl
            for c in arg_str.chars().skip(1) {
                match c {
                    'd' => description = true,
                    'p' => permissions = true,
                    'l' => limitations = true,
                    'c' => conditions = true,
                    'a' => details = true,
                    _ => {
                        return Err(anyhow!("Unknown flag: -{}", c));
                    }
                }
            }
        } else {
            match arg_str {
                "--description" | "-d" => description = true,
                "--permissions" | "-p" => permissions = true,
                "--limitations" | "-l" => limitations = true,
                "--conditions" | "-c" => conditions = true,
                "--details" | "-a" => details = true,
                _ => {
                    return Err(anyhow!("Unknown argument: {}", arg));
                }
            }
        }
    }

    if description {
        if let Some(desc) = license_data.get("description").and_then(|d| d.as_str()) {
            println!("\x1b[36mDescription:\x1b[0m");
            println!("{}", desc);
            println!();
        }
    }

    if permissions || details {
        if let Some(perms) = license_data.get("permissions").and_then(|p| p.as_array()) {
            println!("\x1b[32mPermissions:\x1b[0m");
            for perm in perms {
                if let Some(perm_str) = perm.as_str() {
                    println!("  ✓ {}", format_permission(perm_str));
                }
            }
            println!();
        }
    }

    if limitations || details {
        if let Some(limits) = license_data.get("limitations").and_then(|l| l.as_array()) {
            println!("\x1b[31mLimitations:\x1b[0m");
            for limit in limits {
                if let Some(limit_str) = limit.as_str() {
                    println!("  ✗ {}", format_limitation(limit_str));
                }
            }
            println!();
        }
    }

    if conditions || details {
        if let Some(conds) = license_data.get("conditions").and_then(|c| c.as_array()) {
            println!("\x1b[33mConditions:\x1b[0m");
            for condition in conds {
                if let Some(cond_str) = condition.as_str() {
                    println!("  ! {}", format_condition(cond_str));
                }
            }
            println!();
        }
    }

    // If no specific flags are set, show the full license text
    if !permissions && !limitations && !conditions && !description && !details {
        if let Some(body) = license_data.get("body").and_then(|b| b.as_str()) {
            println!("\x1b[36mLicense Text:\x1b[0m");
            println!("{}", body);
        }
    }

    Ok(())
}

fn format_permission(perm: &str) -> String {
    match perm {
        "commercial-use" => "Commercial use".to_string(),
        "modifications" => "Modify".to_string(),
        "distribution" => "Distribute".to_string(),
        "patent-use" => "Patent use".to_string(),
        "private-use" => "Private use".to_string(),
        _ => perm.replace('-', " "),
    }
}

fn format_limitation(limit: &str) -> String {
    match limit {
        "liability" => "Liability".to_string(),
        "warranty" => "Warranty".to_string(),
        "trademark-use" => "Trademark use".to_string(),
        _ => limit.replace('-', " "),
    }
}

fn format_condition(cond: &str) -> String {
    match cond {
        "include-copyright" => "License and copyright notice".to_string(),
        "document-changes" => "State changes".to_string(),
        "disclose-source" => "Disclose source".to_string(),
        "same-license" => "Same license".to_string(),
        _ => cond.replace('-', " "),
    }
}
