use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

/// Prints highlighted content to the terminal according to the file extension.
///
/// # Arguments
/// * `ext` - The file extension (e.g., "rs", "py").
/// * `content` - The source code/content to highlight.
pub fn print_highlighted(ext: &str, content: &str) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    // Use "txt" as the default extension if ext is empty
    let ext = if ext.is_empty() { "txt" } else { ext };

    let syntax = ps
        .find_syntax_by_extension(ext)
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, theme);

    for line in LinesWithEndings::from(content) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        print!("{}", as_24_bit_terminal_escaped(&ranges[..], false));
    }
}
