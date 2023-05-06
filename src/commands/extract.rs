use std::path::Path;

use anyhow::Result;
use walkdir::{DirEntry, WalkDir};

use crate::commands::convert_to_path;

use super::{
    config::{load_config, Config},
    date_from_dir_path,
};

pub(crate) fn extract_daily_files(src: String, dst: String, dry_run: bool) -> Result<()> {
    let cfg = load_config();
    let dst_dir = convert_to_path(&dst);
    let mut date_dir_entries: Vec<DirEntry> = WalkDir::new(src)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_dir())
        .filter(|entry| is_date_dir(entry, &cfg))
        .collect();
    // 抽出した日次ディレクトリ配下に対して処理を行う
    for dir_entry in date_dir_entries.iter() {
        let date = date_from_dir_path(&dir_entry.path().to_string_lossy(), &cfg);
        for file_entry in WalkDir::new(dir_entry.path())
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
        {
            let old_file_path = file_entry.path();
            let new_file_path = super::generate_new_file_path_with_date(
                &dst_dir,
                file_entry
                    .file_name()
                    .to_str()
                    .expect("Fail to convert file_name to str"),
                &date,
                &cfg,
            );
            if dry_run {
                println!(
                    "[dry_run] move file: {} --> {}",
                    old_file_path.to_string_lossy(),
                    new_file_path.to_string_lossy()
                );
            } else {
                println!(
                    "move file: {} --> {}",
                    old_file_path.to_string_lossy(),
                    new_file_path.to_string_lossy()
                );
                std::fs::rename(old_file_path, new_file_path)?;
            }
        }
    }
    // 空のディレクトリを削除する
    // アイテム数の少ないディレクトリから処理する（空ディレクトリを持つがファイルを持たないディレクトリを処理するため）
    date_dir_entries.sort_by_key(|e| dir_item_count(e.path()));
    for dir_entry in date_dir_entries.iter() {
        let dirpath = dir_entry.path();
        if is_empty_dir(dirpath) {
            if dry_run {
                println!(
                    "[dry_run] delete empty direcotry: {}",
                    dirpath.to_string_lossy()
                );
            } else {
                println!(
                    "[dry_run] delete empty direcotry: {}",
                    dirpath.to_string_lossy()
                );
                std::fs::remove_dir(dirpath)?;
            }
        }
    }
    Ok(())
}

fn is_date_dir(entry: &DirEntry, cfg: &Config) -> bool {
    let path_str = entry.path().to_str().expect("Fail to unwrap path_str.");
    cfg.regex_dir_path_date_format().is_match(path_str)
}

fn dir_item_count(path: &Path) -> usize {
    std::fs::read_dir(path).unwrap().count()
}

fn is_empty_dir(path: &Path) -> bool {
    std::fs::read_dir(path).unwrap().count() == 0
}
