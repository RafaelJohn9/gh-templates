use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::commands::base;
use crate::utils::cache::{Cache, CacheManager};
use crate::utils::file;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

use super::{
    CACHE_MAX_AGE_SECONDS, GITHUB_API_BASE, GITHUB_RAW_BASE, GITIGNORE_CACHE_NAME, OUTPUT,
    OUTPUT_BASE_PATH,
};

// Command to add gitignore templates

#[derive(clap::Args, Debug)]
pub struct AddArgs {
    #[arg(allow_hyphen_values = true)]
    pub args: Vec<String>,
}

impl super::Runnable for AddArgs {
    fn run(&self) -> anyhow::Result<()> {
        let parsed_args = parse_args(self.args.clone());

        let mut cache_manager = CacheManager::new()?;
        let cache: Cache<String> = ensure_gitignore_cache(&mut cache_manager)?;

        let dir = match &parsed_args.common.dir {
            Some(dir) => Some(dir.clone()),
            None => Some(file::find_repo_root()?),
        };

        if parsed_args.common.all {
            download_all_templates(dir.as_ref(), parsed_args.common.force, &cache)?;
        } else if parsed_args.templates.is_empty() {
            return Err(anyhow::anyhow!(
                "No gitignore template specified. Use `--all` or pass template names."
            ));
        } else {
            download_templates(
                &parsed_args.templates,
                dir.as_ref(),
                parsed_args.common.force,
                &cache,
            )?;
        }

        Ok(())
    }
}

/// Arg parsing logic

pub struct ParsedAddArgs {
    pub common: base::CommonAddArgs,
    pub templates: Vec<String>,
}

fn parse_args(args: Vec<String>) -> ParsedAddArgs {
    let mut dir = None;
    let mut force = false;
    let mut all = false;
    let mut templates = Vec::new();

    for arg in &args {
        if arg == "--all" {
            all = true;
        } else if arg.starts_with("--dir=") {
            dir = Some(PathBuf::from(&arg[6..]));
        } else if arg == "--force" {
            force = true;
        } else {
            templates.push(arg.clone());
        }
    }

    ParsedAddArgs {
        common: base::CommonAddArgs { dir, force, all },
        templates,
    }
}

// Helper functions

fn download_all_templates(
    dir_path: Option<&PathBuf>,
    force: bool,
    cache: &Cache<String>,
) -> anyhow::Result<()> {
    println!("Fetching all gitignore templates...");

    let mut merged_content = String::new();

    for (key, _) in cache.entries.iter() {
        let template_name = key;
        let rel_url = cache.get(template_name).ok_or_else(|| {
            anyhow::anyhow!(
                "Template '{}' not found in cache. Try updating the cache.",
                template_name
            )
        })?;

        let fetcher = Fetcher::new();
        let url = format!("{}/{}", GITHUB_RAW_BASE, rel_url);

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

    let default_path = Path::new(OUTPUT_BASE_PATH).join(OUTPUT);
    let base_path = dir_path.map_or(default_path.as_path(), |p| p.as_path());
    let dest_path = base_path.join(".gitignore");

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
) -> anyhow::Result<()> {
    let mut merged_content = String::new();

    for template_name in templates {
        let key = template_name.to_lowercase();
        let rel_url = cache.get(&key).ok_or_else(|| {
            anyhow::anyhow!(
                "Template '{}' not found in cache. Try updating the cache.",
                template_name
            )
        })?;

        let fetcher = Fetcher::new();
        let url = format!("{}/{}", GITHUB_RAW_BASE, rel_url);

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

    let default_path = Path::new(OUTPUT_BASE_PATH).join(OUTPUT);
    let base_path = dir_path.map_or(default_path.as_path(), |p| p.as_path());
    let dest_path = base_path.join(".gitignore");

    file::save_file(&merged_content, &dest_path, force)?;

    println!(
        "\x1b[32m✓\x1b[0m Added gitignore templates: {}",
        templates.join(", ")
    );

    Ok(())
}

/// Ensures the gitignore cache exists and is up-to-date, returns it.
fn ensure_gitignore_cache(cache_manager: &mut CacheManager) -> Result<Cache<String>> {
    let should_update =
        cache_manager.should_update_cache::<String>(GITIGNORE_CACHE_NAME, CACHE_MAX_AGE_SECONDS)?;
    if !should_update {
        return cache_manager.load_cache(GITIGNORE_CACHE_NAME);
    }

    let fetcher = Fetcher::new();
    let url = format!("{}", GITHUB_API_BASE);
    let entries = fetcher.fetch_json(&url)?;

    let mut cache = Cache::new("1.0".to_string());
    if let Some(array) = entries.as_array() {
        for entry in array {
            if let Some(name) = entry.get("name").and_then(|n| n.as_str()) {
                if name.ends_with(".gitignore") {
                    let key = name[..name.len() - ".gitignore".len()].to_lowercase();
                    cache.insert(key, name.to_string());
                }
            }
        }
    }
    cache_manager.save_cache(GITIGNORE_CACHE_NAME, &cache)?;
    Ok(cache)
}
