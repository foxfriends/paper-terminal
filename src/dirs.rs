use directories::ProjectDirs;
use std::path::PathBuf;
use std::env;

fn syncat_directories() -> ProjectDirs {
    ProjectDirs::from("com", "cameldridge", "syncat").unwrap()
}

pub fn syncat_config() -> PathBuf {
    syncat_directories().config_dir().to_owned()
}

pub fn active_color() -> PathBuf {
    let active_color = env::var("syncat_active_style").unwrap_or("active".to_string());
    syncat_config().join("style").join(active_color)
}
