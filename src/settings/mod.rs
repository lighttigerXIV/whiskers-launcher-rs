use serde::{Deserialize, Serialize};

use crate::paths::get_app_resources_icons_dir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default = "default_first_key")]
    pub first_key: String,
    #[serde(default = "default_second_key")]
    pub second_key: Option<String>,
    #[serde(default = "default_third_key")]
    pub third_key: String,
    #[serde(default = "default_scaling")]
    pub scaling: f32,
    #[serde(default = "default_auto_start")]
    pub auto_start: bool,
    #[serde(default = "default_show_recent_apps")]
    pub show_recent_apps: bool,
    #[serde(default = "default_split_results")]
    pub split_results: bool,
    #[serde(default = "default_show_search_icon")]
    pub show_search_icon: bool,
    #[serde(default = "default_show_settings_icon")]
    pub show_settings_icon: bool,
    #[serde(default = "default_show_placeholder")]
    pub show_placeholder: bool,
    #[serde(default = "default_accent_search_border")]
    pub accent_search_border: bool,
    #[serde(default = "default_hide_on_blur")]
    pub hide_on_blur: bool,
    #[serde(default = "default_border_radius")]
    pub border_radius: usize,
    #[serde(default = "default_border_width")]
    pub border_width: usize,
    #[serde(default = "default_highlight_selected_background")]
    pub highlight_selected_background: bool,
    #[serde(default = "default_show_alt_hint")]
    pub show_alt_hint: bool,
    #[serde(default = "default_results_count")]
    pub results_count: usize,
    #[serde(default = "default_blacklist")]
    pub blacklist: Vec<String>,
    #[serde(default = "default_search_keyword")]
    pub search_keyword: String,
    #[serde(default = "default_search_engines")]
    pub search_engines: Vec<SearchEngine>,
    #[serde(default = "default_default_search_engine")]
    pub default_search_engine: usize,
    #[serde(default = "default_theme")]
    pub theme: Theme,
    #[serde(default = "default_extensions")]
    pub extensions: Vec<ExtensionSetting>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchEngine {
    pub id: usize,
    pub icon_path: Option<String>,
    pub tint_icon: bool,
    pub keyword: String,
    pub name: String,
    pub search_query: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme {
    pub background: String,
    pub secondary: String,
    pub tertiary: String,
    pub accent: String,
    pub warning: String,
    pub danger: String,
    pub on_accent: String,
    pub on_danger: String,
    pub text: String,
    pub sub_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionSetting {
    pub extension_id: String,
    pub setting_id: String,
    pub setting_value: String,
}

// =================================================================
// Default Settings
// =================================================================

fn default_first_key() -> String {
    String::from("ctrl")
}

fn default_second_key() -> Option<String> {
    None
}

fn default_third_key() -> String {
    String::from("space")
}

fn default_scaling() -> f32 {
    1.0
}

fn default_auto_start() -> bool {
    true
}

fn default_show_recent_apps() -> bool {
    true
}

fn default_split_results() -> bool {
    false
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

fn default_accent_search_border() -> bool {
    false
}

fn default_hide_on_blur() -> bool {
    true
}

fn default_border_radius() -> usize {
    18
}

fn default_border_width() -> usize {
    1
}

fn default_highlight_selected_background() -> bool {
    true
}

fn default_show_alt_hint() -> bool {
    true
}

fn default_results_count() -> usize {
    6
}

fn default_blacklist() -> Vec<String> {
    Vec::new()
}

fn default_search_keyword() -> String {
    String::from("s")
}

fn default_theme() -> Theme {
    Theme {
        background: String::from("#0E0600"),
        secondary: String::from("#140800"),
        tertiary: String::from("#1B0B00"),
        accent: String::from("#FFE072"),
        warning: String::from("#FFB26C"),
        danger: String::from("#FF8C7C"),
        on_accent: String::from("#000000"),
        on_danger: String::from("#000000"),
        text: String::from("#FFEEE2"),
        sub_text: String::from("#E5D2C5"),
    }
}

fn default_search_engines() -> Vec<SearchEngine> {
    let mut google_icon = get_app_resources_icons_dir();
    google_icon.push("google.svg");

    let mut duckduckgo_icon = get_app_resources_icons_dir();
    duckduckgo_icon.push("duckduckgo.svg");

    let mut brave_icon = get_app_resources_icons_dir();
    brave_icon.push("brave.svg");

    vec![
        SearchEngine {
            id: 0,
            icon_path: Some(
                google_icon
                    .into_os_string()
                    .into_string()
                    .expect("Error converting google icon path to string"),
            ),
            tint_icon: true,
            keyword: String::from("gs"),
            name: String::from("Google"),
            search_query: String::from("https://www.google.com/search?q=%s"),
        },
        SearchEngine {
            id: 1,
            icon_path: Some(
                duckduckgo_icon
                    .into_os_string()
                    .into_string()
                    .expect("Error converting duckduckgo icon path to string"),
            ),
            tint_icon: true,
            keyword: String::from("ds"),
            name: String::from("DuckDuckGo"),
            search_query: String::from("https://duckduckgo.com/?q=%s"),
        },
        SearchEngine {
            id: 2,
            icon_path: Some(
                brave_icon
                    .into_os_string()
                    .into_string()
                    .expect("Error converting brave icon path to string"),
            ),
            tint_icon: true,
            keyword: String::from("bs"),
            name: String::from("Brave"),
            search_query: String::from("https://search.brave.com/search?q=%s"),
        },
        SearchEngine {
            id: 3,
            icon_path: None,
            tint_icon: false,
            keyword: String::from("ss"),
            name: String::from("Startpage"),
            search_query: String::from("https://www.startpage.com/do/dsearch?q=%s"),
        },
    ]
}

fn default_default_search_engine() -> usize {
    0
}

fn default_extensions() -> Vec<ExtensionSetting> {
    vec![]
}

pub fn get_default_settings() -> Settings {
    Settings {
        first_key: default_first_key(),
        second_key: default_second_key(),
        third_key: default_third_key(),
        scaling: default_scaling(),
        auto_start: default_auto_start(),
        show_recent_apps: default_show_recent_apps(),
        split_results: default_split_results(),
        show_search_icon: default_show_search_icon(),
        show_settings_icon: default_show_settings_icon(),
        show_placeholder: default_show_placeholder(),
        accent_search_border: default_accent_search_border(),
        hide_on_blur: default_hide_on_blur(),
        border_radius: default_border_radius(),
        border_width: default_border_width(),
        highlight_selected_background: default_highlight_selected_background(),
        show_alt_hint: default_show_alt_hint(),
        results_count: default_results_count(),
        blacklist: default_blacklist(),
        search_keyword: default_search_keyword(),
        search_engines: default_search_engines(),
        default_search_engine: default_default_search_engine(),
        theme: default_theme(),
        extensions: default_extensions(),
    }
}
