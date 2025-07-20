// build.rs
use regex::Regex;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::process::Command;

fn main() {
    // Generate checksum of this build.rs file
    let build_rs_checksum = generate_build_rs_checksum().unwrap_or_else(|e| {
        println!("cargo:warning=Failed to generate build.rs checksum: {}", e);
        "unknown".to_string()
    });

    // Generate build time
    let build_time = chrono::Utc::now().format("%Y-%m-%d").to_string();

    // Try multiple sources for version in order of preference
    let version = env::var("APP_VERSION")
        .or_else(|_| get_latest_semantic_git_tag())
        .or_else(|_| get_git_describe())
        .unwrap_or_else(|_| env::var("CARGO_PKG_VERSION").unwrap_or("unknown".to_string()));

    println!("cargo:rustc-env=APP_VERSION={}", version);
    println!("cargo:rustc-env=BUILD_RS_CHECKSUM={}", build_rs_checksum);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rerun-if-env-changed=APP_VERSION");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/tags");
}

fn generate_build_rs_checksum() -> Result<String, Box<dyn std::error::Error>> {
    // Read the contents of build.rs
    let build_rs_path = "build.rs";
    let contents = fs::read_to_string(build_rs_path)?;

    // Create a SHA-256 hasher
    let mut hasher = Sha256::new();
    hasher.update(contents.as_bytes());
    let result = hasher.finalize();

    // Convert to hex string
    Ok(format!("{:x}", result))
}

fn get_latest_semantic_git_tag() -> Result<String, Box<dyn std::error::Error>> {
    // Get all tags
    let output = Command::new("git")
        .args(&["tag", "--list", "--sort=-version:refname"])
        .output()?;

    if !output.status.success() {
        return Err("Git tag command failed".into());
    }

    let tags_output = String::from_utf8(output.stdout)?;

    // Regex pattern for semantic versioning: v followed by major.minor.patch (no pre-release suffixes)
    let semver_pattern = Regex::new(r"^v(\d+)\.(\d+)\.(\d+)$")?;

    // Collect and parse valid semantic version tags
    let mut valid_tags: Vec<(String, (u32, u32, u32))> = Vec::new();

    for line in tags_output.lines() {
        let tag = line.trim();
        if let Some(caps) = semver_pattern.captures(tag) {
            let major: u32 = caps[1].parse()?;
            let minor: u32 = caps[2].parse()?;
            let patch: u32 = caps[3].parse()?;
            valid_tags.push((tag.to_string(), (major, minor, patch)));
        }
    }

    if valid_tags.is_empty() {
        return Err(
            "No semantic versioning tags found matching pattern v[0-9]+.[0-9]+.[0-9]+".into(),
        );
    }

    // Sort by semantic version (major, minor, patch) in descending order
    valid_tags.sort_by(|a, b| {
        let (maj_a, min_a, patch_a) = a.1;
        let (maj_b, min_b, patch_b) = b.1;

        maj_b
            .cmp(&maj_a)
            .then_with(|| min_b.cmp(&min_a))
            .then_with(|| patch_b.cmp(&patch_a))
    });

    // Return the latest version
    Ok(valid_tags[0].0.clone())
}

fn get_git_describe() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["describe", "--always", "--dirty"])
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Err("Git command failed".into())
    }
}
