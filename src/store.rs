use directories::ProjectDirs;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

fn get_base_config_path() -> PathBuf {
    ProjectDirs::from("com", "Jabczyk", "Clic")
        .unwrap()
        .config_dir()
        .to_owned()
}

pub fn create_config_dir() {
    create_dir_all(get_base_config_path()).unwrap();
}

pub fn get_config_path(filename: &str) -> PathBuf {
    let mut path = get_base_config_path();
    path.push(filename);

    path
}

pub fn persist_json<T>(filename: &str, value: &T)
where
    T: ?Sized + Serialize,
{
    let writer = File::with_options()
        .write(true)
        .create(true)
        .open(get_config_path(filename))
        .unwrap();

    serde_json::to_writer(writer, value).unwrap();
}

pub fn load_from_json<T>(filename: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let reader = File::open(get_config_path(filename))?;

    Ok(serde_json::from_reader(reader)?)
}
