use super::GITHUB_RAW_BASE;
use crate::utils::pretty_print;
use crate::utils::progress;
use crate::utils::remote::Fetcher;

pub fn preview(template: &str, extra_args: &[String]) -> anyhow::Result<()> {
    if template.is_empty() && extra_args.is_empty() {
        return Err(anyhow::anyhow!("No PR template specified."));
    }

    if !template.is_empty() {
        preview_single_template(template)?;
    }

    for arg in extra_args {
        preview_single_template(arg)?;
    }

    Ok(())
}

fn preview_single_template(template: &str) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/pr-templates/{}.md", GITHUB_RAW_BASE, template);

    let pb = progress::spinner(&format!("Fetching PR template: {}", template));
    let content = fetcher.fetch_content(&url)?;
    let msg = format!("Successfully fetched PR template: {}", template);
    pb.set_message(msg);
    pb.finish_and_clear();

    pretty_print::print_highlighted("md", &content);
    Ok(())
}
