use crate::utils::remote::Fetcher;
use anyhow::anyhow;
use std::path::Path;

use super::GITHUB_LICENSES_API;

pub fn add(args: &[String]) -> anyhow::Result<()> {
    if args.is_empty() {
        return Err(anyhow!("At least one license ID is required"));
    }

    let fetcher = Fetcher::new();

    for id in args {
        let url = format!("{}/{}", GITHUB_LICENSES_API, id.to_lowercase());

        let license_data = fetcher.fetch_json(&url)?;

        let body = license_data
            .get("body")
            .and_then(|b| b.as_str())
            .ok_or_else(|| anyhow!("License body not found for {}", id))?;

        let filename = format!("LICENSE.{}", id.to_uppercase());
        let dest_path = Path::new(&filename);

        std::fs::write(dest_path, body)?;

        println!(
            "\x1b[32m✓\x1b[0m Downloaded and added license: {}",
            dest_path.display()
        );
    }

    Ok(())
}
