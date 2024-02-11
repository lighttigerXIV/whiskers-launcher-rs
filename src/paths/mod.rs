use dirs::home_dir;
use std::env;
use std::path::{Path, PathBuf};

pub fn get_local_dir() -> Option<PathBuf> {
    return match env::consts::OS {
        "windows" => {
            let mut path = Path::new(&env::var("APPDATA").unwrap()).to_owned();
            path.push("com-lighttigerxiv-whiskers-launcher");

            Some(path)
        }
        //Unix Systems
        _ => {
            let mut path = home_dir().unwrap();
            path.push(".local/share/com-lighttigerxiv-whiskers-launcher");

            Some(path)
        }
    };
}

pub fn get_temp_dir() -> Option<PathBuf> {
    return match env::consts::OS {
        "windows" => {
            let mut tmp_path = Path::new(&env::var("TEMP").unwrap()).to_owned();
            tmp_path.push("whiskers-launcher-runtime");

            Some(tmp_path)
        }
        //Unix Systems
        _ => Some(Path::new("/tmp/whiskers-launcher-runtime").to_owned()),
    };
}

pub fn get_home_dir() -> Option<PathBuf> {
    return home_dir();
}

pub fn get_extension_results_path() -> Option<PathBuf> {
    let mut path = match get_temp_dir() {
        None => {
            return None;
        }
        Some(path) => path,
    };

    path.push("extension_results.json");
    return Some(path);
}

pub fn get_extension_context_path() -> Option<PathBuf> {
    let mut path = get_temp_dir()?;

    path.push("extension_context.json");
    return Some(path);
}

pub fn get_dialog_results_path() -> Option<PathBuf> {
    let mut path = match get_temp_dir() {
        None => {
            return None;
        }
        Some(path) => path,
    };

    path.push("dialog_results.json");
    return Some(path);
}

pub fn get_user_extensions_dir() -> Option<PathBuf> {
    let mut path = get_local_dir()?;
    path.push("UserExtensions");
    return Some(path);
}

pub fn get_indexing_dir() -> Option<PathBuf> {
    let mut path = get_local_dir()?;
    path.push("Indexing");
    return Some(path);
}

pub fn get_indexing_shortcuts_dir() -> Option<PathBuf> {
    let mut path = get_indexing_dir()?;
    path.push("Shortcuts");
    return Some(path);
}

pub fn get_indexing_icons_dir() -> Option<PathBuf> {
    let mut path = get_indexing_dir()?;
    path.push("Icons");
    return Some(path);
}

pub fn get_indexing_extensions_path() -> Option<PathBuf> {
    let mut path = get_indexing_dir()?;
    path.push("extensions.json");
    return Some(path);
}

pub fn get_indexing_apps_path() -> Option<PathBuf> {
    let mut path = get_indexing_dir()?;
    path.push("apps.json");
    return Some(path);
}

pub fn get_app_resources_dir() -> Option<PathBuf> {
    let mut path = get_local_dir()?;
    path.push("AppResources");
    return Some(path);
}

pub fn get_app_resources_icons_dir() -> Option<PathBuf> {
    let mut path = get_app_resources_dir()?;
    path.push("Icons");
    return Some(path);
}

pub fn get_settings_path() -> Option<PathBuf> {
    return match env::consts::OS {
        "windows" => {
            let mut path = get_local_dir()?;
            path.push("settings.json");

            Some(path)
        }
        //Unix Systems
        _ => {
            let mut path = get_home_dir()?;
            path.push(".config/whiskers-launcher/settings.json");

            Some(path)
        }
    };
}

pub fn get_autostart_path() -> Option<PathBuf> {
    return match env::consts::OS {
        "linux" => {
            let mut path = get_home_dir()?;
            path.push(".config/autostart");

            return Some(path);
        }
        "windows" => {
            let mut path = Path::new(&env::var("APPDATA").map_err(|_| ()).unwrap()).to_owned();
            path.push("Microsoft\\Windows\\Start Menu\\Programs\\Startup");

            return Some(path);
        }
        _ => None,
    };
}

pub fn get_extension_dialog_action_path()-> Option<PathBuf>{
    let mut path = get_temp_dir()?;
    path.push("extension_dialog_action.json");

    return Some(path); 
}

pub fn get_extension_dialog_response_path()-> Option<PathBuf>{
    let mut path = get_temp_dir()?;
    path.push("extension_dialog_response.json");

    return Some(path); 
}