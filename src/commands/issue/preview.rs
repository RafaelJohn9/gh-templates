use super::GITHUB_RAW_BASE;
use crate::utils::pretty_print;
use crate::utils::remote::Fetcher;

pub fn preview(template: &str, extra_args: &[String]) -> anyhow::Result<()> {
    if template.is_empty() && extra_args.is_empty() {
        return Err(anyhow::anyhow!("No issue template specified."));
    }

    if !template.is_empty() {
        preview_single_template(template)?;
    }

    for arg in extra_args {
        match arg.as_str() {
            template_name => preview_single_template(template_name)?,
        }
    }

    Ok(())
}

fn preview_single_template(template: &str) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/issue-templates/{}.yml", GITHUB_RAW_BASE, template);

    println!("\x1b[32m✓\x1b[0m Previewing issue template: {}", template);

    let content = fetcher.fetch_content(&url)?;
    pretty_print::print_highlighted("yml", &content);
    Ok(())
}
