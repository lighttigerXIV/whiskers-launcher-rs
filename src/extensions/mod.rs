use crate::{paths::{
    get_extension_parameters_path, get_extension_results_path,
}, results::SimpleKLResult, settings};
use serde::{Deserialize, Serialize};
use std::{fs::{File}, fs, io::{Read, Write}};
use std::path::Path;
use std::process::exit;
use crate::paths::{get_extensions_index_path, get_extensions_path, get_settings_path, get_temp_directory};
use crate::settings::{ExtensionOptionSetting, ExtensionsSettings, get_settings, update_settings};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameters {
    pub function: Function,
    pub search_text: Option<String>,
    pub action: Option<String>,
    pub custom_args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Function {
    GetResults,
    RunAction,
}

/// Returns
pub fn get_parameters() -> Option<Parameters> {
    let mut parameters_file =
        File::open(get_extension_parameters_path().unwrap()).expect("Error opening parameters file");
    let mut parameters_json = String::from("");

    parameters_file
        .read_to_string(&mut parameters_json)
        .expect("Error reading parameters file");

    parameters_file
        .flush()
        .expect("Error closing parameters file");

    let parameters = serde_yaml::from_str(&parameters_json).expect("Error getting parameters");
    return Some(parameters);
}

pub fn emit_results(results: &Vec<SimpleKLResult>) {
    let mut results_path_file =
        File::create(get_extension_results_path().unwrap()).expect("Error opening extension results");
    let results_yaml = serde_yaml::to_string(&results).expect("Error converting results to yaml");

    results_path_file
        .write_all(&results_yaml.as_bytes())
        .expect("Error writing extension results");

    results_path_file
        .flush()
        .expect("Error closing extension results file");

    exit(0);
}

impl Parameters {
    pub fn new_get_results(search_text: String) -> Self {
        return Parameters {
            function: Function::GetResults,
            search_text: Some(search_text),
            action: None,
            custom_args: None,
        };
    }

    pub fn new_action(action: String, args: Vec<String>) -> Self {
        return Parameters {
            function: Function::RunAction,
            search_text: None,
            action: Some(action),
            custom_args: Some(args),
        };
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub os: Vec<String>,
    pub keyword: String,
    pub settings: Option<ExtensionSettings>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExtensionSettings {
    #[serde(default = "default_extension_setting")]
    pub any: Vec<ExtensionSetting>,
    #[serde(default = "default_extension_setting")]
    pub linux: Vec<ExtensionSetting>,
    #[serde(default = "default_extension_setting")]
    pub windows: Vec<ExtensionSetting>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExtensionSetting {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub input: String,
    pub default_value: String,
    pub options: Option<Vec<ExtensionOption>>,
    pub show_condition: Option<Vec<ExtensionShowCondition>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExtensionOption {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExtensionShowCondition {
    pub setting: String,
    pub value: String,
}

fn default_extension_setting() -> Vec<ExtensionSetting> { return vec![]; }


pub fn init_extensions() {
    let extension_path = &get_settings_path().unwrap();
    let mut extensions: Vec<ExtensionManifest> = Vec::new();

    if !Path::new(extension_path).exists() {
        fs::create_dir_all(extension_path)
            .expect("Error creating extensions folder");
    }

    if let Ok(folders) = fs::read_dir(&extension_path) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.yml", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_yaml = String::from("");
                    manifest_file.read_to_string(&mut manifest_yaml).unwrap();

                    let manifest: ExtensionManifest = serde_yaml::from_str(&manifest_yaml).unwrap();

                    extensions.push(manifest);

                    manifest_file.flush().unwrap();
                }
            }
        }
    }

    if !get_temp_directory().unwrap().exists() {
        fs::create_dir_all(get_temp_directory().unwrap())
            .expect("Error creating temp folder");
    }

    let mut extension_file = File::create(get_extensions_index_path().unwrap()).unwrap();

    extension_file
        .write_all(serde_yaml::to_string(&extensions).unwrap().as_bytes())
        .unwrap();


    let settings_extensions = get_settings().extensions;

    let mut new_settings_extensions: Vec<ExtensionsSettings> = Vec::new();

    for extension in extensions {
        if !settings_extensions
            .iter()
            .any(|extension_setting| extension_setting.id == extension.id)
        {
            let mut any_settings: Vec<ExtensionOptionSetting> = Vec::new();
            let mut linux_settings: Vec<ExtensionOptionSetting> = Vec::new();
            let mut windows_settings: Vec<ExtensionOptionSetting> = Vec::new();

            if let Some(settings) = extension.settings {
                for any_setting in settings.any {
                    any_settings.push(ExtensionOptionSetting {
                        id: any_setting.id,
                        current_value: any_setting.default_value,
                    })
                }

                for linux_setting in settings.linux {
                    linux_settings.push(ExtensionOptionSetting {
                        id: linux_setting.id,
                        current_value: linux_setting.default_value,
                    })
                }


                for windows_setting in settings.windows {
                    windows_settings.push(ExtensionOptionSetting {
                        id: windows_setting.id,
                        current_value: windows_setting.default_value,
                    })
                }
            }

            new_settings_extensions.push(ExtensionsSettings {
                id: extension.id.clone(),
                keyword: extension.keyword.clone(),
                settings: settings::ExtensionSetting {
                    any: any_settings,
                    linux: linux_settings,
                    windows: windows_settings,
                },
            })
        } else {
            let extension_settings = settings_extensions.iter().find(|e| e.id == extension.id);

            match extension_settings {
                Some(settings) => new_settings_extensions.push(ExtensionsSettings {
                    id: settings.id.clone(),
                    keyword: settings.keyword.clone(),
                    settings: settings::ExtensionSetting {
                        any: settings.to_owned().settings.any,
                        linux: settings.to_owned().settings.linux,
                        windows: settings.to_owned().settings.windows,
                    },
                }),
                None => {}
            }
        }
    }

    let mut new_settings = get_settings();
    new_settings.extensions = new_settings_extensions;

    update_settings(&new_settings);
}

pub fn get_extensions() -> Vec<ExtensionManifest> {
    let mut extensions: Vec<ExtensionManifest> = Vec::new();

    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.json", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file.read_to_string(&mut manifest_json).unwrap();

                    let manifest: ExtensionManifest = serde_yaml::from_str(&manifest_json).unwrap();

                    extensions.push(manifest);

                    manifest_file.flush().unwrap();
                }
            }
        }
    }

    return extensions;
}

