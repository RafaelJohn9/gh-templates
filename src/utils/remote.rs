use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use anyhow::anyhow;
use reqwest::blocking::Client;

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .user_agent("gh-templates-fetcher")
                .build()
                .unwrap(),
        }
    }

    /// Fetch raw content from a URL
    pub fn fetch_content(&self, url: &str) -> anyhow::Result<String> {
        let response = self
            .client
            .get(url)
            .send()
            .map_err(|e| anyhow!("Failed to fetch from {}: {}", url, e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Request failed with status {}: {}",
                response.status(),
                url
            ));
        }

        response
            .text()
            .map_err(|e| anyhow!("Failed to read response: {}", e))
    }

    /// Fetch and parse JSON from a URL
    pub fn fetch_json(&self, url: &str) -> anyhow::Result<serde_json::Value> {
        let response = self
            .client
            .get(url)
            .send()
            .map_err(|e| anyhow!("Failed to fetch JSON from {}: {}", url, e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "JSON request failed with status {}: {}",
                response.status(),
                url
            ));
        }

        response
            .json()
            .map_err(|e| anyhow!("Failed to parse JSON: {}", e))
    }

    /// Fetch content from URL and save to file
    pub fn fetch_to_file(&self, url: &str, output_path: &Path) -> anyhow::Result<()> {
        let content = self.fetch_content(url)?;

        if let Some(parent) = output_path.parent() {
            create_dir_all(parent)?;
        }

        let mut file = File::create(output_path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// Download binary content and save to file
    #[allow(dead_code)]
    pub fn download_to_file(&self, url: &str, output_path: &Path) -> anyhow::Result<()> {
        let response = self
            .client
            .get(url)
            .send()
            .map_err(|e| anyhow!("Failed to download from {}: {}", url, e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Download failed with status {}: {}",
                response.status(),
                url
            ));
        }

        if let Some(parent) = output_path.parent() {
            create_dir_all(parent)?;
        }

        let bytes = response
            .bytes()
            .map_err(|e| anyhow!("Failed to read response bytes: {}", e))?;

        let mut file = File::create(output_path)?;
        file.write_all(&bytes)?;

        Ok(())
    }
}
