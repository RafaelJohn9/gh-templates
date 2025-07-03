pub mod issue;
pub mod license;

pub fn dispatch_add(category: &str, args: &[String]) -> anyhow::Result<()> {
    match category {
        "issue" => issue::add(args),
        "license" => license::add(args),
        _ => Err(anyhow::anyhow!("Unknown category: {}", category)),
    }
}

pub fn dispatch_list(category: &str, extra_args: &[String]) -> anyhow::Result<()> {
    match category {
        "issue" => issue::list(extra_args),
        "license" => license::list(extra_args),
        _ => Err(anyhow::anyhow!("Unknown category: {}", category)),
    }
}

pub fn dispatch_preview(
    category: &str,
    template: &str,
    extra_args: &[String],
) -> anyhow::Result<()> {
    match category {
        "issue" => issue::preview(template, extra_args),
        "license" => license::preview(template, extra_args),
        _ => Err(anyhow::anyhow!("Unknown category: {}", category)),
    }
}
