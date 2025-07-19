/// Matches items in an array against a wildcard pattern
/// Supports '*' for any sequence of characters and '?' for single character
///
/// # Arguments
/// * `pattern` - The wildcard pattern to match against
/// * `items` - Array of strings to filter
///
/// # Returns
/// * `Vec<String>` - Vector containing all matching items
///
/// # Examples
/// ```
/// let items = vec!["hello.txt", "world.rs", "test.py", "hello.rs"];
/// let matches = filter_by_wildcard("*.rs", &items);
/// // Returns: ["world.rs", "hello.rs"]
/// ```
pub fn filter_by_wildcard(pattern: &str, items: &[String]) -> Vec<String> {
    items
        .iter()
        .filter(|item| matches_wildcard(pattern, item))
        .cloned()
        .collect()
}

/// Helper function to check if a string matches a wildcard pattern
fn matches_wildcard(pattern: &str, text: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    matches_recursive(&pattern_chars, &text_chars, 0, 0)
}

/// Recursive helper for wildcard matching
fn matches_recursive(pattern: &[char], text: &[char], p_idx: usize, t_idx: usize) -> bool {
    // Base cases
    if p_idx == pattern.len() && t_idx == text.len() {
        return true; // Both exhausted
    }

    if p_idx == pattern.len() {
        return false; // Pattern exhausted, text remains
    }

    match pattern[p_idx] {
        '*' => {
            // Try matching * with empty string first
            if matches_recursive(pattern, text, p_idx + 1, t_idx) {
                return true;
            }

            // Then try matching * with one or more characters
            for i in t_idx..text.len() {
                if matches_recursive(pattern, text, p_idx + 1, i + 1) {
                    return true;
                }
            }
            false
        }
        '?' => {
            // ? matches exactly one character
            if t_idx < text.len() {
                matches_recursive(pattern, text, p_idx + 1, t_idx + 1)
            } else {
                false
            }
        }
        c => {
            // Regular character match
            if t_idx < text.len() && text[t_idx] == c {
                matches_recursive(pattern, text, p_idx + 1, t_idx + 1)
            } else {
                false
            }
        }
    }
}
