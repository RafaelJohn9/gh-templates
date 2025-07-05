use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Save content to a file with path resolution middleware
pub fn save_file(content: &str, filepath: &Path, force: bool) -> Result<()> {
    let resolved_path = resolve_output_path(filepath)?;

    // Create parent directories only if the path starts with .github
    if filepath.starts_with(".github") {
        if let Some(parent) = resolved_path.parent() {
            fs::create_dir_all(parent)?;
        }
    } else {
        // Check if parent directory exists, error if it doesn't
        if let Some(parent) = resolved_path.parent() {
            if !parent.exists() {
                return Err(anyhow::anyhow!(
                    "Directory '{}/' does not exist. \nPlease run this command from within a git repository or create the missing dir.",
                    parent.display()
                ));
            }
        }
    }

    if resolved_path.exists() && !force {
        return Err(anyhow::anyhow!(
            "File '{}' already exists. Use --force to overwrite.",
            resolved_path.display()
        ));
    }

    fs::write(resolved_path, content)?;
    Ok(())
}

/// Middleware function to resolve the output path
/// If the path starts with .github, it finds the repo root and prepends it
fn resolve_output_path(filepath: &Path) -> Result<PathBuf> {
    if filepath.starts_with(".github") {
        // Find repository root by looking for .git directory
        let repo_root = find_repo_root()?;
        Ok(repo_root.join(filepath))
    } else {
        Ok(filepath.to_path_buf())
    }
}

/// Find the repository root by traversing up the directory tree
pub fn find_repo_root() -> Result<PathBuf> {
    let mut current_dir = std::env::current_dir()?;

    loop {
        if current_dir.join(".git").exists() {
            return Ok(current_dir);
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => {
                return Err(anyhow::anyhow!(
                    "Not in a git repository.\nPlease specify an output directory using `--dir` (To be supported soon) or run from within a git repository."
                ));
            }
        }
    }
}
