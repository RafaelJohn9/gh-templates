use std::path::PathBuf;
use tempfile::TempDir;

pub fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    temp_dir
}

pub fn create_git_repo(path: &PathBuf) {
    std::process::Command::new("git")
        .args(&["init"])
        .current_dir(path)
        .output()
        .unwrap();
}

pub fn assert_file_contains(file_path: &PathBuf, content: &str) {
    let file_content = std::fs::read_to_string(file_path).unwrap();
    assert!(file_content.contains(content));
}

pub fn assert_file_exists(file_path: &PathBuf) {
    assert!(file_path.exists(), "File {:?} should exist", file_path);
}
