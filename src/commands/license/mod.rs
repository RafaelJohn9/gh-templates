// Global constants - these can stay in the main module file
pub const GITHUB_LICENSES_API: &str = "https://api.github.com/licenses";

// Re-export submodules
pub mod add;
pub mod list;
pub mod preview;

// Re-export the main functions for backward compatibility
pub use add::add;
pub use list::list;
pub use preview::preview;
