use dirs::home_dir;
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::extensions::ExtensionManifest;

pub fn get_home_path() -> Option<PathBuf> {
    return home_dir();
}


pub fn get_local_dir() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => {
            let mut path = home_dir().unwrap();
            path.push(".local/share/simple-kl");

            Some(path)
        }
        "windows" => {
            let mut path = Path::new(&env::var("APPDATA").unwrap()).to_owned();
            path.push("simple-kl");

            Some(path)
        }
        _ => None
    };
}

pub fn get_settings_path() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => {
            let mut path = get_home_path()?;
            path.push(".config/simple-kl/settings.yml");

            Some(path)
        }
        "windows" => {
            let mut path = get_local_dir().unwrap();
            path.push("settings.yml");

            Some(path)
        }
        _ => None,
    };
}

pub fn get_apps_index_path() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => Some(Path::new("/tmp/simple-kl/apps.yml").to_owned()),
        "windows" => {
            let mut path = get_temp_directory().unwrap();
            path.push("apps.yml");

            Some(path)
        }
        _ => None,
    };
}

pub fn get_extensions_index_path() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => Some(Path::new("/tmp/simple-kl/extensions.yml").to_owned()),
        "windows" => {
            let mut path = get_temp_directory().unwrap();
            path.push("extensions.yml");

            Some(path)
        }
        _ => None,
    };
}

pub fn get_temp_directory() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => Some(Path::new("/tmp/simple-kl/").to_owned()),
        "windows" => {
            let mut tmp_path = Path::new(&env::var("TEMP").unwrap()).to_owned();
            tmp_path.push("simple-kl");

            Some(tmp_path)
        }
        _ => None,
    };
}

/// Get the extensions folder path
pub fn get_extensions_path() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => {
            let mut path = get_local_dir().unwrap();
            path.push("extensions");

            Some(path)
        }
        "windows" => {
            let mut path = get_local_dir().unwrap();
            path.push("extensions");

            Some(path)
        }
        _ => None,
    };
}

/// Gets the resources folder path.
pub fn get_resources_directory() -> Option<PathBuf> {
    let mut path = get_local_dir().unwrap();
    path.push("resources");

    Some(path)
}

/// Gets the community themes folder path.
pub fn get_community_themes_path() -> Option<PathBuf> {
    let mut path = get_resources_directory()?;
    path.push("themes");

    return Some(path);
}

/// Gets the community themes file path.
pub fn get_community_themes_file_path() -> Option<PathBuf> {
    let mut path = get_community_themes_path()?;
    path.push("themes.yml");

    return Some(path);
}

pub fn get_temp_themes_path() -> Option<PathBuf> {
    let mut path = get_temp_directory()?;
    path.push("themes");

    return Some(path);
}

pub fn get_community_extensions_directory() -> Option<PathBuf> {
    let mut path = get_resources_directory()?;
    path.push("extensions");

    return Some(path);
}

pub fn get_community_extensions_file_path() -> Option<PathBuf> {
    let mut path = get_community_extensions_directory()?;
    path.push("extensions.yml");

    return Some(path);
}

pub fn get_dialog_action_path() -> Option<PathBuf> {
    let mut path = get_temp_directory().unwrap();
    path.push("dialog-action.yml");

    return Some(path);
}

///Gets the extension folder
pub fn get_extension_directory(extension_id: &str) -> Option<PathBuf> {
    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let mut manifest_file_path = folder.path();
                manifest_file_path.push("manifest.yml");

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_yaml = String::from("");
                    manifest_file.read_to_string(&mut manifest_yaml).unwrap();
                    manifest_file.flush().unwrap();

                    let manifest: ExtensionManifest = serde_yaml::from_str(&manifest_yaml).unwrap();

                    if manifest.id == extension_id {
                        return Some(folder.path());
                    }
                }
            }
        }
    }

    return None;
}


pub fn get_extension_parameters_path() -> Option<PathBuf> {

    let mut path = get_temp_directory().unwrap();
    path.push("extension-parameters.yml");

    Some(path)
}

pub fn get_extension_results_path() -> Option<PathBuf> {

    let mut path = get_temp_directory().unwrap();
    path.push("extension-results.yml");

    Some(path)
}

pub fn get_extension_path(id: &str) -> Option<PathBuf> {
    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let mut manifest_file_path = folder.path();
                manifest_file_path.push("manifest.yml");

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_yaml = String::from("");

                    manifest_file
                        .read_to_string(&mut manifest_yaml)
                        .expect("Error reading manifest file");

                    manifest_file.flush().expect("Error closing manifest file");

                    let manifest: ExtensionManifest = serde_yaml::from_str(&manifest_yaml).unwrap();

                    if manifest.id == id {
                        return Some(folder.path());
                    }
                }
            }
        }
    }

    return None;
}
