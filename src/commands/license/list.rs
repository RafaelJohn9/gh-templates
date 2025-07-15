use crate::utils::progress;
use crate::utils::remote::Fetcher;

// SPDX license list URL
const SPDX_LICENSE_LIST_URL: &str =
    "https://raw.githubusercontent.com/spdx/license-list-data/main/json/licenses.json";

// Common licenses with descriptions for quick reference
fn get_common_licenses() -> Vec<(&'static str, &'static str)> {
    vec![
        ("mit", "MIT License"),
        ("apache-2.0", "Apache License 2.0"),
        ("gpl-3.0", "GNU General Public License v3.0"),
        ("gpl-2.0", "GNU General Public License v2.0"),
        ("bsd-2-clause", "BSD 2-Clause \"Simplified\" License"),
        (
            "bsd-3-clause",
            "BSD 3-Clause \"New\" or \"Revised\" License",
        ),
        ("lgpl-2.1", "GNU Lesser General Public License v2.1"),
        ("lgpl-3.0", "GNU Lesser General Public License v3.0"),
        ("mpl-2.0", "Mozilla Public License 2.0"),
        ("unlicense", "The Unlicense"),
        ("cc0-1.0", "Creative Commons Zero v1.0 Universal"),
        ("agpl-3.0", "GNU Affero General Public License v3.0"),
        ("isc", "ISC License"),
        ("artistic-2.0", "Artistic License 2.0"),
        ("eupl-1.2", "European Union Public License 1.2"),
        ("bsl-1.0", "Boost Software License 1.0"),
        ("ms-pl", "Microsoft Public License"),
        ("ofl-1.1", "SIL Open Font License 1.1"),
        ("wtfpl", "Do What The F*ck You Want To Public License"),
        ("zlib", "zlib License"),
    ]
}

#[derive(clap::Args)]
pub struct ListArgs {
    /// Show only popular/common licenses
    #[arg(long, short)]
    pub popular: bool,

    /// Search for licenses containing this term
    #[arg(long, short)]
    pub search: Option<String>,

    /// Show detailed information including descriptions
    #[arg(long, short)]
    pub detailed: bool,

    /// Show deprecated licenses as well
    #[arg(long)]
    pub include_deprecated: bool,

    #[arg(allow_hyphen_values = true)]
    pub args: Vec<String>,
}

impl super::Runnable for ListArgs {
    fn run(&self) -> anyhow::Result<()> {
        // Handle any unknown arguments
        if !self.args.is_empty() {
            for arg in &self.args {
                return Err(anyhow::anyhow!("Unknown argument: {}", arg));
            }
        }

        if self.popular {
            return list_popular_licenses();
        }

        list_all_licenses(
            self.search.as_deref(),
            self.detailed,
            self.include_deprecated,
        )
    }
}

fn list_popular_licenses() -> anyhow::Result<()> {
    println!("\x1b[32m✓\x1b[0m Popular open source licenses:");
    println!();

    let common_licenses = get_common_licenses();
    for (id, name) in common_licenses {
        println!("  \x1b[32m>\x1b[0m {:<15} {}", id, name);
    }

    println!();
    println!("Use --detailed for more information or omit --popular to see all licenses.");

    Ok(())
}

fn list_all_licenses(
    search_term: Option<&str>,
    detailed: bool,
    include_deprecated: bool,
) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();

    let pb = progress::spinner("Fetching SPDX license list...");

    let licenses_data = fetcher.fetch_json(SPDX_LICENSE_LIST_URL)?;

    pb.set_message("Parsing license list...");

    let licenses = licenses_data
        .get("licenses")
        .and_then(|l| l.as_array())
        .ok_or_else(|| anyhow::anyhow!("Failed to parse SPDX licenses list"))?;

    pb.finish_with_message("Successfully fetched licenses");

    // Filter and collect licenses
    let mut filtered_licenses = Vec::new();

    for license in licenses {
        let license_id = license
            .get("licenseId")
            .and_then(|id| id.as_str())
            .unwrap_or("unknown");

        let license_name = license
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("Unknown License");

        let is_deprecated = license
            .get("isDeprecatedLicenseId")
            .and_then(|d| d.as_bool())
            .unwrap_or(false);

        // Skip deprecated licenses unless explicitly requested
        if is_deprecated && !include_deprecated {
            continue;
        }

        // Apply search filter if provided
        if let Some(search) = search_term {
            let search_lower = search.to_lowercase();
            let id_matches = license_id.to_lowercase().contains(&search_lower);
            let name_matches = license_name.to_lowercase().contains(&search_lower);

            if !id_matches && !name_matches {
                continue;
            }
        }

        filtered_licenses.push((license_id, license_name, is_deprecated, license));
    }

    // Sort by license ID for consistent output
    filtered_licenses.sort_by(|a, b| a.0.cmp(b.0));

    // Display results
    if filtered_licenses.is_empty() {
        if let Some(search) = search_term {
            println!("No licenses found matching '{}'", search);
        } else {
            println!("No licenses found");
        }
        return Ok(());
    }

    let header = if let Some(search) = search_term {
        format!("Licenses matching '{}'", search)
    } else {
        "Available SPDX licenses".to_string()
    };

    println!(
        "\x1b[32m✓\x1b[0m {} ({} found):",
        header,
        filtered_licenses.len()
    );
    println!();

    if detailed {
        display_detailed_licenses(&filtered_licenses)?;
    } else {
        display_simple_licenses(&filtered_licenses);
    }

    if !include_deprecated {
        let deprecated_count = licenses
            .iter()
            .filter(|license| {
                license
                    .get("isDeprecatedLicenseId")
                    .and_then(|d| d.as_bool())
                    .unwrap_or(false)
            })
            .count();

        if deprecated_count > 0 {
            println!();
            println!(
                "Note: {} deprecated licenses are hidden. Use --include-deprecated to show them.",
                deprecated_count
            );
        }
    }

    Ok(())
}

fn display_simple_licenses(licenses: &[(&str, &str, bool, &serde_json::Value)]) {
    for (id, name, is_deprecated, _) in licenses {
        let deprecated_marker = if *is_deprecated { " (deprecated)" } else { "" };
        println!(
            "  \x1b[32m>\x1b[0m {:<20} {}{}",
            id, name, deprecated_marker
        );
    }
}

fn display_detailed_licenses(
    licenses: &[(&str, &str, bool, &serde_json::Value)],
) -> anyhow::Result<()> {
    for (id, name, is_deprecated, license) in licenses {
        println!("  \x1b[32m>\x1b[0m \x1b[1m{}\x1b[0m", id);
        println!("    Name: {}", name);

        if *is_deprecated {
            println!("    \x1b[33mStatus: DEPRECATED\x1b[0m");
        }

        // Show additional details if available
        if let Some(reference) = license.get("reference").and_then(|r| r.as_str()) {
            println!("    URL: {}", reference);
        }

        if let Some(osi_approved) = license.get("isOsiApproved").and_then(|o| o.as_bool()) {
            if osi_approved {
                println!("    \x1b[32mOSI Approved\x1b[0m");
            }
        }

        if let Some(fsf_libre) = license.get("isFsfLibre").and_then(|f| f.as_bool()) {
            if fsf_libre {
                println!("    \x1b[32mFSF Libre\x1b[0m");
            }
        }

        println!();
    }

    Ok(())
}
