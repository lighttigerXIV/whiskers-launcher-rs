use std::{fs, io};
use bincode::deserialize;
use serde::{Deserialize, Serialize};
use crate::paths::get_extension_context_path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Context {
    pub action: Action,
    pub search_text: Option<String>,
    pub custom_args: Vec<String>,
}

impl Context {
    pub fn new(action: Action) -> Self {
        return Self {
            action,
            search_text: None,
            custom_args: vec![],
        };
    }

    pub fn search_text(&mut self, search_text: impl Into<String>) -> Self {
        self.search_text = Some(search_text.into());
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

// =====================================================
// Functions
// =====================================================

pub fn write_context(context: Context) -> io::Result<()>{
    let file_path = get_extension_context_path().ok_or(()).unwrap();
    let serialized_context = bincode::serialize(&context).map_err(|_|()).unwrap();
    fs::write(file_path, &serialized_context).map_err(|_|()).unwrap();

    return Ok(());
}

pub fn get_extension_context() -> Option<Context>{

    let context_path = get_extension_context_path()?;
    let file_content = fs::read(context_path).ok()?;
    let deserialized_context: Context = deserialize(&file_content).ok()?;

    return Some(deserialized_context);
}