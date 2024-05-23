use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::exit};
use walkdir::WalkDir;

use crate::{
    extension::Extension,
    paths::{
        get_extension_request_path, get_extension_response_path, get_extensions_dir,
        get_indexing_extensions_path,
    },
    result::WLResult,
    settings::ExtensionSetting,
};

use super::settings::{get_settings, write_settings};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionRequest {
    pub extension_id: String,
    pub action_context: ActionContext,
    pub extension_action: Option<String>,
    pub search_text: Option<String>,
    pub args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionContext {
    ResultsRequest,
    RunAction,
}

impl ExtensionRequest {
    pub fn new(extension_id: impl Into<String>, action_context: ActionContext) -> Self {
        Self {
            extension_id: extension_id.into(),
            action_context,
            extension_action: None,
            search_text: None,
            args: None,
        }
    }

    pub fn search_text(&mut self, search_text: impl Into<String>) -> Self {
        self.search_text = Some(search_text.into());
        self.to_owned()
    }

    pub fn extension_action(&mut self, extension_action: impl Into<String>) -> Self {
        self.extension_action = Some(extension_action.into());
        self.to_owned()
    }

    pub fn args(&mut self, args: Vec<String>) -> Self {
        self.args = Some(args.into());
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionResponse {
    pub results: Vec<WLResult>,
}

impl ExtensionResponse {
    pub fn new(results: Vec<WLResult>) -> Self {
        Self { results }
    }
}

pub fn index_extensions() {
    let mut extensions = Vec::<Extension>::new();
    let extensions_dir = get_extensions_dir();
    let indexing_extensions_path = get_indexing_extensions_path();
    let mut settings = get_settings();

    if !indexing_extensions_path.parent().unwrap().exists() {
        fs::create_dir_all(&indexing_extensions_path.parent().unwrap())
            .expect("Error creating directory");
    }

    if !extensions_dir.exists() {
        fs::create_dir_all(&extensions_dir).expect("Error creating extensions directory");
    }

    for entry in WalkDir::new(&extensions_dir) {
        if entry.is_ok() {
            let entry = entry.unwrap();
            let name = entry.file_name();

            if name == "manifest.json" {
                let json =
                    fs::read_to_string(entry.path()).expect("Error getting manifest content");

                if let Ok(extension) = serde_json::from_str::<Extension>(&json) {
                    extensions.push(extension.to_owned());

                    if let Some(extension_settings) = extension.settings {
                        for extension_setting in extension_settings {
                            let has_setting = settings.extensions.iter().any(|es| {
                                es.extension_id == extension.id
                                    && es.setting_id == extension_setting.id
                            });

                            if !has_setting {
                                settings.extensions.push(ExtensionSetting {
                                    extension_id: extension.id.to_owned(),
                                    setting_id: extension_setting.id.to_owned(),
                                    setting_value: extension_setting.default_value.to_owned(),
                                })
                            }

                            let has_keyword = settings.extensions.iter().any(|es| {
                                es.extension_id == extension.id && es.setting_id == "keyword"
                            });

                            if !has_keyword {
                                settings.extensions.push(ExtensionSetting {
                                    extension_id: extension.id.to_owned(),
                                    setting_id: String::from("keyword"),
                                    setting_value: extension.keyword.to_owned(),
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    write_settings(settings);

    let bytes = bincode::serialize(&extensions).expect("Error serializing extensions");
    fs::write(&get_indexing_extensions_path(), &bytes).expect("Error writing extensions");
}

pub fn get_extensions() -> Vec<Extension> {
    let path = get_indexing_extensions_path();
    let bytes = fs::read(path).expect("Error reading extensions");

    match bincode::deserialize(&bytes) {
        Ok(extensions) => extensions,
        Err(_) => Vec::new(),
    }
}

pub fn write_extension_request(request: ExtensionRequest) {
    let bytes = bincode::serialize(&request).expect("Error serializing context");
    fs::write(&get_extension_request_path(), &bytes).expect("Error writing context");
}

pub fn get_extension_request() -> ExtensionRequest {
    let bytes = fs::read(get_extension_request_path()).expect("Error reading extension request");
    let request = bincode::deserialize(&bytes).expect("Error deserializing extension request");
    request
}

pub fn write_extension_response(response: ExtensionResponse) {
    let bytes = bincode::serialize(&response).expect("Error serializing response");
    fs::write(&get_extension_response_path(), bytes).expect("Error writing extension response");
}

pub fn get_extension_response() -> ExtensionResponse {
    let bytes = fs::read(get_extension_response_path()).expect("Error reading extension response");
    let response = bincode::deserialize(&bytes).expect("Error deserializing extension response");
    response
}

pub fn send_response(results: Vec<WLResult>) {
    let response = ExtensionResponse::new(results);
    write_extension_response(response);
    exit(0);
}

pub fn get_extension_dir(extension_id: impl Into<String>) -> Option<PathBuf> {
    let extension_id = extension_id.into();
    let extensions_dir = get_extensions_dir();

    for entry in WalkDir::new(&extensions_dir) {
        if entry.is_ok() {
            let entry = entry.unwrap();
            let name = entry.file_name();

            if name == "manifest.json" {
                let json =
                    fs::read_to_string(entry.path()).expect("Error getting manifest content");
                if let Ok(extension) = serde_json::from_str::<Extension>(&json) {
                    if extension_id == extension.id {
                        return Some(entry.path().parent().unwrap().to_owned());
                    }
                }
            }
        }
    }

    return None;
}

pub fn get_extension_setting(
    extension_id: impl Into<String>,
    setting_id: impl Into<String>,
) -> Option<String> {
    let setting_id = setting_id.into();
    let extension_id = extension_id.into();

    let settings = get_settings();

    for setting in settings.extensions {
        if setting.extension_id == extension_id && setting.setting_id == setting_id {
            return Some(setting.setting_value);
        }
    }

    None
}
