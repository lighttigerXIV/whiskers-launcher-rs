use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::paths::{get_resources_directory, get_settings_path};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub general: GeneralSettings,
    pub search_box: SearchBoxSettings,
    pub theme: Theme,
    pub search_engines: Vec<SearchEngine>,
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
pub struct ThemeSettings {
    pub current: Theme,
    pub themes: Vec<Theme>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub background: String,
    pub secondary_background: String,
    pub tertiary_background: String,
    pub accent: String,
    pub on_accent: String,
    pub danger: String,
    pub on_danger: String,
    pub text: String,
    pub secondary_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchEngine {
    pub keyword: String,
    pub icon: Option<String>,
    pub tint_icon: bool,
    pub name: String,
    pub query: String,
    pub default: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtensionsSettings {
    pub id: String,
    pub keyword: String,
    pub settings: ExtensionSetting,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionSetting {
    pub any: Vec<ExtensionOptionSetting>,
    pub linux: Vec<ExtensionOptionSetting>,
    pub windows: Vec<ExtensionOptionSetting>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionOptionSetting {
    pub id: String,
    pub current_value: String,
}

impl Settings {
    pub fn default_settings() -> Settings {
        let mut google_svg_path = get_resources_directory().unwrap();
        google_svg_path.push("images/google.svg");

        let mut brave_svg_path = get_resources_directory().unwrap();
        brave_svg_path.push("images/brave.svg");

        let mut duckduckgo_svg_path = get_resources_directory().unwrap();
        duckduckgo_svg_path.push("images/duckduckgo.svg");

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
            theme: Theme {
                background: "#1e1e2e".to_string(),
                secondary_background: "#11111b".to_string(),
                tertiary_background: "#181825".to_string(),
                accent: "#89b4fa".to_string(),
                on_accent: "#1e1e2e".to_string(),
                danger: "#f38ba8".to_string(),
                on_danger: "#1e1e2e".to_string(),
                text: "#cdd6f4".to_string(),
                secondary_text: "#bac2de".to_string(),
            },
            search_engines: vec![
                SearchEngine {
                    icon: Some(google_svg_path.into_os_string().into_string().unwrap()),
                    tint_icon: true,
                    name: "Google".to_string(),
                    keyword: "gg".to_string(),
                    query: "https://google.com/search?q=%s".to_string(),
                    default: true,
                },
                SearchEngine {
                    icon: Some(duckduckgo_svg_path.into_os_string().into_string().unwrap()),
                    tint_icon: true,
                    name: "DuckDuckGo".to_string(),
                    keyword: "dd".to_string(),
                    query: "https://duckduckgo.com/?q=%s".to_string(),
                    default: false,
                },
                SearchEngine {
                    icon: Some(brave_svg_path.into_os_string().into_string().unwrap()),
                    tint_icon: true,
                    name: "Brave Search".to_string(),
                    keyword: "bs".to_string(),
                    query: "https://search.brave.com/search?q=%s".to_string(),
                    default: false,
                },
            ],
            extensions: vec![],
        };
    }

    pub fn init() {
        let settings_path = get_settings_path().unwrap();

        if !settings_path.exists() {
            fs::create_dir_all(Path::new(&settings_path).parent().unwrap())
                .expect("Failed to create configs folder");

            let mut settings_file = File::create(&settings_path)
                .expect("Failed to create settings file");

            let settings_yaml = serde_yaml::to_string(&Settings::default_settings())
                .expect("Error converting settings yaml");

            settings_file.write_all(&settings_yaml.as_bytes())
                .expect("Error saving settings");

            settings_file.flush()
                .expect("Error closing settings file");
        }
    }

    pub fn current_settings() -> Option<Settings> {
        let settings_path = get_settings_path().unwrap();

        let mut settings_file = File::open(&settings_path)
            .expect("Failed to open settings");

        let mut settings_content = String::new();

        settings_file
            .read_to_string(&mut settings_content)
            .expect("Failed to read settings");

        let current_settings = serde_yaml::from_str(&settings_content)
            .expect("Error getting current settings");

        return Some(current_settings);
    }

    pub fn update(new_settings: String) -> Result<(), String> {
        let settings_path = get_settings_path().unwrap();
        let mut settings_file = File::create(&settings_path).expect("Failed to open settings");

        settings_file.write_all(&new_settings.as_bytes()).expect("");

        return match settings_file.flush() {
            Ok(()) => Ok(()),
            Err(error) => Err(error.to_string()),
        };
    }

    pub fn launch_shortcut() -> String {
        let settings = Settings::current_settings().unwrap();
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

    pub fn get_extension_setting(
        extension_id: &str,
        setting_id: &str,
    ) -> Result<String, String> {
        let settings = Settings::current_settings().unwrap();
        let extensions_settings = settings.extensions;

        for extension_setting in extensions_settings {
            if extension_setting.id == extension_id {
                for setting in extension_setting.settings.any {
                    if setting.id == setting_id {
                        return Ok(setting.current_value);
                    }
                }
                for setting in extension_setting.settings.linux {
                    if setting.id == setting_id {
                        return Ok(setting.current_value);
                    }
                }
                for setting in extension_setting.settings.windows {
                    if setting.id == setting_id {
                        return Ok(setting.current_value);
                    }
                }
            }
        }

        return Err("Error getting extension settings".into());
    }

    pub fn update_extension_setting(
        extension_id: String,
        setting_id: String,
        new_value: String,
    ) -> Result<(), String> {
        let mut new_settings = Settings::current_settings().unwrap();

        for (extension_setting_index, extension_setting) in
        Settings::current_settings().unwrap().extensions.iter().enumerate()
        {
            if extension_setting.id == extension_id {
                for (any_index, any_setting) in extension_setting.settings.any.iter().enumerate() {
                    if any_setting.id == setting_id {
                        new_settings.extensions[extension_setting_index]
                            .settings
                            .any[any_index]
                            .current_value = new_value.clone();
                    }
                }

                for (linux_index, linux_setting) in
                extension_setting.settings.linux.iter().enumerate()
                {
                    if linux_setting.id == setting_id {
                        new_settings.extensions[extension_setting_index]
                            .settings
                            .linux[linux_index]
                            .current_value = new_value.clone();
                    }
                }

                for (windows_index, windows_setting) in
                extension_setting.settings.windows.iter().enumerate()
                {
                    if windows_setting.id == setting_id {
                        new_settings.extensions[extension_setting_index]
                            .settings
                            .windows[windows_index]
                            .current_value = new_value.clone();
                    }
                }
            }
        }

        return Settings::update(serde_yaml::to_string(&new_settings).unwrap());
    }

    pub fn update_extension_keyword(extension_id: String, keyword: String) -> Result<(), String> {
        let mut new_settings = Settings::current_settings()
            .expect("Error getting settings");

        for (extension_setting_index, extension_setting) in Settings::current_settings().expect("Error getting settings").extensions.iter().enumerate()
        {
            if extension_setting.id == extension_id {
                new_settings.extensions[extension_setting_index].keyword = keyword.clone();
            }
        }

        return Settings::update(serde_yaml::to_string(&new_settings).unwrap());
    }
}
