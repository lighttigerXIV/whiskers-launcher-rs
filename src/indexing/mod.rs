use std::fs;

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{
    api::extensions::manifest::Manifest,
    paths::{get_indexing_apps_path, get_user_extensions_dir},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppIndex {
    pub icon_path: Option<String>,
    pub exec_path: String,
    pub name: String,
}

impl AppIndex {
    pub fn new(
        icon_path: impl Into<String>,
        exec_path: impl Into<String>,
        name: impl Into<String>,
    ) -> AppIndex {
        return AppIndex {
            icon_path: Some(icon_path.into()),
            exec_path: exec_path.into(),
            name: name.into(),
        };
    }
}

pub fn get_indexed_apps() -> Option<Vec<AppIndex>> {
    let path = get_indexing_apps_path()?;
    let content = fs::read_to_string(&path).ok()?;
    let apps = serde_json::from_str::<Vec<AppIndex>>(&content).ok()?;

    return Some(apps);
}

pub fn get_user_extensions() -> Option<Vec<Manifest>> {
    let extensions_dir = get_user_extensions_dir()?;
    let mut extensions: Vec<Manifest> = vec![];

    if extensions_dir.is_dir() {
        for entry in WalkDir::new(&extensions_dir){
            let entry = entry.ok()?;
            let entry_name = entry.file_name();

            if entry_name == "manifest.json" {
                let content = fs::read_to_string(&entry.path()).ok()?;
                if let Ok(manifest) = serde_json::from_str(&content){
                    extensions.push(manifest);
                }
            }
        }
    }

    return Some(extensions);
}
