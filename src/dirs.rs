use directories::ProjectDirs;
use std::path::PathBuf;

fn syncat_directories() -> ProjectDirs {
    ProjectDirs::from("com", "cameldridge", "syncat").unwrap()
}

pub fn syncat_config() -> PathBuf {
    syncat_directories().config_dir().to_owned()
}
