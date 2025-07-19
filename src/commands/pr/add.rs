use std::path::{Path, PathBuf};

use crate::utils::file;
use crate::utils::manifest_navigator::ManifestNavigator;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::GITHUB_RAW_BASE;

const OUTPUT_BASE_PATH: &str = ".github";
const OUTPUT: &str = "PULL_REQUEST_TEMPLATE";

// Command to add pull request templates

#[derive(clap::Args, Debug)]
pub struct AddArgs {
    /// Template names to add (e.g., rust, python, global/windows)
    #[arg(value_name = "TEMPLATE")]
    pub templates: Vec<String>,

    /// Directory to save the pull request template file(s)
    #[arg(long, value_name = "DIR")]
    pub dir: Option<PathBuf>,

    /// Force overwrite existing pull request template file(s)
    #[arg(long)]
    pub force: bool,

    /// Download all available templates
    #[arg(long)]
    pub all: bool,

    /// Update the pull request template cache
    #[arg(long = "update-cache", default_value = "false")]
    pub update_cache: bool,
}

impl super::Runnable for AddArgs {
    fn run(&self) -> anyhow::Result<()> {
        if self.all {
            download_all_templates(self.dir.as_ref(), self.force)?;
        } else if self.templates.is_empty() {
            return Err(anyhow::anyhow!(
                "No pull request template specified. Use `--all` or pass template names."
            ));
        } else {
            for template_name in &self.templates {
                download_single_template(template_name, self.dir.as_ref(), self.force)?;
            }
        }

        Ok(())
    }
}

// Helper functions

fn download_all_templates(dir_path: Option<&PathBuf>, force: bool) -> anyhow::Result<()> {
    let manifest_url = format!("{}/pr-templates/manifest.yml", GITHUB_RAW_BASE);
    let manifest_navigator = ManifestNavigator::new(&manifest_url)?;
    let template_entries = manifest_navigator.list_entries()?;

    let mut errors = Vec::new();

    for entry in template_entries {
        // Remove extension from name for consistency
        let template_name = match entry.name.rfind('.') {
            Some(idx) => &entry.name[..idx],
            None => &entry.name,
        };

        if let Err(e) = download_single_template(template_name, dir_path, force) {
            eprintln!(
                "\x1b[31m✗\x1b[0m Failed to add template '{}': {}",
                template_name, e
            );
            errors.push((template_name.to_string(), e));
        }
    }

    let default_output = format!("{}/{}", OUTPUT_BASE_PATH, OUTPUT);
    let output_location = dir_path
        .map(|p| p.display().to_string())
        .unwrap_or(default_output);

    if errors.is_empty() {
        println!(
            "\x1b[32m✓\x1b[0m Downloaded all pull request templates to {}",
            output_location
        );
    } else {
        println!("\x1b[33m⚠\x1b[0m Some templates failed to download. See errors above.");
    }

    Ok(())
}

fn download_single_template(
    template_name: &str,
    dir_path: Option<&PathBuf>,
    force: bool,
) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    let url = format!("{}/pr-templates/{}.md", GITHUB_RAW_BASE, template_name);

    let msg = format!("Downloading pull request template: {}", template_name);
    let pb = progress::spinner(&msg);
    let content = fetcher.fetch_content(&url)?;
    pb.set_message("Download Complete");
    pb.finish_and_clear();

    let dest_path = if template_name == "default" {
        Path::new(OUTPUT_BASE_PATH).join("pull_request_template.md")
    } else {
        let default_path = Path::new(OUTPUT_BASE_PATH).join(OUTPUT);
        let base_path = dir_path.map_or(default_path.as_path(), |p| p.as_path());
        base_path.join(format!("{}.md", template_name))
    };

    file::save_file(&content, &dest_path, force)?;

    println!("\x1b[32m✓\x1b[0m {} - has beed added.", dest_path.display());

    Ok(())
}
