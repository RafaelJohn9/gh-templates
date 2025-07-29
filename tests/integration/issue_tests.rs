use assert_cmd::Command as AssertCommand;
use predicates::prelude::*;
use std::fs;

/**
 Integration tests for the `gh-templates` issue subcommand.

This test suite covers the following scenarios:

- `test_issue_add_bug`: Verifies that adding a "bug" issue template creates the correct file with expected content.
- `test_issue_add_multiple`: Tests the addition of multiple issue templates in one command.
- `test_issue_add_with_dir`: Ensures that an issue template can be added to a specified directory.
- `test_issue_add_force_overwrite`: Tests that an existing issue template file is not overwritten unless the `--force` flag is used.
- `test_issue_add_invalid_type`: Confirms that an unknown issue template returns an appropriate error.
- `test_issue_add_unknown_argument`: Checks that an unknown argument results in an error.
- `test_issue_list`: Ensures the list command displays available issue templates.
- `test_issue_preview_bug`: Validates that the preview command displays the content of the "bug" issue template.
- `test_issue_preview_multiple`: Validates that the preview command displays the content of multiple issue templates.
- `test_issue_preview_invalid_id`: Ensures that an invalid issue template name results in an error.
- `test_issue_help_command`: Validates that the help command displays usage information for the issue subcommands.

Each test uses a temporary directory to avoid side effects and leverages `assert_cmd` and `predicates` for command-line assertions.
*/
// Import utility functions
use crate::common::test_utils::{
    assert_file_contains, assert_file_exists, create_git_repo, setup_test_env,
};

// --------     ADD COMMAND TESTS     --------

#[test]
fn test_issue_add_bug() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["issue", "add", "bug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added issue template").or(predicate::str::contains("✓")));

    assert_file_exists(&temp_path.join(".github/ISSUE_TEMPLATE/bug.yml"));
    assert_file_contains(
        &temp_path.join(".github/ISSUE_TEMPLATE/bug.yml"),
        "Bug Report",
    );
}

#[test]
fn test_issue_add_multiple() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["issue", "add", "bug", "feature"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added issue template").or(predicate::str::contains("✓")));

    assert_file_exists(&temp_path.join(".github/ISSUE_TEMPLATE/bug.yml"));
    assert_file_exists(&temp_path.join(".github/ISSUE_TEMPLATE/feature.yml"));
}

#[test]
fn test_issue_add_with_dir() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();
    let target_dir = temp_path.join("custom_dir");
    fs::create_dir_all(&target_dir).unwrap();

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["issue", "add", "bug", "--dir", target_dir.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added issue template").or(predicate::str::contains("✓")));

    assert_file_exists(&target_dir.join("bug.yml"));
    assert_file_contains(&target_dir.join("bug.yml"), "Bug Report");
}

#[test]
fn test_issue_add_force_overwrite() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let issue_path = temp_path.join(".github/ISSUE_TEMPLATE/bug.yml");
    fs::create_dir_all(issue_path.parent().unwrap()).unwrap();
    fs::write(&issue_path, "existing content").unwrap();

    // Try to add without force (should fail)
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["issue", "add", "bug"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));

    // Try with force flag (should succeed)
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["issue", "add", "bug", "--force"])
        .assert()
        .success();

    assert_file_contains(&issue_path, "Bug Report");
}

#[test]
fn test_issue_add_invalid_type() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["issue", "add", "invalid-template"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Request failed").or(predicate::str::contains("not found")),
        );
}

#[test]
fn test_issue_add_no_template() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["issue", "add"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No issue template specified"));
}

#[test]
fn test_issue_add_unknown_argument() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["issue", "add", "--unknown"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "unexpected argument '--unknown' found",
        ));
}

// --------     LIST COMMAND TEST     --------

#[test]
fn test_issue_list() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["issue", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bug"))
        .stdout(predicate::str::contains("feature"));
}

// --------     PREVIEW COMMAND TEST     --------

#[test]
fn test_issue_preview_bug() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["issue", "preview", "bug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Bug Report"));
}

#[test]
fn test_issue_preview_multiple() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["issue", "preview", "bug", "feature"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Bug Report"))
        .stdout(predicate::str::contains("Feature Request"));
}

#[test]
fn test_issue_preview_invalid_id() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["issue", "preview", "not-a-template"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Request failed").or(predicate::str::contains("not found")),
        );
}

// --------     HELP COMMAND TEST     --------

#[test]
fn test_issue_help_command() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["issue", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "The `Issue` subcommand provides functionality related to managing issue templates",
        ))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("preview"))
        .stdout(predicate::str::contains("list"));
}
