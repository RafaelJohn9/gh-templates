use crate::utils::remote::Fetcher;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use super::GITHUB_LICENSES_API;

pub fn list(args: &[String]) -> anyhow::Result<()> {
    if args.is_empty() {
        return list_all_licenses();
    }

    for arg in args {
        match arg.as_str() {
            "--all" => list_all_licenses()?,
            "--help" | "-h" => show_help()?,
            _ => return Err(anyhow::anyhow!("Unknown argument: {}", arg)),
        }
    }

    Ok(())
}

fn list_all_licenses() -> anyhow::Result<()> {
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

fn show_help() -> anyhow::Result<()> {
    println!("Usage: license list [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --all       List all available licenses (default)");
    println!("  -h, --help  Show this help message");
    Ok(())
}
