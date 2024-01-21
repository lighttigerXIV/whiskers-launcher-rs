use std::env;
use std::path::{Path, PathBuf};
use dirs::home_dir;

pub fn get_local_dir() -> Option<PathBuf> {
    return match env::consts::OS {
        "windows" => {
            let mut path = Path::new(&env::var("APPDATA").unwrap()).to_owned();
            path.push("simple-kl");

            Some(path)
        }
        //Unix Systems
        _ => {
            let mut path = home_dir().unwrap();
            path.push(".local/share/simple-kl");

            Some(path)
        }
    };
}

pub fn get_temp_dir() -> Option<PathBuf> {
    return match env::consts::OS {
        "windows" => {
            let mut tmp_path = Path::new(&env::var("TEMP").unwrap()).to_owned();
            tmp_path.push("simple-kl-runtime");

            Some(tmp_path)
        }
        //Unix Systems
        _ => Some(Path::new("/tmp/simple-kl-runtime/").to_owned()),
    };
}

pub fn get_extension_results_path() -> Option<PathBuf> {
    let mut path = match get_temp_dir() {
        None => { return None; }
        Some(path) => path
    };

    path.push("ExtensionResults.yml");
    return Some(path);
}

pub fn get_extension_context_path() -> Option<PathBuf> {
    let mut path = get_temp_dir()?;

    path.push("ExtensionContext.yml");
    return Some(path);
}

pub fn get_dialog_results_path() -> Option<PathBuf> {
    let mut path = match get_temp_dir() {
        None => { return None; }
        Some(path) => path
    };

    path.push("DialogResults.yml");
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
    path.push("Extensions.yml");
    return Some(path);
}

pub fn get_indexing_apps_path() -> Option<PathBuf> {
    let mut path = get_indexing_dir()?;
    path.push("Apps.yml");
    return Some(path);
}

pub fn get_app_resources_dir() -> Option<PathBuf> {
    let mut path = get_local_dir()?;
    path.push("AppResources");
    return Some(path);
}

pub fn get_app_resources_icons_dir() -> Option<PathBuf> {
    let mut path = get_indexing_dir()?;
    path.push("Icons");
    return Some(path);
}