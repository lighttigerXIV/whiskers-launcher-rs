use crate::actions;
use crate::paths::{
    get_extension_context_path, get_extension_dialog_action_path, get_extension_results_path,
    get_user_extensions_dir,
};
use crate::results::WhiskersResult;
use crate::settings::{get_settings, ExtensionSetting};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::exit;
use std::{fs, io};
use walkdir::WalkDir;

use self::manifest::Manifest;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Context {
    pub action: Action,
    pub search_text: Option<String>,
    pub extension_action: Option<String>,
    pub custom_args: Vec<String>,
}

impl Context {
    pub fn new(action: Action) -> Self {
        return Self {
            action,
            search_text: None,
            extension_action: None,
            custom_args: vec![],
        };
    }

    pub fn search_text(&mut self, search_text: impl Into<String>) -> Self {
        self.search_text = Some(search_text.into());
        self.to_owned()
    }

    pub fn extension_action(&mut self, extension_action: impl Into<String>) -> Self {
        self.extension_action = Some(extension_action.into());
        self.to_owned()
    }

    pub fn custom_args(&mut self, custom_args: Vec<String>) -> Self {
        self.custom_args = custom_args;
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    GetResults,
    RunAction,
}

pub mod manifest {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Manifest {
        pub id: String,
        pub name: String,
        pub version_name: String,
        pub version_code: usize,
        pub description: String,
        pub os: Vec<String>,
        pub keyword: String,
        pub settings: Option<Vec<Setting>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Setting {
        pub id: String,
        pub title: String,
        pub description: String,
        pub setting_type: String,
        pub default_value: String,
        pub os: Vec<String>,
        pub select_options: Option<Vec<SelectOption>>,
        pub show_conditions: Option<Vec<ShowCondition>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SelectOption {
        pub id: String,
        pub text: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ShowCondition {
        pub setting_id: String,
        pub setting_value: String,
    }
}

pub struct DialogResult {
    pub id: String,
    pub value: String,
}

impl DialogResult {
    pub fn new(id: impl Into<String>, value: impl Into<String>) -> Self {
        return Self {
            id: id.into(),
            value: value.into(),
        };
    }
}

// =====================================================
// Functions
// =====================================================

pub fn get_extension_dir(extension_id: impl Into<String>) -> Option<PathBuf> {
    let extensions_dir = get_user_extensions_dir()?;
    let id = extension_id.into();

    for entry in WalkDir::new(&extensions_dir) {
        let entry = entry.ok()?;

        if entry.file_name() == "manifest.json" {
            let manifest: Manifest =
                serde_json::from_str(&fs::read_to_string(&entry.path()).ok()?).ok()?;
            if manifest.id == id {
                return Some(entry.path().parent()?.to_owned());
            }
        }
    }

    None
}

pub fn get_extension_manifest(extension_id: impl Into<String>) -> Option<Manifest> {
    let extensions_dir = get_user_extensions_dir()?;
    let id = extension_id.into();

    for entry in WalkDir::new(&extensions_dir) {
        let entry = entry.ok()?;

        if entry.file_name() == "manifest.json" {
            let manifest: Manifest =
                serde_json::from_str(&fs::read_to_string(&entry.path()).ok()?).ok()?;
            if manifest.id == id {
                return Some(manifest);
            }
        }
    }

    None
}

pub fn send_extension_context(context: Context) -> io::Result<()> {
    let file_path = get_extension_context_path().ok_or(()).unwrap();
    let json_context = serde_json::to_string(&context).map_err(|_| ()).unwrap();
    fs::write(file_path, &json_context).map_err(|_| ()).unwrap();

    return Ok(());
}

pub fn get_extension_context() -> Option<Context> {
    let file_path = get_extension_context_path()?;
    let file_content = read_to_string(&file_path).ok()?;
    let deserialized_context: Context = serde_json::from_str(&file_content).ok()?;

    return Some(deserialized_context);
}

pub fn send_extension_results(results: Vec<WhiskersResult>) {
    let file_path = get_extension_results_path().unwrap();
    let json_results = serde_json::to_string(&results).unwrap();
    fs::write(file_path, &json_results).unwrap();

    exit(0);
}

pub fn get_extension_results() -> Option<Vec<WhiskersResult>> {
    let file_path = get_extension_results_path()?;
    let file_content = read_to_string(&file_path).ok()?;
    let extension_results = serde_json::from_str(&file_content).ok()?;

    Some(extension_results)
}

pub fn get_extension_settings(extension_id: impl Into<String>) -> Option<Vec<ExtensionSetting>> {
    let settings = get_settings()?;
    let extension_id = extension_id.into();

    for extension in settings.extensions {
        if extension.id == extension_id {
            return Some(extension.settings);
        }
    }

    return None;
}

pub fn get_extension_setting(
    extension_id: impl Into<String>,
    setting_id: impl Into<String>,
) -> Option<String> {
    let settings = get_settings()?;
    let extension_id = extension_id.into();
    let setting_id = setting_id.into();

    for extension in settings.extensions {
        if extension.id == extension_id {
            for extension_setting in extension.settings {
                if extension_setting.id == setting_id {
                    return Some(extension_setting.value);
                }
            }
        }
    }

    return None;
}

pub fn send_extension_dialog_action(action: actions::Dialog) {
    let path = get_extension_dialog_action_path().expect("Error getting action path");

    let action_json = serde_json::to_string(&action).expect("Error converting action to a json");

    fs::write(&path, &action_json).expect("Error writing action");
}

pub fn get_extension_dialog_action() -> Option<actions::Dialog> {
    let path = get_extension_dialog_action_path().expect("Error getting action path");
    let action_json = fs::read_to_string(&path).expect("Error getting action file content");
    let action: actions::Dialog = serde_json::from_str(&action_json).expect("Error getting action");

    return Some(action);
}
