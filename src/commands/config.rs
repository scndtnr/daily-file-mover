use std::env;

use regex::Regex;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) file_prefix_date_format: String,
    pub(crate) dir_path_date_format: String,
    pub(crate) daily_report_file_name: String,
    pub(crate) base_dir_path: String,
    pub(crate) template_file_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            file_prefix_date_format: r"%Y%m%d_".into(),
            dir_path_date_format: r"%Y-%m\%Y-%m-%d".into(),
            daily_report_file_name: "report.md.txt".into(),
            base_dir_path: "".into(),
            template_file_path: "".into(),
        }
    }
}

impl Config {
    pub(super) fn regex_file_prefix_date_format(&self) -> Regex {
        let pattern = self
            .file_prefix_date_format
            .replace("%Y", r"\d{4}")
            .replace("%m", r"\d{2}")
            .replace("%d", r"\d{2}");
        Regex::new(&pattern).expect("Fail to generate regex.")
    }
    pub(super) fn regex_dir_path_date_format(&self) -> Regex {
        let pattern = self
            .dir_path_date_format
            .replace('\\', r"\\")
            .replace("%Y", r"\d{4}")
            .replace("%m", r"\d{2}")
            .replace("%d", r"\d{2}");
        Regex::new(&pattern).expect("Fail to generate regex.")
    }
}

/// 既定の設定情報名称を返す
fn default_config_name() -> (&'static str, Option<&'static str>) {
    let app_name = "daily-file-mover";
    let config_name = None;
    (app_name, config_name)
}

/// 設定情報を読み込んで `Config` 構造体を返す
pub(crate) fn load_config() -> Config {
    let (app_name, config_name) = default_config_name();
    confy::load(app_name, config_name).expect("Fail to load config.")
}

/// 設定ファイルのパスや、設定内容を表示する
pub(crate) fn print_config() {
    let (app_name, config_name) = default_config_name();
    let cfg_path = confy::get_configuration_file_path(app_name, config_name)
        .expect("Fail to load config file path.");
    let cfg: Config = confy::load(app_name, config_name).expect("Fail to load config.");
    println!("Config File Path: {:#?}\n{:#?}", cfg_path, cfg);

    let pwd = env::current_dir().expect("Fail to get current directory.");
    println!("Currnet Directory: {:#?}", pwd);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ファイルの日付接頭辞フォーマットを正規表現パターンに変換する() {
        let cfg = Config::default();
        assert_eq!(cfg.file_prefix_date_format, r"%Y%m%d_");
        assert_eq!(
            cfg.regex_file_prefix_date_format().as_str(),
            r"\d{4}\d{2}\d{2}_"
        )
    }
    #[test]
    fn ディレクトリパスの日付フォーマットを正規表現パターンに変換する() {
        let cfg = Config::default();
        assert_eq!(cfg.dir_path_date_format, r"%Y-%m\%Y-%m-%d");
        assert_eq!(
            cfg.regex_dir_path_date_format().as_str(),
            r"\d{4}-\d{2}\\\d{4}-\d{2}-\d{2}"
        )
    }
}
