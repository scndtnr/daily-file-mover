mod config;
mod extract;
mod pack;

pub(super) use config::print_config;
pub(super) use extract::extract_daily_files;
pub(super) use pack::pack_daily_files;

use chrono::NaiveDate;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use self::config::{load_config, Config};

/// 文字列を `PathBuf` に変換する
fn convert_to_path(path_str: &str) -> PathBuf {
    PathBuf::from_str(path_str).expect("Fail to &str to PathBuf.")
}

/// 設定情報を元に、ディレクトリパスから日付情報を生成する
fn date_from_dir_path(path: &str, cfg: &Config) -> NaiveDate {
    let date_regex = cfg.regex_dir_path_date_format();
    let date_str = date_regex
        .captures(path)
        .expect("Fail to capture by regex")
        .get(0)
        .expect("Not match by date_regex.")
        .as_str();
    NaiveDate::parse_from_str(date_str, &cfg.dir_path_date_format)
        .expect("Fail to convert date_str to NaiveDate.")
}

/// 設定情報を元に、ファイル名に日付接頭辞が含まれているか判定する
fn has_date_prefix(file_name: &str, cfg: &Config) -> bool {
    let prefix_regex = cfg.regex_file_prefix_date_format();
    prefix_regex.is_match(file_name)
}

/// 設定情報を元に、日付ディレクトリパスを作成する
fn generate_new_dir_path_with_date(root_dir: &Path, date: &NaiveDate, cfg: &Config) -> PathBuf {
    let dir_path_str = date.format(&cfg.dir_path_date_format).to_string();
    root_dir.join(dir_path_str)
}

/// 与えられたディレクトリとファイルから `PathBuf` を生成する
fn generate_new_file_path(new_dir_path: &Path, file_name: &str) -> PathBuf {
    new_dir_path.join(file_name)
}

/// 設定情報を元に、日付接頭辞を持つファイルの `PathBuf` を生成する
fn generate_new_file_path_with_date(
    new_dir_path: &Path,
    file_name: &str,
    date: &NaiveDate,
    cfg: &Config,
) -> PathBuf {
    let file_prefix_date = date.format(&cfg.file_prefix_date_format).to_string();
    // 日付接頭辞を持つファイル名にする
    let new_file_name = if has_date_prefix(file_name, cfg) {
        file_name.to_string()
    } else {
        format!("{}{}", file_prefix_date, file_name)
    };
    new_dir_path.join(new_file_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ディレクトリパスから日付情報を生成する() {
        let path = "C:\\dev\\sandbox\\2023-05\\2023-05-04";
        let cfg = load_config();

        let date = date_from_dir_path(path, &cfg);
        let expect_date = NaiveDate::parse_from_str("20230504", "%Y%m%d").unwrap();
        assert_eq!(date, expect_date);
    }

    #[test]
    fn ファイル名に日付接頭辞を持っている() {
        // 下記設定を前提とする
        // file_prefix_date_format: "%Y%m%d_"
        let file_name = "20230401_test.txt";
        let cfg = load_config();
        assert!(has_date_prefix(file_name, &cfg))
    }
    #[test]
    fn ファイル名に日付接頭辞を持っていない() {
        // 下記設定を前提とする
        // file_prefix_date_format: "%Y%m%d_"
        let file_name = "test.txt";
        let cfg = load_config();
        assert!(!has_date_prefix(file_name, &cfg))
    }

    #[test]
    fn ファイル名に日付接頭辞を持つのでリネームされない() {
        // 下記設定を前提とする
        // file_prefix_date_format: "%Y%m%d_"
        let new_dir_path = convert_to_path("C:\\dev\\sandbox");
        let file_name = "20230501_test.txt";
        let date = NaiveDate::parse_from_str("20230401", "%Y%m%d").unwrap();
        let cfg = load_config();

        // ファイルパス作成
        let new_path = generate_new_file_path_with_date(&new_dir_path, file_name, &date, &cfg);
        let expect_path = convert_to_path("C:\\dev\\sandbox\\20230501_test.txt");
        assert_eq!(new_path, expect_path);
    }

    #[test]
    fn ファイル名に日付接頭辞を持たないのでリネームされる() {
        // 下記設定を前提とする
        // file_prefix_date_format: "%Y%m%d_"
        let new_dir_path = convert_to_path("C:\\dev\\sandbox");
        let file_name = "test.txt";
        let date = NaiveDate::parse_from_str("20230401", "%Y%m%d").unwrap();
        let cfg = load_config();

        // ファイルパス作成
        let new_path = generate_new_file_path_with_date(&new_dir_path, file_name, &date, &cfg);
        let expect_path = convert_to_path("C:\\dev\\sandbox\\20230401_test.txt");
        assert_eq!(new_path, expect_path);
    }
}
