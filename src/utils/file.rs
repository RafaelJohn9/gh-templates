use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

/// Save content to a file with path resolution middleware
pub fn save_file(content: &str, filepath: &Path) -> Result<()> {
    let resolved_path = resolve_output_path(filepath)?;

    // Create parent directories if they don't exist
    if let Some(parent) = resolved_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(resolved_path, content)
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
fn find_repo_root() -> Result<PathBuf> {
    let mut current_dir = std::env::current_dir()?;

    loop {
        if current_dir.join(".git").exists() {
            return Ok(current_dir);
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Not in a git repository.\nPlease specify an output directory using `--dir`  (To be supported soon) or run from within a git repository.",
                ));
            }
        }
    }
}
