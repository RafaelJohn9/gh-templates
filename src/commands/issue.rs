use std::path::Path;

use crate::utils::remote;

const OUTPUT_BASE_PATH: &str = ".github";
const OUTPUT: &str = "ISSUE_TEMPLATE";

pub fn add(template: &str) -> anyhow::Result<()> {
    let dest_path = format!("{}/{}.yml", OUTPUT, template);

    let output_path = Path::new(&dest_path);
    remote::fetch_template(
        "issue",
        template,
        "yml",
        Path::new(OUTPUT_BASE_PATH),
        output_path,
    )?;

    println!(
        "Downloaded and added issue template: {}",
        output_path.display()
    );
    Ok(())
}
pub fn list() -> anyhow::Result<()> {
    let templates = remote::fetch_template_list("issue")?;
    if templates.is_empty() {
        println!("No issue templates found.");
    } else {
        for (name, description_opt) in templates {
            match description_opt {
                Some(description) => println!("> {:<12} {}", name, description),
                None => println!("{}", name),
            }
        }
    }
    Ok(())
}

pub fn preview(template: &str) -> anyhow::Result<()> {
    crate::utils::remote::fetch_template_preview("issue", template, "yml")
}
