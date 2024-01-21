use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings{
    pub launch_first_key: String,
    pub launch_second_key: Option<String>,
    pub launch_third_key: String,
    pub auto_start: bool,
    pub fraction_scaling: f32,
    pub show_search_icon: bool,
    pub show_settings_icon: bool,
    pub show_placeholder: bool,
    pub layout: String,
    pub border_radius: u8,
    pub border_width: u8,
    pub results_count: u8,
    pub density: String,
    pub blacklist: Vec<String>,
    pub search_engines: Vec<SearchEngine>,
    pub theme: Theme,
    pub extensions: Vec<Extension>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchEngine{
    pub icon_path: String,
    pub tint_icon: bool,
    pub keyword: String,
    pub name: String,
    pub query: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme{
    pub background_main: String,
    pub background_secondary: String,
    pub background_tertiary: String,
    pub accent_primary: String,
    pub accent_danger: String,
    pub text_on_background: String,
    pub text_on_primary: String,
    pub text_on_danger: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Extension{
    pub id: String,
    pub settings: Vec<ExtensionSetting>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionSetting{
    pub id: String,
    pub value: String
}