use crate::utils::remote::Fetcher;

use super::GITHUB_LICENSES_API;

#[derive(clap::Args, Debug)]
pub struct PreviewArgs {
    /// License ID (e.g. mit, apache-2.0)
    #[arg(value_name = "LICENSE")]
    pub id: String,

    /// Show description
    #[arg(long, short = 'd')]
    pub description: bool,

    /// Show permissions
    #[arg(long, short = 'p')]
    pub permissions: bool,

    /// Show limitations
    #[arg(long, short = 'l')]
    pub limitations: bool,

    /// Show conditions
    #[arg(long, short = 'c')]
    pub conditions: bool,

    /// Show all details
    #[arg(long, short = 'a')]
    pub details: bool,
}

impl super::Runnable for PreviewArgs {
    fn run(&self) -> anyhow::Result<()> {
        let fetcher = Fetcher::new();
        let url = format!("{}/{}", GITHUB_LICENSES_API, self.id.to_lowercase());
        let license_data = fetcher.fetch_json(&url)?;

        let name = license_data
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("Unknown License");

        println!("\x1b[36mLicense Name:\x1b[0m {}\n", name);

        // If no flags, show full license
        if !self.description
            && !self.permissions
            && !self.limitations
            && !self.conditions
            && !self.details
        {
            show_full_license(&license_data)?;
            return Ok(());
        }

        if self.description {
            show_description(&license_data)?;
        }
        if self.permissions {
            show_permissions(&license_data)?;
        }
        if self.limitations {
            show_limitations(&license_data)?;
        }
        if self.conditions {
            show_conditions(&license_data)?;
        }
        if self.details {
            show_all_details(&license_data)?;
        }

        Ok(())
    }
}

fn show_description(license_data: &serde_json::Value) -> anyhow::Result<()> {
    if let Some(desc) = license_data.get("description").and_then(|d| d.as_str()) {
        println!("\x1b[36mDescription:\x1b[0m");
        println!("{}", desc);
        println!();
    }
    Ok(())
}

fn show_permissions(license_data: &serde_json::Value) -> anyhow::Result<()> {
    if let Some(perms) = license_data.get("permissions").and_then(|p| p.as_array()) {
        println!("\x1b[32mPermissions:\x1b[0m");
        for perm in perms {
            if let Some(perm_str) = perm.as_str() {
                println!("  ✓ {}", format_permission(perm_str));
            }
        }
        println!();
    }
    Ok(())
}

fn show_limitations(license_data: &serde_json::Value) -> anyhow::Result<()> {
    if let Some(limits) = license_data.get("limitations").and_then(|l| l.as_array()) {
        println!("\x1b[31mLimitations:\x1b[0m");
        for limit in limits {
            if let Some(limit_str) = limit.as_str() {
                println!("  ✗ {}", format_limitation(limit_str));
            }
        }
        println!();
    }
    Ok(())
}

fn show_conditions(license_data: &serde_json::Value) -> anyhow::Result<()> {
    if let Some(conds) = license_data.get("conditions").and_then(|c| c.as_array()) {
        println!("\x1b[33mConditions:\x1b[0m");
        for condition in conds {
            if let Some(cond_str) = condition.as_str() {
                println!("  ! {}", format_condition(cond_str));
            }
        }
        println!();
    }
    Ok(())
}

fn show_all_details(license_data: &serde_json::Value) -> anyhow::Result<()> {
    show_permissions(license_data)?;
    show_limitations(license_data)?;
    show_conditions(license_data)?;
    Ok(())
}

fn show_full_license(license_data: &serde_json::Value) -> anyhow::Result<()> {
    if let Some(body) = license_data.get("body").and_then(|b| b.as_str()) {
        println!("\x1b[36mLicense Text:\x1b[0m");
        println!("{}", body);
    }
    Ok(())
}

fn format_permission(perm: &str) -> String {
    match perm {
        "commercial-use" => "Commercial use".to_string(),
        "modifications" => "Modify".to_string(),
        "distribution" => "Distribute".to_string(),
        "patent-use" => "Patent use".to_string(),
        "private-use" => "Private use".to_string(),
        _ => perm.replace('-', " "),
    }
}

fn format_limitation(limit: &str) -> String {
    match limit {
        "liability" => "Liability".to_string(),
        "warranty" => "Warranty".to_string(),
        "trademark-use" => "Trademark use".to_string(),
        _ => limit.replace('-', " "),
    }
}

fn format_condition(cond: &str) -> String {
    match cond {
        "include-copyright" => "License and copyright notice".to_string(),
        "document-changes" => "State changes".to_string(),
        "disclose-source" => "Disclose source".to_string(),
        "same-license" => "Same license".to_string(),
        _ => cond.replace('-', " "),
    }
}
