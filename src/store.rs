use directories::ProjectDirs;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    ProjectDirs::from("com", "Jabczyk", "Clic")
        .unwrap()
        .config_dir()
        .to_owned()
}

pub fn create_config_dir() {
    create_dir_all(get_config_path()).unwrap();
}

pub fn get_history_path() -> PathBuf {
    let mut path = get_config_path();
    path.push("history.txt");

    path
}
