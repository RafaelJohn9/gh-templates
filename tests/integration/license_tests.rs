use assert_cmd::Command as AssertCommand;
use predicates::prelude::*;
use std::fs;

// Import utility functions
use crate::common::test_utils::{assert_file_exists, create_git_repo, setup_test_env};

/**
Integration tests for the `gh-templates` license subcommand.

This test suite covers the following scenarios:

- `test_license_add_with_params`: Verifies that adding an MIT license with parameters creates the correct file and fills in the provided values.
- `test_license_add_with_unused_param_warning`: Checks that unused parameters trigger a warning but do not prevent license creation.
- `test_license_add_interactive_mode`: Ensures that interactive mode prompts for required parameters and fills them in the license file.
- `test_license_add_update_cache_flag`: Tests that the `--update-cache` flag works when adding a license.
- `test_license_list_popular`: Ensures the list command displays popular licenses such as "mit" and "apache-2.0".
- `test_license_list_non_software`: Checks that non-software licenses are listed when the appropriate flag is used.
- `test_license_list_search_wildcard`: Validates that searching for a license by name returns matching results.
- `test_license_list_osi_approved`: Ensures that OSI-approved licenses are listed.
- `test_license_list_fsf_libre`: Checks that FSF-libre licenses are listed.
- `test_license_list_include_deprecated`: Validates that deprecated licenses are included when requested.
- `test_license_list_unknown_argument`: Confirms that an unknown argument results in an error.
- `test_license_preview_with_update_cache`: Tests that previewing a license with `--update-cache` works and displays license content.
- `test_license_preview_with_all_flags`: Ensures that all preview flags display the correct sections for a license.
- `test_license_preview_nonexistent_license`: Checks that previewing a nonexistent license returns an error message.
- `test_license_help_command`: Validates that the help command displays usage information for the license subcommands.

Each test uses a temporary directory to avoid side effects and leverages `assert_cmd` and `predicates` for command-line assertions.
*/

// --------     ADD COMMAND TESTS     --------

#[test]
fn test_license_add_with_params() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&[
        "license",
        "add",
        "mit",
        "--param",
        "year=2024",
        "--param",
        "copyright-holders=John Doe",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("added license"))
    .stdout(predicate::str::contains("Filled"));

    let content = fs::read_to_string(temp_path.join("LICENSE.MIT")).unwrap();
    assert!(content.contains("2024"));
    assert!(content.contains("John Doe"));
}

#[test]
fn test_license_add_with_unused_param_warning() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["license", "add", "mit", "--param", "unusedparam=foobar"])
        .assert()
        .success()
        .stdout(predicate::str::contains("unused parameter"));

    let license_path = temp_path.join("LICENSE.MIT");

    assert_file_exists(&license_path);
    let content = fs::read_to_string(license_path).unwrap();
    assert!(content.contains("MIT License"));
}

#[test]
fn test_license_add_interactive_mode() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    // Simulate interactive input by piping values (requires assert_cmd::Command::write_stdin)
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["license", "add", "mit", "--interactive"])
        .write_stdin("2024\nJohn Doe\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Filled"))
        .stdout(predicate::str::contains("Enter value"));

    let content = fs::read_to_string(temp_path.join("LICENSE.MIT")).unwrap();
    assert!(content.contains("2024"));
    assert!(content.contains("John Doe"));
}

#[test]
#[ignore] // it affects the cache, so it should be run manually (isolated)
fn test_license_add_update_cache_flag() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["license", "add", "mit", "--update-cache"])
        .assert()
        .success()
        .stdout(predicate::str::contains("added license"));

    assert_file_exists(&temp_path.join("LICENSE.MIT"));
}

// --------     LIST COMMAND TESTS     --------

#[test]
fn test_license_list_popular() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["license", "list", "--popular"])
        .assert()
        .success()
        .stdout(predicate::str::contains("mit"))
        .stdout(predicate::str::contains("apache-2.0"));
}

#[test]
fn test_license_list_non_software() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["license", "list", "--non-software"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Non-Software Licenses"))
        .stdout(predicate::str::contains("CC0-1.0"))
        .stdout(predicate::str::contains("OFL-1.1"))
        .stdout(predicate::str::contains("CERN-OHL-P-2.0"));
}

#[test]
fn test_license_list_search_wildcard() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["license", "list", "--search", "mit"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Licenses matching"))
        .stdout(predicate::str::contains("MIT"));
}

#[test]
fn test_license_list_osi_approved() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["license", "list", "--osi-approved"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available SPDX licenses"))
        .stdout(predicate::str::contains("MIT"));
}

#[test]
fn test_license_list_fsf_libre() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["license", "list", "--fsf-libre"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available SPDX licenses"));
}

#[test]
fn test_license_list_include_deprecated() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["license", "list", "--include-deprecated"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deprecated"));
}

#[test]
fn test_license_list_unknown_argument() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["license", "list", "--unknown"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown argument"));
}

// --------     PREVIEW COMMAND TESTS     --------

#[test]
fn test_license_preview_with_update_cache() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["license", "preview", "mit", "--update-cache"])
        .assert()
        .success()
        .stdout(predicate::str::contains("License:"))
        .stdout(predicate::str::contains("MIT License"));
}

#[test]
fn test_license_preview_with_all_flags() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&[
        "license",
        "preview",
        "mit",
        "--description",
        "--permissions",
        "--limitations",
        "--conditions",
        "--details",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Description:"))
    .stdout(predicate::str::contains("Permissions:"))
    .stdout(predicate::str::contains("Limitations:"))
    .stdout(predicate::str::contains("Conditions:"))
    .stdout(predicate::str::contains("SPDX Metadata:"));
}

#[test]
fn test_license_preview_nonexistent_license() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["license", "preview", "not-a-license"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Could not fetch license text"))
        .stdout(predicate::str::contains("404 Not Found"));
}

// --------     HELP COMMAND TEST     --------

#[test]
fn test_license_help_command() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gh-templates").unwrap();
    cmd.args(&["license", "help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "The `License` subcommand provides functionality related to managing license templates",
        ))
        .stdout(predicate::str::contains(
            "Usage: gh-templates license <COMMAND>",
        ))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("preview"))
        .stdout(predicate::str::contains("help"))
        .stdout(predicate::str::contains("-h, --help"));
}
