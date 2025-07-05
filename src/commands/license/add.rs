use crate::commands::add::AddTemplateRequest;
use crate::utils::file;
use crate::utils::progress;
use crate::utils::remote::Fetcher;
use anyhow::anyhow;
use std::path::PathBuf;

use super::GITHUB_LICENSES_API;

pub fn add(request: AddTemplateRequest) -> anyhow::Result<()> {
    // Determine the directory to use
    let dir = match &request.dir {
        Some(d) => d.clone(),
        None => file::find_repo_root()?,
    };

    if request.all {
        download_all_licenses(Some(&dir), request.force)?;
    } else if request.templates.is_empty() {
        return Err(anyhow!(
            "At least one license ID is required (or use --all)"
        ));
    } else {
        for license_id in &request.templates {
            download_single_license(license_id, Some(&dir), request.force)?;
        }
    }

    Ok(())
}

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