pub fn update_extension_keyword(extension_id: &str, keyword: &str) {
    let mut settings = get_settings();

    for (index, setting) in get_settings().extensions.iter().enumerate() {
        if setting.id == extension_id {
            settings.extensions[index].keyword = keyword.to_owned();
        }
    }

    update_settings(&settings);
}

pub fn update_extension_setting(extension_id: &str, setting_id: &str, value: &str) {
    let mut settings = get_settings();

    for (index, setting) in get_settings().extensions.iter().enumerate() {
        if setting.id == extension_id {
            for (any_index, any_setting) in setting.settings.any.iter().enumerate() {
                if any_setting.id == setting_id {
                    settings.extensions[index]
                        .settings
                        .any[any_index]
                        .current_value = value.to_owned();
                }
            }

            for (linux_index, linux_setting) in
            setting.settings.linux.iter().enumerate()
            {
                if linux_setting.id == setting_id {
                    settings.extensions[index]
                        .settings
                        .linux[linux_index]
                        .current_value = value.to_owned();
                }
            }

            for (windows_index, windows_setting) in
            setting.settings.windows.iter().enumerate()
            {
                if windows_setting.id == setting_id {
                    settings.extensions[index]
                        .settings
                        .windows[windows_index]
                        .current_value = value.to_owned();
                }
            }
        }
    }

    update_settings(&settings);
}