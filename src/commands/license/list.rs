use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::GITHUB_LICENSES_API;

#[derive(clap::Args)]
pub struct ListArgs {
    // You can add more options here if needed in the future
    #[arg(allow_hyphen_values = true)]
    pub args: Vec<String>,
}

impl super::Runnable for ListArgs {
    fn run(&self) -> anyhow::Result<()> {
        if self.args.is_empty() {
            return list_all_licenses();
        }

        for arg in &self.args {
            return Err(anyhow::anyhow!("Unknown argument: {}", arg));
        }

        Ok(())
    }
}

fn list_all_licenses() -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    let pb = progress::spinner("Fetching licenses list...");

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
