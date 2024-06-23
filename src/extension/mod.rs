use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Extension {
    pub id: String,
    pub name: String,
    pub description: String,
    pub keyword: String,
    #[serde(default = "default_settings")]
    pub settings: Option<Vec<ExtensionSetting>>,
    #[serde(default = "default_os")]
    pub os: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionSetting {
    pub id: String,
    pub title: String,
    pub description: String,
    pub setting_type: SettingType,
    pub default_value: String,
    #[serde(default = "default_show_conditions")]
    pub show_conditions: Option<Vec<ShowCondition>>,
    #[serde(default = "default_select_options")]
    pub select_options: Option<Vec<SelectOption>>,
    #[serde(default = "default_os")]
    pub os: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SettingType {
    Input,
    TextArea,
    Select,
    Toggle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShowCondition {
    pub setting_id: String,
    pub setting_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectOption {
    pub id: String,
    pub value: String,
}

fn default_settings() -> Option<Vec<ExtensionSetting>> {
    None
}

fn default_os() -> String {
    "*".to_string()
}

fn default_show_conditions() -> Option<Vec<ShowCondition>> {
    None
}

fn default_select_options() -> Option<Vec<SelectOption>> {
    None
}
