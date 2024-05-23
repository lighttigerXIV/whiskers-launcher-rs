use std::fs;

use crate::{
    paths::get_settings_path,
    settings::{get_default_settings, Settings},
};

pub fn get_settings() -> Settings {
    let settings_path = get_settings_path();

    if !settings_path.parent().unwrap().exists() {
        fs::create_dir_all(&settings_path.parent().unwrap())
            .expect("Error creating settings directory");
    }

    if !settings_path.exists() {
        write_settings(get_default_settings());
    }

    let settings_bytes = fs::read(get_settings_path()).expect("Error reading settings");
    let decoded_settings = bincode::deserialize::<Settings>(&settings_bytes);

    match decoded_settings {
        Ok(settings) => settings,
        Err(_) => get_default_settings(),
    }
}

pub fn write_settings(settings: Settings) {
    let bytes = bincode::serialize(&settings).expect("Error encoding settings");
    fs::write(&get_settings_path(), bytes).expect("Error writing settings");
}
