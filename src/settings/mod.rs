use crate::paths::{get_app_resources_icons_dir, get_autostart_path, get_settings_path};

#[cfg(target_os = "windows")]
use crate::paths::get_app_resources_dir;

use serde::{Deserialize, Serialize};
use std::{env, fs, io, os::unix::fs::PermissionsExt};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    #[serde(default = "default_launch_first_key")]
    pub launch_first_key: String,
    #[serde(default = "default_launch_second_key")]
    pub launch_second_key: Option<String>,
    #[serde(default = "default_launch_third_key")]
    pub launch_third_key: String,
    #[serde(default = "default_auto_start")]
    pub auto_start: bool,
    #[serde(default = "default_hide_on_blur")]
    pub hide_on_blur: bool,
    #[serde(default = "default_fractional_scaling")]
    pub fractional_scaling: f32,
    #[serde(default = "default_show_search_icon")]
    pub show_search_icon: bool,
    #[serde(default = "default_show_settings_icon")]
    pub show_settings_icon: bool,
    #[serde(default = "default_show_placeholder")]
    pub show_placeholder: bool,
    #[serde(default = "default_layout")]
    pub layout: String,
    #[serde(default = "default_border_radius")]
    pub border_radius: u8,
    #[serde(default = "default_border_width")]
    pub border_width: u8,
    #[serde(default = "default_results_count")]
    pub results_count: u8,
    #[serde(default = "default_density")]
    pub density: String,
    #[serde(default = "default_blacklist")]
    pub blacklist: Vec<String>,
    #[serde(default = "default_search_engines")]
    pub search_engines: Vec<SearchEngine>,
    #[serde(default = "default_theme")]
    pub theme: Theme,
    #[serde(default = "default_extensions")]
    pub extensions: Vec<Extension>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchEngine {
    pub icon_path: Option<String>,
    pub tint_icon: bool,
    pub keyword: String,
    pub name: String,
    pub query: String,
    pub default: bool,
}

impl SearchEngine {
    pub fn new(
        tint_icon: bool,
        keyword: impl Into<String>,
        name: impl Into<String>,
        query: impl Into<String>,
        default: bool,
    ) -> Self {
        return Self {
            icon_path: None,
            tint_icon,
            keyword: keyword.into(),
            name: name.into(),
            query: query.into(),
            default,
        };
    }

