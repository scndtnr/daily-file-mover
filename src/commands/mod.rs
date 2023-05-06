mod config;
mod extract;
mod pack;

pub(super) use config::print_config;
pub(super) use pack::pack_daily_files;

use chrono::NaiveDate;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use self::config::{load_config, Config};

fn convert_to_path(path_str: &str) -> PathBuf {
    PathBuf::from_str(path_str).expect("Fail to Convert src to root directory.")
}

fn generate_new_dir_path_with_date(root_dir: &Path, date: &NaiveDate, cfg: &Config) -> PathBuf {
    let dir_path_str = date.format(&cfg.dir_path_date_format).to_string();
    root_dir.join(dir_path_str)
}

fn generate_new_file_path(new_dir_path: &Path, file_name: &str) -> PathBuf {
    new_dir_path.join(file_name)
}

fn has_date_prefix(file_name: &str, cfg: &Config) -> bool {
    let prefix_pattern = cfg
        .file_prefix_date_format
        .replace("%Y", r"\d{4}")
        .replace("%m", r"\d{2}")
        .replace("%d", r"\d{2}");
    let prefix_regex = Regex::new(&prefix_pattern).expect("Fail to generate regex.");
    prefix_regex.is_match(file_name)
}

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
    fn 日付接頭辞を持っている() {
        // 下記設定を前提とする
        // file_prefix_date_format: "%Y%m%d_"
        let file_name = "20230401_test.txt";
        let cfg = load_config();
        assert!(has_date_prefix(file_name, &cfg))
    }
    #[test]
    fn 日付接頭辞を持っていない() {
        // 下記設定を前提とする
        // file_prefix_date_format: "%Y%m%d_"
        let file_name = "test.txt";
        let cfg = load_config();
        assert!(!has_date_prefix(file_name, &cfg))
    }

    #[test]
    fn 日付接頭辞を持つのでリネームされない() {
        // 下記設定を前提とする
        // file_prefix_date_format: "%Y%m%d_"
        let new_dir_path = convert_to_path("C:\\dev\\sandbox");
        let file_name = "20230501_test.txt";
        let date = NaiveDate::parse_from_str("20230401", "%Y%m%d").unwrap();
        println!("{}", date);
        let cfg = load_config();

        // ファイルパス作成
        let new_path = generate_new_file_path_with_date(&new_dir_path, file_name, &date, &cfg);
        let expect_path = convert_to_path("C:\\dev\\sandbox\\20230501_test.txt");
        assert_eq!(new_path, expect_path);
    }

    #[test]
    fn 日付接頭辞を持たないのでリネームされる() {
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
