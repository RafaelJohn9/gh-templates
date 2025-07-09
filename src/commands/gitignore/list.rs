use crate::utils::cache::{Cache, CacheManager};
use crate::utils::progress;
use crate::utils::remote::Fetcher;
use anyhow::{Context, Result};
use std::collections::HashMap;

use super::GITHUB_API_BASE;

#[derive(clap::Args)]
pub struct ListArgs {
    /// Show popular templates
    #[arg(short = 'p', long)]
    pub popular: bool,

    /// Show global templates
    #[arg(short = 'g', long)]
    pub global: bool,

    /// Show community templates
    #[arg(short = 'c', long)]
    pub community: bool,
}

impl super::Runnable for ListArgs {
    fn run(&self) -> anyhow::Result<()> {
        list_gitignore_templates(self)
    }
}

#[derive(Debug, Clone)]
pub struct GitIgnoreTemplate {
    pub name: String,
    pub path: String,
    pub category: String,
}

impl serde::Serialize for GitIgnoreTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("GitIgnoreTemplate", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("category", &self.category)?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for GitIgnoreTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct GitIgnoreTemplateVisitor;

        impl<'de> Visitor<'de> for GitIgnoreTemplateVisitor {
            type Value = GitIgnoreTemplate;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct GitIgnoreTemplate")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GitIgnoreTemplate, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut path = None;
                let mut category = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "name" => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        "path" => {
                            if path.is_some() {
                                return Err(de::Error::duplicate_field("path"));
                            }
                            path = Some(map.next_value()?);
                        }
                        "category" => {
                            if category.is_some() {
                                return Err(de::Error::duplicate_field("category"));
                            }
                            category = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<serde_json::Value>()?;
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let path = path.ok_or_else(|| de::Error::missing_field("path"))?;
                let category = category.ok_or_else(|| de::Error::missing_field("category"))?;

                Ok(GitIgnoreTemplate {
                    name,
                    path,
                    category,
                })
            }
        }

        deserializer.deserialize_struct(
            "GitIgnoreTemplate",
            &["name", "path", "category"],
            GitIgnoreTemplateVisitor,
        )
    }
}

fn list_gitignore_templates(args: &ListArgs) -> Result<()> {
    let cache_manager = CacheManager::new()?;
    let cache_stale =
        cache_manager.should_update_cache::<GitIgnoreTemplate>("gitignore", 604800)?;

    let cache: Cache<GitIgnoreTemplate> = if cache_stale {
        update_gitignore_cache(&cache_manager)?
    } else {
        cache_manager.load_cache("gitignore")?
    };

    // Filter templates based on arguments
    let templates = filter_templates(&cache, args);

    // Display results
    display_templates(templates);

    Ok(())
}

fn update_gitignore_cache(cache_manager: &CacheManager) -> Result<Cache<GitIgnoreTemplate>> {
    let fetcher = Fetcher::new();
    let pb = progress::spinner("Updating gitignore cache...");

    let mut cache: Cache<GitIgnoreTemplate> = Cache::new("1.0".to_string());

    // Fetch from different folders
    let folders = vec![
        ("", "popular"),
        ("Global", "global"),
        ("community", "community"),
    ];

    for (folder, category) in folders {
        let msg = format!("Fetching {} templates...", category);
        pb.set_message(msg);

        let templates = fetch_folder_templates(&fetcher, folder, category)?;

        for template in templates {
            let mut metadata = HashMap::new();
            metadata.insert("category".to_string(), template.category.clone());
            metadata.insert("path".to_string(), template.path.clone());

            cache.insert_with_metadata(template.name.clone(), template, metadata);
        }
    }

    // Save updated cache
    cache_manager.save_cache("gitignore", &cache)?;

    pb.finish_with_message("Cache updated successfully");
    Ok(cache)
}

fn fetch_folder_templates(
    fetcher: &Fetcher,
    folder: &str,
    category: &str,
) -> Result<Vec<GitIgnoreTemplate>> {
    let url = if folder.is_empty() {
        format!("{}/contents/", GITHUB_API_BASE)
    } else {
        format!("{}/contents/{}", GITHUB_API_BASE, folder)
    };

    let entries = fetcher
        .fetch_json(&url)
        .with_context(|| format!("Failed to fetch templates from folder: {}", folder))?;

    let mut templates = Vec::new();

    if let Some(array) = entries.as_array() {
        for entry in array {
            if entry.get("type").and_then(|t| t.as_str()) == Some("file") {
                if let Some(name) = entry.get("name").and_then(|n| n.as_str()) {
                    // Skip non-gitignore files
                    if !name.ends_with(".gitignore") {
                        continue;
                    }

                    let clean_name = name.trim_end_matches(".gitignore").to_string();
                    let path = if folder.is_empty() {
                        name.to_string()
                    } else {
                        format!("{}/{}", folder, name)
                    };

                    templates.push(GitIgnoreTemplate {
                        name: clean_name,
                        path,
                        category: category.to_string(),
                    });
                }
            }
        }
    }

    Ok(templates)
}

fn filter_templates<'a>(
    cache: &'a Cache<GitIgnoreTemplate>,
    args: &ListArgs,
) -> Vec<&'a GitIgnoreTemplate> {
    let mut templates = Vec::new();

    // If no flags are set, default to popular
    if !args.popular && !args.global && !args.community {
        templates.extend(cache.filter_by_metadata("category", "popular"));
    } else {
        if args.popular {
            templates.extend(cache.filter_by_metadata("category", "popular"));
        }
        if args.global {
            templates.extend(cache.filter_by_metadata("category", "global"));
        }
        if args.community {
            templates.extend(cache.filter_by_metadata("category", "community"));
        }
    }

    // Remove duplicates and sort
    templates.sort_by(|a, b| a.0.cmp(b.0));
    templates.dedup_by(|a, b| a.0 == b.0);

    templates
        .into_iter()
        .map(|(_, template)| template)
        .collect()
}

fn display_templates(templates: Vec<&GitIgnoreTemplate>) {
    if templates.is_empty() {
        println!("No gitignore templates found.");
        return;
    }

    println!("\x1b[32m✓\x1b[0m Available gitignore templates:");

    // Group by category for better display
    let mut by_category: HashMap<String, Vec<&GitIgnoreTemplate>> = HashMap::new();
    for template in templates {
        by_category
            .entry(template.category.clone())
            .or_insert_with(Vec::new)
            .push(template);
    }

    for (category, mut templates) in by_category {
        templates.sort_by(|a, b| a.name.cmp(&b.name));

        println!("\n\x1b[1m{}:\x1b[0m", category.to_uppercase());
        for template in templates {
            println!(
                "  \x1b[32m>\x1b[0m {:<20} ({})",
                template.name, template.path
            );
        }
    }
}
