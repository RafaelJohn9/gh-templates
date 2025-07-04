use crate::utils::remote::Fetcher;
use anyhow::anyhow;

use super::GITHUB_LICENSES_API;

pub fn preview(id: &str, extra_args: &[String]) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/{}", GITHUB_LICENSES_API, id.to_lowercase());
    let license_data = fetcher.fetch_json(&url)?;

    let name = license_data
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("Unknown License");

    println!("\x1b[36mLicense Name:\x1b[0m {}\n", name);

    if extra_args.is_empty() {
        show_full_license(&license_data)?;
        return Ok(());
    }

    for arg in extra_args {
        let arg_str = arg.as_str();
        if arg_str.starts_with('-') && !arg_str.starts_with("--") {
            handle_combined_flags(&license_data, arg_str)?;
        } else {
            match arg_str {
                "--description" | "-d" => show_description(&license_data)?,
                "--permissions" | "-p" => show_permissions(&license_data)?,
                "--limitations" | "-l" => show_limitations(&license_data)?,
                "--conditions" | "-c" => show_conditions(&license_data)?,
                "--details" | "-a" => show_all_details(&license_data)?,
                _ => return Err(anyhow!("Unknown argument: {}", arg)),
            }
        }
    }

    Ok(())
}

fn handle_combined_flags(license_data: &serde_json::Value, arg: &str) -> anyhow::Result<()> {
    for c in arg.chars().skip(1) {
        match c {
            'd' => show_description(license_data)?,
            'p' => show_permissions(license_data)?,
            'l' => show_limitations(license_data)?,
            'c' => show_conditions(license_data)?,
            'a' => show_all_details(license_data)?,
            _ => return Err(anyhow!("Unknown flag: -{}", c)),
        }
    }
    Ok(())
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