    pub fn icon_path(&mut self, icon_path: impl Into<String>) -> Self {
        self.icon_path = Some(icon_path.into());
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme {
    pub background_main: String,
    pub background_secondary: String,
    pub background_tertiary: String,
    pub accent_primary: String,
    pub accent_danger: String,
    pub text_on_background: String,
    pub text_on_primary: String,
    pub text_on_danger: String,
}

impl Theme {
    pub fn new(
        background_main: impl Into<String>,
        background_secondary: impl Into<String>,
        background_tertiary: impl Into<String>,
        accent_primary: impl Into<String>,
        accent_danger: impl Into<String>,
        text_on_background: impl Into<String>,
        text_on_primary: impl Into<String>,
        text_on_danger: impl Into<String>,
    ) -> Self {
        Self {
            background_main: background_main.into(),
            background_secondary: background_secondary.into(),
            background_tertiary: background_tertiary.into(),
            accent_primary: accent_primary.into(),
            accent_danger: accent_danger.into(),
            text_on_background: text_on_background.into(),
            text_on_primary: text_on_primary.into(),
            text_on_danger: text_on_danger.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Extension {
    pub id: String,
    pub keyword: String,
    pub settings: Vec<ExtensionSetting>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionSetting {
    pub id: String,
    pub value: String,
}

impl ExtensionSetting {
    pub fn new(id: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
        }
    }
}

// ==============================
// Default Values
// ==============================

fn default_launch_first_key() -> String {
    "ctrl".to_owned()
}

fn default_launch_second_key() -> Option<String> {
    None
}

fn default_launch_third_key() -> String {
    "space".to_owned()
}

fn default_auto_start() -> bool {
    true
}

fn default_hide_on_blur() -> bool {
    true
}

fn default_fractional_scaling() -> f32 {
    1.0
}

fn default_show_search_icon() -> bool {
    true
}

fn default_show_settings_icon() -> bool {
    true
}

fn default_show_placeholder() -> bool {
    true
}

fn default_layout() -> String {
    "modern".to_owned()
}

fn default_border_radius() -> u8 {
    24
}

fn default_border_width() -> u8 {
    2
}

fn default_results_count() -> u8 {
    6
}

fn default_density() -> String {
    "medium".to_owned()
}

fn default_blacklist() -> Vec<String> {
    vec![]
}

fn default_search_engines() -> Vec<SearchEngine> {
    let mut google_icon = get_app_resources_icons_dir().unwrap();
    let mut duckduckgo_icon = get_app_resources_icons_dir().unwrap();
    let mut brave_icon = get_app_resources_icons_dir().unwrap();

    google_icon.push("google.svg");
    duckduckgo_icon.push("duckduckgo.svg");
    brave_icon.push("brave.svg");

    vec![
        SearchEngine::new(
            true,
            "gs",
            "Google Search",
            "https://www.google.com/search?q=%s",
            true,
        )
        .icon_path(google_icon.into_os_string().into_string().unwrap()),
        SearchEngine::new(
            true,
            "ds",
            "DuckDuckGo Search",
            "https://duckduckgo.com/?q=%s",
            false,
        )
        .icon_path(duckduckgo_icon.into_os_string().into_string().unwrap()),
        SearchEngine::new(
            true,
            "bs",
            "Brave Search",
            "https://search.brave.com/search?q=%s",
            false,
        )
        .icon_path(brave_icon.into_os_string().into_string().unwrap()),
    ]
}

fn default_theme() -> Theme {
    Theme::new(
        "#221A00", "#403200", "#5B4700", "#FFDB5D", "#FF7373", "#FFE792", "#221A00", "#221A00",
    )
}

fn default_extensions() -> Vec<Extension> {
    vec![]
}

pub fn get_default_settings() -> Settings {
    Settings {
        launch_first_key: default_launch_first_key(),
        launch_second_key: default_launch_second_key(),
        launch_third_key: default_launch_third_key(),
        hide_on_blur: default_hide_on_blur(),
        auto_start: default_auto_start(),
        fractional_scaling: default_fractional_scaling(),
        show_search_icon: default_show_search_icon(),
        show_settings_icon: default_show_settings_icon(),
        show_placeholder: default_show_placeholder(),
        layout: default_layout(),
        border_radius: default_border_radius(),
        border_width: default_border_width(),
        results_count: default_results_count(),
        density: default_density(),
        blacklist: default_blacklist(),
        search_engines: default_search_engines(),
        theme: default_theme(),
        extensions: default_extensions(),
    }
}

// ================================
// Settings
// ================================
impl Settings {
    pub fn set_launch_first_key(&mut self, value: impl Into<String>) -> Self {
        self.launch_first_key = value.into();
        self.to_owned()
    }

    pub fn set_launch_second_key(&mut self, value: Option<String>) -> Self {
        self.launch_second_key = value;
        self.to_owned()
    }

    pub fn set_launch_third_key(&mut self, value: impl Into<String>) -> Self {
        self.launch_third_key = value.into();
        self.to_owned()
    }

    pub fn set_auto_start(&mut self, value: bool) -> Self {
        self.auto_start = value;
        self.to_owned()
    }

    pub fn set_hide_on_blur(&mut self, value: bool) -> Self {
        self.hide_on_blur = value;
        self.to_owned()
    }

    pub fn handle_autostart(self) {
        let path = get_autostart_path().ok_or(()).unwrap();
        let settings = get_settings().ok_or(()).unwrap();

        if !path.exists() && settings.auto_start {
            fs::create_dir_all(&path.parent().ok_or(()).unwrap())
                .map_err(|_| ())
                .unwrap();
        }

        match env::consts::OS {
            "linux" => {
                let desktop_content = r#"[Desktop Entry]
Version=0.1
Type=Application
Name=Whiskers Launcher Companion
Comment=Whiskers Launcher companion tray app
Terminal=false
StartupNotify=false
Exec=sh -c '/usr/bin/whiskers-launcher-companion'"#;

                let mut desktop_file_path = path.to_owned();
                desktop_file_path.push("whiskers-launcher.desktop");

                if self.auto_start {
                    fs::write(&desktop_file_path, &desktop_content)
                        .map_err(|_| ())
                        .unwrap();

                    // Gives read and write permissions so that it can be executed on autostart
                    fs::set_permissions(&desktop_file_path, fs::Permissions::from_mode(0o755))
                        .map_err(|_| ())
                        .unwrap();
                } else {
                    if desktop_file_path.exists() {
                        fs::remove_file(&desktop_file_path).map_err(|_| ()).unwrap();
                    }
                }
            }
            #[cfg(target_os = "windows")]
            "windows" => {
                let script = if self.auto_start {
                    "enable-autostart.ps1"
                } else {
                    "disable-autostart.ps1"
                };

                let mut path = get_app_resources_dir().unwrap();
                path.push("Scripts");
                path.push(script);

                let script_content = fs::read_to_string(&path).map_err(|_| ()).unwrap();
                powershell_script::run(&script_content)
                    .map_err(|_| ())
                    .unwrap();
            }
            _ => {}
        }
    }

    pub fn set_fraction_scaling(&mut self, value: f32) -> Self {
        self.fractional_scaling = value;
        self.to_owned()
    }

    pub fn set_show_search_icon(&mut self, value: bool) -> Self {
        self.show_search_icon = value;
        self.to_owned()
    }

    pub fn set_show_settings_icon(&mut self, value: bool) -> Self {
        self.show_settings_icon = value;
        self.to_owned()
    }

    pub fn set_show_placeholder(&mut self, value: bool) -> Self {
        self.show_placeholder = value;
        self.to_owned()
    }

    pub fn set_layout(&mut self, value: impl Into<String>) -> Self {
        self.layout = value.into();
        self.to_owned()
    }

    pub fn set_border_radius(&mut self, value: u8) -> Self {
        self.border_radius = value;
        self.to_owned()
    }

    pub fn set_border_width(&mut self, value: u8) -> Self {
        self.border_width = value;
        self.to_owned()
    }

    pub fn set_results_count(&mut self, value: u8) -> Self {
        self.results_count = value;
        self.to_owned()
    }

    pub fn set_density(&mut self, value: impl Into<String>) -> Self {
        self.density = value.into();
        self.to_owned()
    }

    pub fn set_blacklist(&mut self, value: Vec<String>) -> Self {
        self.blacklist = value;
        self.to_owned()
    }

    pub fn set_search_engines(&mut self, value: bool) -> Self {
        self.auto_start = value;
        self.to_owned()
    }

    pub fn set_theme(&mut self, value: Theme) -> Self {
        self.theme = value;
        self.to_owned()
    }

    pub fn set_extensions(&mut self, value: Vec<Extension>) -> Self {
        self.extensions = value;
        self.to_owned()
    }

    pub fn update_extension_setting(
        &mut self,
        extension_id: String,
        setting_id: String,
        value: String,
    ) -> Self {
        let mut new_extensions: Vec<Extension> = vec![];

        for extension in self.extensions.to_owned() {
            if extension.id == extension_id {
                let mut new_settings: Vec<ExtensionSetting> = vec![];

                for setting in extension.settings {
                    if setting.id == setting_id {
                        new_settings.push(ExtensionSetting {
                            id: setting.id,
                            value: value.to_owned(),
                        });
                    } else {
                        new_settings.push(setting);
                    }
                }
            } else {
                new_extensions.push(extension);
            }
        }

        self.extensions = new_extensions;

        return self.to_owned();
    }
}

pub fn index_settings() -> io::Result<()> {
    let settings_path = get_settings_path().ok_or(()).unwrap();
    let parent = settings_path.parent().ok_or(()).unwrap();

    if !parent.exists() {
        fs::create_dir_all(&parent).map_err(|_| ()).unwrap();
    }

    if !settings_path.exists() {
        let json_settings = serde_json::to_string_pretty(&get_default_settings())
            .map_err(|_| ())
            .unwrap();
        fs::write(&settings_path, &json_settings)
            .map_err(|_| ())
            .unwrap();
    }

    get_settings().ok_or(()).unwrap().handle_autostart();

    Ok(())
}

pub fn get_settings() -> Option<Settings> {
    let settings_path = get_settings_path()?;
    let settings_content = fs::read_to_string(&settings_path).ok()?;

    return match serde_json::from_str(&settings_content) {
        Ok(settings) => Some(settings),
        Err(_) => Some(get_default_settings()),
    };
}

pub fn update_settings(settings: Settings) -> io::Result<()> {
    //Updates Settings
    let settings_path = get_settings_path().ok_or(()).unwrap();
    let json_settings = serde_json::to_string_pretty(&settings)
        .map_err(|_| ())
        .unwrap();
    fs::write(&settings_path, &json_settings)
        .map_err(|_| ())
        .unwrap();

    settings.handle_autostart();

    Ok(())
}
