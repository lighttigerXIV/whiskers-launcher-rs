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

pub fn get_settings_path() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => {
            let mut home_path = get_home_path()?;
            home_path.push(".config/simple-kl/settings.yml");

            Some(home_path)
        }
        _ => None,
    };
}

pub fn get_apps_index_path() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => Some(Path::new("/tmp/simple-kl/apps.yml").to_owned()),
        _ => None,
    };
}

pub fn get_extensions_index_path() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => Some(Path::new("/tmp/simple-kl/extensions.yml").to_owned()),
        _ => None,
    };
}

pub fn get_temp_directory() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => Some(Path::new("/tmp/simple-kl/").to_owned()),
        _ => None,
    };
}

pub fn get_extensions_path() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => {
            let mut home_path = get_home_path()?;
            home_path.push(".local/share/simple-kl/extensions");

            Some(home_path)
        }
        _ => None,
    };
}

/// Gets the resources folder path.
///
/// Linux: `$HOME/.local/share/simple-kl/resources`
///
/// Windows: `C:\Program Files x64\simple-kl\resources`
pub fn get_resources_directory() -> Option<PathBuf> {
    let mut path = get_home_path()?;
    path.push(".local/share/simple-kl/resources");

    return Some(path);
}

/// Gets the communit themes folder path.
///
/// Linux: `$HOME/.local/share/simple-kl/resources/themes`
///
/// Windows: `C:\Program Files x64\simple-kl\resources\themes`
pub fn get_community_themes_path() -> Option<PathBuf> {
    let mut path = get_resources_directory()?;
    path.push("themes");

    return Some(path);
}

/// Gets the communit themes file path.
///
/// Linux: `$HOME/.local/share/simple-kl/resources/themes`
///
/// Windows: `C:\Program Files x64\simple-kl\resources\themes`
pub fn get_community_themes_file_path() -> Option<PathBuf> {
    let mut path = get_community_themes_path()?;
    path.push("themes.json");

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
    path.push("extensions.json");

    return Some(path);
}

///Gets a icon to use on the result
///
///**Note:** Use `@` to make the location relative to the extension folder. If not it will use the location as an absolute path.
///
///Usage Example:
///```no_run
///get_extension_icon(extension_id, "@src/images/icon.svg".to_string())
///```
pub fn get_extension_icon(extension_id: &str, location: &str) -> Option<PathBuf> {
    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let mut manifest_file_path = folder.path();
                manifest_file_path.push("manifest.json");

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file.read_to_string(&mut manifest_json).unwrap();
                    manifest_file.flush().unwrap();

                    let manifest: ExtensionManifest = serde_json::from_str(&manifest_json).unwrap();

                    if manifest.id == extension_id {

                        if location.starts_with("@"){
                            let mut path = get_extension_path(manifest.id.to_owned()).unwrap();
                            path.push(location.replace("@", ""));

                            return Some(path);
                        }

                        return Some(Path::new(location).to_owned());
                    }
                }
            }
        }
    }

    return None;
}

///Gets the extension folder
///
///**Linux**: `{HOME}/.local/share/simple-kl/extensions/{extension_folder}` 
///
///**Example**
///
///```let directory = get_extension_directory("com-lighttigerxiv-bookmarks")```
pub fn get_extension_directory(extension_id: &str)-> Option<PathBuf>{
    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let mut manifest_file_path = folder.path();
                manifest_file_path.push("manifest.json");

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file.read_to_string(&mut manifest_json).unwrap();
                    manifest_file.flush().unwrap();

                    let manifest: ExtensionManifest = serde_json::from_str(&manifest_json).unwrap();

                    if manifest.id == extension_id {
                        return Some(folder.path())
                    }
                }
            }
        }
    }

    return None
}



pub fn get_extension_parameters_path() -> String {
    return match env::consts::OS {
        "linux" => String::from("/tmp/simple-kl/extension-parameters.yml"),
        _ => String::from(""),
    };
}

pub fn get_extension_results_path() -> String {
    return match env::consts::OS {
        "linux" => String::from("/tmp/simple-kl/extension-results.yml"),
        _ => String::from(""),
    };
}

pub fn get_extension_path(id: String) -> Option<PathBuf> {
    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let mut manifest_file_path = folder.path();
                manifest_file_path.push("manifest.json");

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");

                    manifest_file
                        .read_to_string(&mut manifest_json)
                        .expect("Error reading manifest file");

                    manifest_file.flush().expect("Error closing manifest file");

                    let manifest: ExtensionManifest = serde_json::from_str(&manifest_json).unwrap();

                    if manifest.id == id {
                        return Some(folder.path());
                    }
                }
            }
        }
    }

    return None;
}
