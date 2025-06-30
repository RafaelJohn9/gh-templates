use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Checks that all template files in the `templates` directory start with a comment
// appropriate for their file type. This is useful for ensuring that templates are well-documented
// and provide clear instructions or context for users who create new repositories from these templates.
#[test]
fn all_templates_start_with_comment() {
    let template_dir = Path::new("./templates");

    // Map extensions to comment symbols
    let mut comment_map = HashMap::new();
    comment_map.insert("rs", "//");
    comment_map.insert("py", "#");
    comment_map.insert("sh", "#");
    comment_map.insert("js", "//");
    comment_map.insert("ts", "//");
    comment_map.insert("html", "<!--");
    comment_map.insert("css", "/*");
    comment_map.insert("yaml", "#");
    comment_map.insert("yml", "#");
    comment_map.insert("toml", "#");
    comment_map.insert("md", "<!--");

    assert!(
        template_dir.exists(),
        "Template directory does not exist: {:?}",
        template_dir
    );

    fn check_templates_in_dir(
        dir: &Path,
        comment_map: &HashMap<&str, &str>,
        failed_files: &mut Vec<(String, String, String)>,
    ) {
        for entry in fs::read_dir(dir).expect("Failed to read templates directory") {
            let entry = entry.expect("Failed to read file entry");
            let path = entry.path();

            if path.is_dir() {
                check_templates_in_dir(&path, comment_map, failed_files);
            } else if path.is_file() {
                let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let comment_prefix = comment_map.get(extension).unwrap_or(&"#");

                let file = fs::File::open(&path).expect("Failed to open template file");
                let mut reader = BufReader::new(file);
                let mut first_line = String::new();

                reader
                    .read_line(&mut first_line)
                    .expect("Failed to read first line");

                let trimmed = first_line.trim_start();

                let starts_with_comment = trimmed.starts_with(comment_prefix);
                if !starts_with_comment {
                    failed_files.push((
                        path.display().to_string(),
                        comment_prefix.to_string(),
                        first_line.clone(),
                    ));
                }
            }
        }
    }

    let mut failed_files = Vec::new();
    check_templates_in_dir(template_dir, &comment_map, &mut failed_files);

    if !failed_files.is_empty() {
        println!("\n❌ The following files do not start with the expected comment:");
        for (file, expected, first_line) in &failed_files {
            println!(
                "🔺 File: {}\n  Expected prefix: {:?}\n  First line: {}\n",
                file,
                expected,
                first_line.trim_end()
            );
        }
    }

    assert!(
        failed_files.is_empty(),
        "Some template files do not start with the expected comment. See output above."
    );
}
