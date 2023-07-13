use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::paths::get_settings_path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub general: GeneralSettings,
    pub search_box: SearchBoxSettings,
    pub theming: ThemingSettings,
    pub web_search: WebSearchSettings,
    pub extensions: Vec<ExtensionsSettings>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralSettings {
    pub first_key: String,
    pub second_key: String,
    pub third_key: String,
    pub limit: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchBoxSettings {
    pub show_search_icon: bool,
    pub show_settings_icon: bool,
    pub roundness: usize,
    pub border_width: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThemingSettings {
    pub background: String,
    pub secondary_background: String,
    pub tertiary_background: String,
    pub accent: String,
    pub on_accent: String,
    pub text: String,
    pub seconday_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSearchSettings {
    default: Vec<SearchOption>,
    custom: Vec<SearchOption>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchOption {
    pub keyword: String,
    pub icon: Option<String>,
    pub name: String,
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtensionsSettings {
    pub id: String,
    pub keyword: String,
    pub settings: ExtensionSetting,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtensionSetting {
    pub any: Vec<ExtensionOptionSetting>,
    pub linux: Vec<ExtensionOptionSetting>,
    pub windows: Vec<ExtensionOptionSetting>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtensionOptionSetting {
    pub id: String,
    pub current_value: String,
}

impl Settings {
    pub fn default_settings() -> Settings {
        return Settings {
            general: GeneralSettings {
                first_key: "ctrl".to_string(),
                second_key: "".to_string(),
                third_key: "space".to_string(),
                limit: 6,
            },
            search_box: SearchBoxSettings {
                show_search_icon: false,
                show_settings_icon: true,
                roundness: 4,
                border_width: 2,
            },
            theming: ThemingSettings {
                background: "#1e1e2e".to_string(),
                secondary_background: "#11111b".to_string(),
                tertiary_background: "#181825".to_string(),
                accent: "#89b4fa".to_string(),
                on_accent: "#1e1e2e".to_string(),
                text: "#cdd6f4".to_string(),
                seconday_text: "#bac2de".to_string(),
            },
            web_search: WebSearchSettings {
                default: vec![
                    SearchOption {
                        icon: None,
                        name: "Google".to_string(),
                        keyword: "gg".to_string(),
                        query: "https://google.com/search?q=%s".to_string(),
                    },
                    SearchOption {
                        icon: None,
                        name: "DuckDuckGo".to_string(),
                        keyword: "dd".to_string(),
                        query: "https://duckduckgo.com/?q=%s".to_string(),
                    },
                ],
                custom: vec![],
            },
            extensions: vec![],
        };
    }

    pub fn init() {
        let settings_path = get_settings_path();
        let default_settings = Settings::default_settings();

        if !Path::new(&settings_path).exists() {
            fs::create_dir_all(Path::new(&settings_path).parent().unwrap())
                .expect("Failed to create configs folder");

            let mut settings_file =
                File::create(&settings_path).expect("Failed to create settings file");
            let settings_json =
                serde_json::to_string(&default_settings).expect("Error converting settings json");

            settings_file
                .write_all(&settings_json.as_bytes())
                .expect("Error saving default settings");

            settings_file.flush().unwrap();
        }
    }

    pub fn current_settings() -> Settings {
        let settings_path = get_settings_path();
        let mut settings_file = File::open(&settings_path).expect("Failed to open settings");
        let mut settings_content = String::new();

        settings_file
            .read_to_string(&mut settings_content)
            .expect("Failed to read settings");

        let current_settings =
            serde_json::from_str(&settings_content).unwrap_or(Settings::default_settings());
        return current_settings;
    }

    pub fn update(new_settings: String) -> Result<(), String> {
        let settings_path = get_settings_path();
        let mut settings_file = File::create(&settings_path).expect("Failed to open settings");

        settings_file.write_all(&new_settings.as_bytes()).expect("");

        return match settings_file.flush() {
            Ok(()) => Ok(()),
            Err(error) => Err(error.to_string()),
        };
    }

    pub fn launch_shortcut() -> String {
        let settings = Settings::current_settings();
        let first_key = settings.general.first_key;
        let second_key = settings.general.second_key;
        let third_key = settings.general.third_key;

        return match second_key.is_empty() || second_key.as_str() == "-" {
            true => {
                format!("{first_key}+{third_key}")
            }
            false => {
                format!("{first_key}+{second_key}+{third_key}")
            }
        };
    }

    pub fn search_options() -> Vec<SearchOption> {
        let settings = Settings::current_settings();
        let default_search_options = settings.web_search.default;
        let custom_search_options = settings.web_search.custom;
        let mut search_options: Vec<SearchOption> = Vec::new();

        search_options.extend(default_search_options);
        search_options.extend(custom_search_options);

        return search_options;
    }

    pub fn get_extension_setting(extension_id: String, setting_id: String) -> Result<String, String> {
        let settings = Settings::current_settings();
        let extensions_settings = settings.extensions;

        for extension_setting in extensions_settings {
            if extension_setting.id == extension_id {
                for setting in extension_setting.settings.any{
                    if setting.id == setting_id{
                        return Ok(setting.current_value);
                    }
                }
                for setting in extension_setting.settings.linux{
                    if setting.id == setting_id{
                        return Ok(setting.current_value);
                    }
                }
                for setting in extension_setting.settings.windows{
                    if setting.id == setting_id{
                        return Ok(setting.current_value);
                    }
                }
            }
        }

        return Err("Error getting extension settings".into());
    }

    pub fn update_extension_setting(extension_id: String, setting_id: String, new_value: String) -> Result<(), String>{

        let mut new_settings = Settings::current_settings();

        for (extension_setting_index, extension_setting) in Settings::current_settings().extensions.iter().enumerate(){
            if extension_setting.id == extension_id{
                
                for (any_index, any_setting) in extension_setting.settings.any.iter().enumerate(){
                    if any_setting.id == setting_id{
                        new_settings.extensions[extension_setting_index].settings.any[any_index].current_value = new_value.clone();
                    }
                }

                for (linux_index, linux_setting) in extension_setting.settings.linux.iter().enumerate(){
                    if linux_setting.id == setting_id{
                        new_settings.extensions[extension_setting_index].settings.linux[linux_index].current_value = new_value.clone();
                    }
                }

                for (windows_index, windows_setting) in extension_setting.settings.windows.iter().enumerate(){
                    if windows_setting.id == setting_id{
                        new_settings.extensions[extension_setting_index].settings.windows[windows_index].current_value = new_value.clone(); 
                    }
                }
            }
        }

        return Settings::update(serde_json::to_string(&new_settings).unwrap())
    }
}
