use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use std::fs;
use tempfile::TempDir;

// テスト用のヘルパー関数
fn get_binary_cmd() -> Command {
    Command::cargo_bin("daily-file-mover").expect("Failed to find binary")
}

fn create_test_config_dir() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    // テスト用の設定ディレクトリを環境変数で指定
    env::set_var("HOME", temp_dir.path());
    temp_dir
}

#[test]
fn test_config_command_displays_configuration() {
    let _temp_dir = create_test_config_dir();

    let mut cmd = get_binary_cmd();
    cmd.arg("config");

    let assert = cmd.assert();
    assert
        .success()
        .stdout(predicate::str::contains("Config File Path:"))
        .stdout(predicate::str::contains("file_prefix_date_format:"))
        .stdout(predicate::str::contains("dir_path_date_format:"))
        .stdout(predicate::str::contains("Currnet Directory:"));
}

#[test]
fn test_help_command_works() {
    let mut cmd = get_binary_cmd();
    cmd.arg("--help");

    let assert = cmd.assert();
    assert
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"));
}

#[test]
fn test_version_command_works() {
    let mut cmd = get_binary_cmd();
    cmd.arg("--version");

    let assert = cmd.assert();
    assert
        .success()
        .stdout(predicate::str::contains("Daily File Mover"));
}

#[test]
fn test_pack_command_dry_run() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let source_dir = temp_dir.path().join("source");
    let dest_dir = temp_dir.path().join("dest");

    // テストファイルの準備
    fs::create_dir_all(&source_dir).expect("Failed to create source directory");
    fs::create_dir_all(&dest_dir).expect("Failed to create dest directory");

    let test_file = source_dir.join("20231225_test.txt");
    fs::write(&test_file, "test content").expect("Failed to write test file");

    let mut cmd = get_binary_cmd();
    cmd.arg("pack")
        .arg(&source_dir)
        .arg(&dest_dir)
        .arg("--dry-run");

    let assert = cmd.assert();
    assert
        .success()
        .stdout(predicate::str::contains("PackArgs"))
        .stdout(predicate::str::contains("dry_run: true"));

    // dry-runなのでファイルは移動されていないことを確認
    assert!(
        test_file.exists(),
        "File should not be moved in dry-run mode"
    );
}

#[test]
fn test_extract_command_dry_run() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let source_dir = temp_dir.path().join("source");
    let dest_dir = temp_dir.path().join("dest");

    // 日付ディレクトリ構造を作成
    let date_dir = source_dir.join("2023-12").join("2023-12-25");
    fs::create_dir_all(&date_dir).expect("Failed to create date directory");
    fs::create_dir_all(&dest_dir).expect("Failed to create dest directory");

    let test_file = date_dir.join("20231225_test.txt");
    fs::write(&test_file, "test content").expect("Failed to write test file");

    let mut cmd = get_binary_cmd();
    cmd.arg("extract")
        .arg(&source_dir)
        .arg(&dest_dir)
        .arg("--dry-run");

    let assert = cmd.assert();
    assert
        .success()
        .stdout(predicate::str::contains("ExtractArgs"))
        .stdout(predicate::str::contains("dry_run: true"));

    // dry-runなのでファイルは移動されていないことを確認
    assert!(
        test_file.exists(),
        "File should not be moved in dry-run mode"
    );
}

#[test]
fn test_create_command_shows_arguments() {
    let mut cmd = get_binary_cmd();
    cmd.arg("create-new-daily-report")
        .arg("--date")
        .arg("2023-12-25")
        .arg("--dry-run");

    let assert = cmd.assert();
    // テンプレートファイルが存在しないためエラーになるが、引数は正しく表示される
    assert
        .failure()
        .stdout(predicate::str::contains("CreateNewDailyReportArgs"))
        .stdout(predicate::str::contains("dry_run: true"));
}

#[test]
fn test_invalid_command_returns_error() {
    let mut cmd = get_binary_cmd();
    cmd.arg("invalid-command");

    let assert = cmd.assert();
    assert.failure();
}

#[test]
fn test_pack_command_without_arguments_shows_help() {
    let mut cmd = get_binary_cmd();
    cmd.arg("pack");

    let assert = cmd.assert();
    assert
        .failure()
        .stderr(predicate::str::contains("required"));
}
