use std::{fs, io::Write};

use anyhow::{Context, Result};
use chrono::{Datelike, Local};

pub(crate) fn create_new_daily_report(
    path: Option<String>,
    template: Option<String>,
    date: Option<String>,
    dry_run: bool,
) -> Result<()> {
    // 設定読み込み
    let cfg = super::load_config();

    // 引数処理
    let base_dir = match path {
        Some(path_str) => super::convert_to_path(&path_str),
        None => super::convert_to_path(&cfg.base_dir_path),
    };
    let template_path = match template {
        Some(path_str) => super::convert_to_path(&path_str),
        None => super::convert_to_path(&cfg.template_file_path),
    };
    let target_date = match date {
        Some(date_str) => {
            super::date_from_str(&date_str).expect("Fail to parse date_str: date regex not match.")
        }
        None => Local::now().date_naive(),
    };

    // テンプレートファイルの内容を取得する
    let template_content = fs::read_to_string(&template_path)
        .with_context(|| {
            format!(
                "Template file not found: {}",
                &template_path.to_string_lossy()
            )
        })?
        .replace("mm", &format!("{:02}", target_date.month()))
        .replace("dd", &format!("{:02}", target_date.day()));

    // 新規ファイルのpathを作成する
    let file_name = &cfg.daily_report_file_name;
    let new_dir_path = super::generate_new_dir_path_with_date(&base_dir, &target_date, &cfg);
    let new_file_path =
        super::generate_new_file_path_with_date(&new_dir_path, file_name, &target_date, &cfg);

    // 新規ファイルを作成する
    if dry_run {
        println!("[dry_run] create file: {}", new_file_path.to_string_lossy());
    } else {
        println!("create file: {}", new_file_path.to_string_lossy());
        std::fs::create_dir_all(&new_dir_path)?;
        let mut file = fs::File::create(&new_file_path)?;
        file.write_all(&template_content.bytes().collect::<Vec<u8>>())?;
        file.flush()?;
    }

    Ok(())
}
