use std::fs;

use crate::{indexing::App, paths::get_indexing_apps_path};

pub fn get_apps() -> Vec<App> {
    let bytes = fs::read(get_indexing_apps_path()).expect("Error reading indexing apps");

    match bincode::deserialize(&bytes) {
        Ok(apps_indexing) => apps_indexing,
        Err(_) => Vec::new(),
    }
}
