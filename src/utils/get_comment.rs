// extracts first line comment from a file content based on its extension
pub fn extract_comment(content: &str, ext: &str) -> Option<String> {
    match ext {
        "rs" | "c" | "cpp" | "js" | "ts" | "java" => {
            // Handle block comments /* ... */
            if let Some(start) = content.find("/*") {
                if let Some(end) = content.find("*/") {
                    let comment = &content[start + 2..end];
                    return Some(comment.trim().to_string());
                }
            }
            // Handle line comments //
            if let Some(start) = content.find("//") {
                let comment = &content[start + 2..];
                return Some(comment.trim().to_string());
            }
            None
        }
        "yml" | "py" | "sh" | "rb" => {
            // Handle line comments #
            if let Some(start) = content.find('#') {
                let comment = &content[start + 1..];
                return Some(comment.trim().to_string());
            }
            None
        }
        "md" | "html" => {
            // Handle HTML comments <!-- ... -->
            if let Some(start) = content.find("<!--") {
                if let Some(end) = content.find("-->") {
                    let comment = &content[start + 4..end];
                    return Some(comment.trim().to_string());
                }
            }
            None
        }
        _ => None,
    }
}
