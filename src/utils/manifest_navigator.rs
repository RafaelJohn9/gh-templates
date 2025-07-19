use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// Custom error types
#[derive(Debug)]
pub enum ManifestError {
    HttpError(String),
    ParseError(String),
    NotFound(String),
    InvalidPath(String),
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ManifestError::HttpError(msg) => write!(f, "HTTP Error: {}", msg),
            ManifestError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            ManifestError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ManifestError::InvalidPath(msg) => write!(f, "Invalid Path: {}", msg),
        }
    }
}

impl Error for ManifestError {}

// Main structure for the manifest file system
pub struct ManifestNavigator {
    url: String,
    base_url: String,
    client: reqwest::blocking::Client,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub is_directory: bool,
    pub full_url: String,
}

impl ManifestNavigator {
    pub fn new(url: &str) -> Result<Self, ManifestError> {
        let url = url.trim_end_matches('/');

        // Extract base URL by removing manifest.yml from the end
        let base_url = if url.ends_with("/manifest.yml") {
            url.trim_end_matches("/manifest.yml").to_string()
        } else if url.ends_with("manifest.yml") {
            url.trim_end_matches("manifest.yml")
                .trim_end_matches('/')
                .to_string()
        } else {
            return Err(ManifestError::InvalidPath(
                "URL must point to a manifest.yml file".to_string(),
            ));
        };

        Ok(Self {
            url: url.to_string(),
            base_url,
            client: reqwest::blocking::Client::new(),
        })
    }

    /// Fetch and parse the manifest.yml file
    pub fn fetch_manifest(&self) -> Result<HashMap<String, String>, ManifestError> {
        let response = self
            .client
            .get(&self.url)
            .send()
            .map_err(|e| ManifestError::HttpError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ManifestError::NotFound(format!(
                "Manifest not found at: {}",
                self.url
            )));
        }

        let content = response
            .text()
            .map_err(|e| ManifestError::HttpError(e.to_string()))?;

        self.parse_manifest(&content)
    }

    /// Parse YAML manifest content into a HashMap
    fn parse_manifest(&self, content: &str) -> Result<HashMap<String, String>, ManifestError> {
        let mut manifest = HashMap::new();
        let mut current_section = String::new();
        let mut current_subsection = String::new();
        let mut indent_level = 0;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let leading_spaces = line.len() - line.trim_start().len();

            // Handle top-level key-value pairs
            if let Some(colon_pos) = trimmed.find(':') {
                let key = trimmed[..colon_pos].trim();
                let value = trimmed[colon_pos + 1..].trim();

                if leading_spaces == 0 {
                    // Top-level key
                    if key == "templates" {
                        current_section = key.to_string();
                        indent_level = leading_spaces;
                        continue;
                    } else if !value.is_empty() {
                        // Simple top-level key-value pair like "type: issue"
                        manifest.insert(
                            key.to_string(),
                            value.trim_matches('"').trim_matches('\'').to_string(),
                        );
                    }
                } else if leading_spaces > indent_level && current_section == "templates" {
                    // Second-level under templates
                    if value.ends_with('/') {
                        // Directory reference like "python: python/"
                        manifest.insert(format!("{}/", key), value.to_string());
                    } else if value.is_empty() {
                        // Section header like "build:" with subsections
                        current_subsection = key.to_string();
                    } else {
                        // Direct file reference
                        manifest.insert(key.to_string(), String::new());
                    }
                }
            }
            // Handle list items
            else if trimmed.starts_with("- ") {
                let filename = trimmed[2..].trim().trim_matches('"').trim_matches('\'');
                if !filename.is_empty() {
                    if current_subsection.is_empty() {
                        // Direct template list
                        manifest.insert(filename.to_string(), String::new());
                    } else {
                        // Nested under a subsection
                        let full_path = format!("{}/{}", current_subsection, filename);
                        manifest.insert(full_path, String::new());
                    }
                }
            }
        }

        if manifest.is_empty() {
            return Err(ManifestError::ParseError(
                "No valid entries found in manifest".to_string(),
            ));
        }

        Ok(manifest)
    }

    /// List all files and directories from the manifest
    pub fn list_entries(&self) -> Result<Vec<FileEntry>, ManifestError> {
        let manifest = self.fetch_manifest()?;
        let mut entries = Vec::new();

        for (key, value) in manifest {
            // Skip metadata fields like "type"
            if key == "type" {
                continue;
            }

            // Check if it's a directory
            let is_directory = key.ends_with('/') || value.ends_with('/');

            let full_url = if is_directory {
                // Remove trailing slash from key for cleaner path handling
                let clean_key = key.trim_end_matches('/');
                format!("{}/{}", self.base_url, clean_key)
            } else {
                // This is a file
                format!("{}/{}", self.base_url, key)
            };

            // Use clean name without trailing slash
            let clean_name = key.trim_end_matches('/').to_string();

            entries.push(FileEntry {
                name: clean_name.clone(),
                is_directory,
                full_url,
            });
        }

        // Sort entries: directories first, then files
        entries.sort_by(|a, b| match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        Ok(entries)
    }
}
