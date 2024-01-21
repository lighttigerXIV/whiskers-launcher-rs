use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Context {
    pub action: Action,
    pub search_text: Option<String>,
    pub custom_args: Option<Vec<String>>,
}

impl Context {
    pub fn new(action: Action) -> Self {
        return Self {
            action,
            search_text: None,
            custom_args: None,
        };
    }

    pub fn search_text(&mut self, search_text: impl Into<String>) -> Self {
        self.search_text = Some(search_text.into());
        self.to_owned()
    }

    pub fn custom_args(&mut self, custom_args: Vec<String>) -> Self {
        self.custom_args = Some(custom_args);
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    GetResults,
    RunAction,
}

pub mod manifest{
    /** A struct to deserialize the extension manifest
     */
    pub struct Manifest{
        pub id: String,
        pub name: String,
        pub version_name: String,
        pub version_code: usize,
        pub description: String,
        pub os: Vec<String>,
        pub keyword: String,
        pub settings: Vec<Setting>
    }

    pub struct Setting{
        pub id: String,
        pub title: String,
        pub description: String,
        pub setting_type: String,
        pub default_value: String,
        pub os: Vec<String>,
        pub select_options: Option<Vec<SelectOption>>,
        pub show_condition: Option<ShowCondition>,
    }

    pub struct SelectOption{
        pub id: String,
        pub text: String
    }

    pub struct ShowCondition{
        pub setting_id: String,
        pub setting_value: String
    }
}