use dirs::home_dir;
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
};

use crate::extensions::ExtensionManifest;

pub fn get_home_path() -> String {
    return home_dir().unwrap().into_os_string().into_string().unwrap();
}

pub fn get_settings_path() -> String {
    return match env::consts::OS {
        "linux" => {
            let home_path = get_home_path();

            format!("{home_path}/.config/simple-kl/settings.json")
        }
        _ => "".to_string(),
    };
}

pub fn get_apps_index_path() -> String {
    return match env::consts::OS {
        "linux" => "/tmp/simple-kl/apps.json".to_string(),
        _ => "".to_string(),
    };
}

pub fn get_extensions_index_path() -> String {
    return match env::consts::OS {
        "linux" => "/tmp/simple-kl/extensions.json".to_string(),
        _ => "".to_string(),
    };
}

pub fn get_temp_folder_path() -> String {
    return match env::consts::OS {
        "linux" => "/tmp/simple-kl/".to_string(),
        _ => "".to_string(),
    };
}

pub fn get_extensions_path() -> String {
    return match env::consts::OS {
        "linux" => {
            let home_path = get_home_path();

            format!("{home_path}/.local/share/simple-kl/extensions")
        }
        _ => "".to_string(),
    };
}

/// Gets the resources folder path.
///
/// Linux: `$HOME/.local/share/simple-kl/resources`
///
/// Windows: `C:\Program Files x64\simple-kl\resources`
pub fn get_resources_path() -> String {
    return format!("{}/.local/share/simple-kl/resources", get_home_path());
}

/// Gets the communit themes folder path.
///
/// Linux: `$HOME/.local/share/simple-kl/resources/themes`
///
/// Windows: `C:\Program Files x64\simple-kl\resources\themes`
pub fn get_community_themes_path() -> String {
    return format!(
        "{}/.local/share/simple-kl/resources/themes",
        get_home_path()
    );
}

/// Gets the communit themes file path.
///
/// Linux: `$HOME/.local/share/simple-kl/resources/themes`
///
/// Windows: `C:\Program Files x64\simple-kl\resources\themes`
pub fn get_community_themes_file_path() -> String {
    return format!(
        "{}/.local/share/simple-kl/resources/themes/themes.json",
        get_home_path()
    );
}

pub fn get_temp_themes_path() -> String {
    return format!("{}/themes", get_temp_folder_path());
}

pub fn get_community_extensions_path() -> String {
    return format!(
        "{}/.local/share/simple-kl/resources/extensions",
        get_home_path()
    );
}

pub fn get_community_extensions_file_path() -> String {
    return format!(
        "{}/.local/share/simple-kl/resources/extensions/extensions.json",
        get_home_path()
    );
}

///Gets a icon to use on the result
///
///**Note:** Use `@` to make the location relative to the extension folder. If not it will use the location as an absolute path.
///
///Usage Example:
///```no_run
///get_extension_icon(extension_id, "@src/images/icon.svg".to_string())
///```
pub fn get_extension_icon(extension_id: String, location: String) -> String {
    if let Ok(folders) = fs::read_dir(&get_extensions_path()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.json", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file.read_to_string(&mut manifest_json).unwrap();
                    manifest_file.flush().unwrap();

                    let manifest: ExtensionManifest = serde_json::from_str(&manifest_json).unwrap();

                    if manifest.id == extension_id {
                        return match location.starts_with("@") {
                            true => format!(
                                "{}",
                                location.replace(
                                    "@",
                                    (get_extension_path(manifest.id.clone()).unwrap() + "/")
                                        .as_str()
                                )
                            ),
                            false => location.to_string(),
                        };
                    }
                }
            }
        }
    }

    return "".to_string();
}

pub fn get_extension_parameters_path() -> String {
    return match env::consts::OS {
        "linux" => String::from("/tmp/simple-kl/extension-parameters.json"),
        _ => String::from(""),
    };
}

pub fn get_extension_results_path() -> String {
    return match env::consts::OS {
        "linux" => String::from("/tmp/simple-kl/extension-results.json"),
        _ => String::from(""),
    };
}

pub fn get_extension_path(id: String) -> Result<String, String> {
    if let Ok(folders) = fs::read_dir(&get_extensions_path()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.json", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file.read_to_string(&mut manifest_json).unwrap();
                    let _ = manifest_file.flush();
                    let manifest: ExtensionManifest = serde_json::from_str(&manifest_json).unwrap();

                    if manifest.id == id {
                        return Ok(folder_path.to_string());
                    }
                }
            }
        }
    }

    return Err("Error getting extension folder".into());
}
