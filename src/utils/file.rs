use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use colored::*;

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

    // Display the resolved path relative to the repo root or current directory
    let display_path = if let Ok(repo_root) = find_repo_root() {
        match resolved_path.strip_prefix(&repo_root) {
            Ok(rel_path) => rel_path.display().to_string(),
            Err(_) => resolved_path.display().to_string(),
        }
    } else if let Ok(current_dir) = std::env::current_dir() {
        match resolved_path.strip_prefix(&current_dir) {
            Ok(rel_path) => rel_path.display().to_string(),
            Err(_) => resolved_path.display().to_string(),
        }
    } else {
        resolved_path.display().to_string()
    };

    if resolved_path.exists() && !force {
        return Err(anyhow::anyhow!(
            "File '{}' already exists. Use --force to overwrite.",
            display_path
        ));
    }

    fs::write(resolved_path, content)?;

    println!("{} {} - has been added.", "✓".green(), display_path);
    Ok(())
}

/// Append content (including multi-line) to a file with path resolution middleware
/// By default, appends at the end of the file. If `line_position` is Some(line_num), inserts at the specified line.
pub fn append_file(content: &str, filepath: &Path, line_position: Option<usize>) -> Result<()> {
    let resolved_path = resolve_output_path(filepath)?;

    // If the file doesn't exist, redirect to save_file to handle creation and directory checks
    if !resolved_path.exists() {
        return save_file(content, filepath, false);
    }

    // Display the resolved path relative to the repo root or current directory
    let _display_path = if let Ok(repo_root) = find_repo_root() {
        match resolved_path.strip_prefix(&repo_root) {
            Ok(rel_path) => rel_path.display().to_string(),
            Err(_) => resolved_path.display().to_string(),
        }
    } else if let Ok(current_dir) = std::env::current_dir() {
        match resolved_path.strip_prefix(&current_dir) {
            Ok(rel_path) => rel_path.display().to_string(),
            Err(_) => resolved_path.display().to_string(),
        }
    } else {
        resolved_path.display().to_string()
    };

    match line_position {
        None => {
            // Simple append at the end
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&resolved_path)?;
            file.write_all(content.as_bytes())?;
        }
        Some(line_num) => {
            // Read existing content, insert at specified line, then write back
            let existing = if resolved_path.exists() {
                fs::read_to_string(&resolved_path)?
            } else {
                String::new()
            };

            let mut lines: Vec<&str> = existing.lines().collect();
            let insert_pos = line_num.min(lines.len());

            // Insert content lines at the specified position
            for (i, line) in content.lines().enumerate() {
                lines.insert(insert_pos + i, line);
            }

            // Write the modified content back to file
            fs::write(&resolved_path, lines.join("\n"))?;
        }
    }

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
                    "Not in a git repository.\nPlease specify an output directory using `--dir` or run from within a git repository."
                ));
            }
        }
    }
}
