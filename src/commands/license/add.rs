// src/commands/license/add.rs
use crate::utils::file;
use crate::utils::remote::Fetcher;
use crate::utils::template_args_parser::{TemplateArgs, parse_template_args};
use anyhow::anyhow;
use std::path::{Path, PathBuf};

use super::GITHUB_LICENSES_API;

pub fn add(args: &[String]) -> anyhow::Result<()> {
    let parsed: TemplateArgs = parse_template_args(args)?;

    if parsed.all {
        download_all_licenses(parsed.dir.as_deref())?;
    } else if parsed.names.is_empty() {
        return Err(anyhow!(
            "At least one license ID is required (or use --all)"
        ));
    } else {
        for license_id in parsed.names {
            download_single_license(&license_id, parsed.dir.as_deref())?;
        }
    }

    Ok(())
}

fn download_single_license(id: &str, dir_path: Option<&str>) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/{}", GITHUB_LICENSES_API, id.to_lowercase());

    let license_data = fetcher.fetch_json(&url)?;

    let body = license_data
        .get("body")
        .and_then(|b| b.as_str())
        .ok_or_else(|| anyhow!("License body not found for {}", id))?;

    let filename = format!("LICENSE.{}", id.to_uppercase());
    let dest_path: PathBuf = match dir_path {
        Some(dir) => Path::new(dir).join(filename),
        None => PathBuf::from(&filename),
    };

    file::save_file(body, &dest_path)?;

    println!(
        "\x1b[32m✓\x1b[0m Downloaded and added license: {}",
        dest_path.display()
    );

    Ok(())
}

fn download_all_licenses(dir_path: Option<&str>) -> anyhow::Result<()> {
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

        download_single_license(key, dir_path)?;
    }

    Ok(())
}
