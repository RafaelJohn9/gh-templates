// Dependencies

#[path = "common/mod.rs"]
mod common;

// Integration tests

#[path = "integration/license_tests.rs"]
mod license_tests;

#[path = "integration/gitignore_tests.rs"]
mod gitignore_tests;

#[path = "integration/issue_tests.rs"]
mod issue_tests;

#[path = "integration/pr_tests.rs"]
mod pr_tests;
