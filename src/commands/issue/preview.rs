use super::GITHUB_RAW_BASE;
use crate::utils::pretty_print;
use crate::utils::remote::Fetcher;

pub fn preview(template: &str, _extra_args: &[String]) -> anyhow::Result<()> {
    let fetcher = Fetcher::new();
    let url = format!("{}/issue-templates/{}.yml", GITHUB_RAW_BASE, template);

    println!("\x1b[32m✓\x1b[0m Previewing issue template: {}", template);

    let content = fetcher.fetch_content(&url)?;
    pretty_print::print_highlighted("yml", &content);
    Ok(())
}
