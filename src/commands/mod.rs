pub mod issue;

pub fn dispatch_add(category: &str, template: &str) -> anyhow::Result<()> {
    match category {
        "issue" => issue::add(template),
        _ => Err(anyhow::anyhow!("Unknown category: {}", category)),
    }
}

pub fn dispatch_list(category: &str) -> anyhow::Result<()> {
    match category {
        "issue" => issue::list(),
        _ => Err(anyhow::anyhow!("Unknown category: {}", category)),
    }
}

pub fn dispatch_preview(category: &str, template: &str) -> anyhow::Result<()> {
    match category {
        "issue" => issue::preview(template),
        _ => Err(anyhow::anyhow!("Unknown category: {}", category)),
    }
}
