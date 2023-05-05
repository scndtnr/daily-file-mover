mod config;
mod extract;
mod pack;

pub(super) use config::print_config;
pub(super) use pack::pack_daily_files;

use chrono::NaiveDate;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use self::config::{load_config, Config};

fn convert_to_path(path_str: &str) -> PathBuf {
    PathBuf::from_str(path_str).expect("Fail to Convert src to root directory.")
}

fn generate_new_dir_path(root_dir: &Path, date: &NaiveDate, cfg: &Config) -> PathBuf {
    let dir_path_str = date.format(&cfg.dir_path_date_format).to_string();
    root_dir.join(dir_path_str)
}

fn generate_new_file_path(new_dir_path: &Path, file_name: &str) -> PathBuf {
    new_dir_path.join(file_name)
}
