// Global constants - these can stay in the main module file
const GITHUB_RAW_BASE: &str =
    "https://raw.githubusercontent.com/rafaeljohn9/gh-templates/main/templates";
const GITHUB_API_BASE: &str =
    "https://api.github.com/repos/rafaeljohn9/gh-templates/contents/templates";

// Re-export submodules
pub mod add;
pub mod list;
pub mod preview;

// Re-export the main functions for backward compatibility
pub use add::add;
pub use list::list;
pub use preview::preview;

