use std::path::PathBuf;

use anyhow::anyhow;

use crate::commands::base::CommonAddArgs;
use crate::utils::file;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::GITHUB_LICENSES_API;

// Command to add licenses

#[derive(clap::Args, Debug)]
pub struct AddArgs {
    #[arg(allow_hyphen_values = true)]
    pub args: Vec<String>,
}

impl super::Runnable for AddArgs {
    fn run(&self) -> anyhow::Result<()> {
        let parsed_args = parse_args(self.args.clone());

        // Determine the directory to use
        let dir = match &parsed_args.common.dir {
            Some(d) => d.clone(),
            None => file::find_repo_root()?,
        };

        if parsed_args.common.all {
            download_all_licenses(Some(&dir), parsed_args.common.force)?;
        } else if parsed_args.licenses.is_empty() {
            return Err(anyhow!(
                "At least one license ID is required (or use --all)"
            ));
        } else {
            for license_id in &parsed_args.licenses {
                download_single_license(license_id, Some(&dir), parsed_args.common.force)?;
            }
        }

        Ok(())
    }
}

// Arg parsing logic
pub struct ParsedAddArgs {
    pub common: CommonAddArgs,
    pub licenses: Vec<String>,
}

fn parse_args(args: Vec<String>) -> ParsedAddArgs {
    let mut dir = None;
    let mut force = false;
    let mut all = false;
    let mut licenses = Vec::new();

    for arg in &args {
        if arg == "--all" {
            all = true;
        } else if arg.starts_with("--dir=") {
            dir = Some(PathBuf::from(&arg[6..]));
        } else if arg == "--force" {
            force = true;
        } else {
            licenses.push(arg.clone());
        }
    }

    ParsedAddArgs {
        common: CommonAddArgs { dir, force, all },
        licenses,
    }
}

// Helper functions

fn download_single_license(
    id: &str,
    dir_path: Option<&PathBuf>,
    force: bool,
) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/{}", GITHUB_LICENSES_API, id.to_lowercase());

    let msg = format!("Fetching license: {}", id);
    let pb = progress::spinner(msg.as_str());
    let license_data = fetcher.fetch_json(&url)?;
    pb.set_message("Successfully fetched license data");
    pb.finish_and_clear();

    let body = license_data
        .get("body")
        .and_then(|b| b.as_str())
        .ok_or_else(|| anyhow!("License body not found for {}", id))?;

    let filename = format!("LICENSE.{}", id.to_uppercase());
    let dest_path: PathBuf = match dir_path {
        Some(dir) => dir.join(filename),
        None => PathBuf::from(&filename),
    };

    file::save_file(body, &dest_path, force)?;

    println!(
        "\x1b[32m✓\x1b[0m Downloaded and added license: {}",
        dest_path.display()
    );

    Ok(())
}

fn download_all_licenses(dir_path: Option<&PathBuf>, force: bool) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let licenses_data = fetcher.fetch_json(GITHUB_LICENSES_API)?;

    let licenses = licenses_data
        .as_array()
        .ok_or_else(|| anyhow!("Failed to parse licenses list"))?;

    for license in licenses {
        let key = license
            .get("key")
            .and_then(|k| k.as_str())
            .ok_or_else(|| anyhow!("License key not found"))?;

        download_single_license(key, dir_path, force)?;
    }

    Ok(())
}
