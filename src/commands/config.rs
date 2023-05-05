use std::env;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) file_prefix_date_format: String,
    pub(crate) dir_path_date_format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            file_prefix_date_format: "%Y%m%d_".into(),
            dir_path_date_format: "%Y-%m/%Y-%m-%d".into(),
        }
    }
}

fn default_config_name() -> (&'static str, Option<&'static str>) {
    let app_name = "daily-file-mover";
    let config_name = None;
    (app_name, config_name)
}

pub(crate) fn load_config() -> Config {
    let (app_name, config_name) = default_config_name();
    confy::load(app_name, config_name).expect("Fail to load config.")
}

pub(crate) fn print_config() {
    let (app_name, config_name) = default_config_name();
    let cfg_path = confy::get_configuration_file_path(app_name, config_name)
        .expect("Fail to load config file path.");
    let cfg: Config = confy::load(app_name, config_name).expect("Fail to load config.");
    println!("Config File Path: {:#?}\n{:#?}", cfg_path, cfg);

    let pwd = env::current_dir().expect("Fail to get current directory.");
    println!("Currnet Directory: {:#?}", pwd);
}
