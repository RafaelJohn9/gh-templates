use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::utils::cache::{Cache, CacheManager};
use crate::utils::file;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::{
    GITHUB_RAW_BASE, GITIGNORE_CACHE_NAME, OUTPUT, OUTPUT_BASE_PATH, ensure_gitignore_cache,
    find_template_in_cache,
};

#[derive(clap::Args, Debug)]
pub struct AddArgs {
    /// Template names to add (e.g., rust, python, global/windows)
    #[arg(value_name = "TEMPLATE")]
    pub templates: Vec<String>,

    /// Directory to save the .gitignore file
    #[arg(long, value_name = "DIR")]
    pub dir: Option<PathBuf>,

    /// Force overwrite existing .gitignore file
    #[arg(long)]
    pub force: bool,

    /// Download all available templates
    #[arg(long)]
    pub all: bool,

    /// Update the gitignore cache
    #[arg(long = "update-cache", default_value = "false")]
    pub update_cache: bool,
}

impl super::Runnable for AddArgs {
    fn run(&self) -> anyhow::Result<()> {
        let mut cache_manager = CacheManager::new()?;

        // If --update-cache is set, force update the cache before proceeding
        if self.update_cache {
            cache_manager.clear_cache(GITIGNORE_CACHE_NAME)?;
        }

        let cache: Cache<String> = ensure_gitignore_cache(&mut cache_manager)?;

        let dir = match &self.dir {
            Some(dir) => Some(dir.clone()),
            None => Some(file::find_repo_root()?),
        };

        if self.all {
            download_all_templates(dir.as_ref(), self.force, &cache)?;
        } else if self.templates.is_empty() {
            return Err(anyhow::anyhow!(
                "No gitignore template specified. Use `--all` or pass template names."
            ));
        } else {
            download_templates(&self.templates, dir.as_ref(), self.force, &cache)?;
        }

        Ok(())
    }
}

fn download_all_templates(
    dir_path: Option<&PathBuf>,
    force: bool,
    cache: &Cache<String>,
) -> Result<()> {
    println!("Fetching all gitignore templates...");

    let mut merged_content = String::new();

    for (key, rel_path_entry) in cache.entries.iter() {
        let fetcher = Fetcher::new();
        let url = format!("{}/{}", GITHUB_RAW_BASE, rel_path_entry.data);

        let msg = format!("Downloading gitignore template: {}", key);
        let pb = progress::spinner(&msg);
        let content = fetcher.fetch_content(&url)?;
        pb.set_message("Download Complete");
        pb.finish_and_clear();

        merged_content.push_str(&format!("# ===== {}.gitignore =====\n{}\n\n", key, content));
    }

    let dest_path = dir_path
        .map(|p| p.join(".gitignore"))
        .unwrap_or_else(|| Path::new(OUTPUT_BASE_PATH).join(OUTPUT).join(".gitignore"));

    file::save_file(&merged_content, &dest_path, force)?;

    println!(
        "\x1b[32m✓\x1b[0m Downloaded and merged all gitignore templates to {}",
        dest_path.display()
    );

    Ok(())
}

fn download_templates(
    templates: &[String],
    dir_path: Option<&PathBuf>,
    force: bool,
    cache: &Cache<String>,
) -> Result<()> {
    let mut merged_content = String::new();

    for template_name in templates {
        // Try to find the template in cache with different key formats
        let template_path = find_template_in_cache(template_name, cache)?;

        let fetcher = Fetcher::new();
        let url = format!("{}/{}", GITHUB_RAW_BASE, template_path);

        let msg = format!("Downloading gitignore template: {}", template_name);
        let pb = progress::spinner(&msg);
        let content = fetcher.fetch_content(&url)?;
        pb.set_message("Download Complete");
        pb.finish_and_clear();

        merged_content.push_str(&format!(
            "# ===== {}.gitignore =====\n{}\n\n",
            template_name, content
        ));
    }

    let dest_path = dir_path
        .map(|p| p.join(".gitignore"))
        .unwrap_or_else(|| Path::new(OUTPUT_BASE_PATH).join(OUTPUT).join(".gitignore"));

    file::save_file(&merged_content, &dest_path, force)?;

    println!(
        "\x1b[32m✓\x1b[0m Added gitignore templates: {}",
        templates.join(", ")
    );

    Ok(())
}
