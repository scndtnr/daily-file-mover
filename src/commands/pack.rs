use super::convert_to_path;
use anyhow::Result;
use chrono::NaiveDate;
use regex::Regex;

pub(crate) fn pack_daily_files(src: String, dst: String, dry_run: bool) -> Result<()> {
    let cfg = super::load_config();
    let src_dir = convert_to_path(&src);
    let dst_dir = convert_to_path(&dst);
    let date_regex =
        Regex::new(r"^(?P<y>\d{4})[/-]?(?P<m>\d{1,2})[/-]?(?P<d>\d{1,2})_.*$").unwrap();
    for entry_result in src_dir.read_dir()? {
        let entry = entry_result?;
        if entry.metadata()?.is_file() {
            if let Some(file_name) = entry.file_name().to_str() {
                // 日次ディレクトリ作成のため、ファイルの接頭辞から日付データを作成する
                let Some(caps) = date_regex.captures(file_name) else {continue};
                let date_str = format!("{}{:02}{:02}", &caps["y"], &caps["m"], &caps["d"]);
                let date = NaiveDate::parse_from_str(&date_str, "%Y%m%d").unwrap();

                // 日次ディレクトリにファイルを格納する
                let new_dir_path = super::generate_new_dir_path_with_date(&dst_dir, &date, &cfg);
                let new_file_path = super::generate_new_file_path(&new_dir_path, file_name);

                if dry_run {
                    println!(
                        "[dry_run] move file: {} --> {}",
                        entry.path().to_string_lossy(),
                        new_file_path.to_string_lossy()
                    );
                } else {
                    println!(
                        "move file: {} --> {}",
                        entry.path().to_string_lossy(),
                        new_file_path.to_string_lossy()
                    );
                    std::fs::create_dir_all(&new_dir_path)?;
                    std::fs::rename(entry.path(), new_file_path)?;
                }
            }
        }
    }
    Ok(())
}
