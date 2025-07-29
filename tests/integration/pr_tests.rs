use assert_cmd::Command as AssertCommand;
use std::fs;

/**
Integration tests for the `gh-templates` pr subcommand.

This test suite covers the following scenarios:

- `test_pr_add_default`: Verifies that adding a default PR template creates the correct file with expected content.
- `test_pr_add_with_dir`: Ensures that a PR template can be added to a specified directory.
- `test_pr_add_force_overwrite`: Tests that an existing PR template file is not overwritten unless the `--force` flag is used.
- `test_pr_add_invalid_type`: Confirms that an unknown PR template type returns an appropriate error.
- `test_pr_add_unknown_argument`: Checks that an unknown argument results in an error.
- `test_pr_list`: Ensures the list command displays available PR templates.
- `test_pr_preview_single`: Validates that the preview command displays the content of a PR template.
- `test_pr_preview_invalid_id`: Ensures that an invalid PR template ID results in an error.
- `test_pr_help_command`: Validates that the help command displays usage information for the pr subcommands.

Each test uses a temporary directory to avoid side effects and leverages `assert_cmd` and `predicates` for command-line assertions.
*/
use predicates::prelude::*;

// Import utility functions
use crate::common::test_utils::{
    assert_file_contains, assert_file_exists, create_git_repo, setup_test_env,
};

// --------     ADD COMMAND TESTS     --------

#[test]
fn test_pr_add_default() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["pr", "add", "default"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            ".github/pull_request_template.md - has been added.",
        ));

    assert_file_exists(&temp_path.join(".github/pull_request_template.md"));
}

#[test]
fn test_pr_add_with_dir() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();
    let target_dir = temp_path.join("custom_dir");
    fs::create_dir_all(&target_dir).unwrap();

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&[
        "pr",
        "add",
        "default",
        "--dir",
        target_dir.to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains(format!(
        "pull_request_template.md - has been added.",
    )));

    assert_file_exists(&target_dir.join("./pull_request_template.md"));
}

#[test]
fn test_pr_add_force_overwrite() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let pr_template_path = temp_path.join(".github/pull_request_template.md");
    fs::create_dir_all(pr_template_path.parent().unwrap()).unwrap();
    fs::write(&pr_template_path, "existing content").unwrap();

    // Try to add without force (should fail)
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["pr", "add", "default"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
    cmd.current_dir(&temp_path);

    // Try with force flag (should succeed)
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["pr", "add", "default", "--force"])
        .assert()
        .success();

    let content = fs::read_to_string(&pr_template_path).unwrap();
    assert!(!content.is_empty());
}

#[test]
fn test_pr_add_invalid_type() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["pr", "add", "invalid-template"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Not Found"));
}

#[test]
fn test_pr_add_unknown_argument() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["pr", "add", "--unknown"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "unexpected argument '--unknown' found",
        ));
}

// --------     LIST COMMAND TESTS     --------

#[test]
fn test_pr_list() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["pr", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("default.md"));
}

// --------     PREVIEW COMMAND TESTS     --------

#[test]
fn test_pr_preview_single() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["pr", "preview", "default"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(".+").unwrap());
}

#[test]
fn test_pr_preview_invalid_id() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["pr", "preview", "not-a-template"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("No issue template specified")
                .or(predicate::str::contains("Not Found")),
        );
}

// --------     HELP COMMAND TEST     --------

#[test]
fn test_pr_help_command() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["pr", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PR"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("preview"))
        .stdout(predicate::str::contains("list"));
}
