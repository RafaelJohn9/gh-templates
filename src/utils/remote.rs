use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

use crate::utils::get_comment;
use crate::utils::pretty_print;

const REMOTE_BASE_URL: &str =
    "https://raw.githubusercontent.com/rafaeljohn9/gh-templates/main/templates";
const REMOTE_API_URL: &str =
    "https://api.github.com/repos/rafaeljohn9/gh-templates/contents/templates";

pub fn fetch_template(
    category: &str,
    template: &str,
    extension: &str,
    output_base_path: &Path,
    output_path: &Path,
) -> anyhow::Result<()> {
    let url = format!(
        "{}/{}-templates/{}.{}",
        REMOTE_BASE_URL, category, template, extension
    );

    let response = match reqwest::blocking::get(&url) {
        Ok(resp) => resp,
        Err(e) if e.is_connect() => {
            return Err(anyhow::anyhow!(
                "Network connection error while fetching template: {category}/{template}.{extension} from {url}"
            ));
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to GET from: {url} ({e})"));
        }
    };

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Template not found: {category}/{template}.{extension}"
        ));
    }

    let content = response.text()?;

    // Prepend output_base_path to the output_path
    let full_output_path = output_base_path.join(output_path);

    if let Some(parent) = full_output_path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(&full_output_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn fetch_template_list(category: &str) -> anyhow::Result<Vec<(String, Option<String>)>> {
    let url = format!("{}/{}-templates", REMOTE_API_URL, category);

    let client = reqwest::blocking::Client::new();
    let response = match client
        .get(&url)
        .header("User-Agent", "gh-templates-fetcher")
        .send()
    {
        Ok(resp) => resp,
        Err(e) if e.is_connect() => {
            return Err(anyhow::anyhow!(
                "Network connection error while fetching template list for category: {category} from {url}"
            ));
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to GET from: {url} ({e})"));
        }
    };

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to fetch template list for category: {category}"
        ));
    }

    let entries: serde_json::Value = response.json()?;
    let mut templates = Vec::new();

    if let Some(array) = entries.as_array() {
        for entry in array {
            if let Some(name) = entry.get("name").and_then(|n| n.as_str()) {
                // Remove the extension if present
                let (name_without_ext, extension) = match name.rfind('.') {
                    Some(idx) => (&name[..idx], &name[idx + 1..]),
                    None => (name, ""),
                };

                // Fetch the template file to read the first line (comment)
                let file_url = format!(
                    "{}/{}-templates/{}.{}",
                    REMOTE_BASE_URL, category, name_without_ext, extension
                );

                let comment = match reqwest::blocking::get(&file_url) {
                    Ok(response) if response.status().is_success() => {
                        if let Ok(text) = response.text() {
                            if let Some(first_line) = text.lines().next() {
                                Some(get_comment::extract_comment(first_line, extension))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                templates.push((name_without_ext.to_string(), comment.flatten()));
            }
        }
    }

    Ok(templates)
}

pub fn fetch_template_preview(
    category: &str,
    template: &str,
    extension: &str,
) -> anyhow::Result<()> {
    let url = format!(
        "{}/{}-templates/{}.{}",
        REMOTE_BASE_URL, category, template, extension
    );

    let response = match reqwest::blocking::get(&url) {
        Ok(resp) => resp,
        Err(e) if e.is_connect() => {
            return Err(anyhow::anyhow!(
                "Network connection error while fetching template preview: {category}/{template}.{extension} from {url}"
            ));
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to GET from: {url} ({e})"));
        }
    };

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Template not found: {category}/{template}.{extension}"
        ));
    }

    let content = response.text()?;
    pretty_print::print_highlighted(extension, &content);
    Ok(())
}
