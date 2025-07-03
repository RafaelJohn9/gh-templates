use crate::utils::remote::Fetcher;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use super::GITHUB_LICENSES_API;

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
