use crate::{
    paths::{get_extension_parameters_path, get_extension_results_path},
    results::SimpleKLResult,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File},
    io::{Read, Write},
    process::exit,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    pub function: Function,
    pub search_text: Option<String>,
    pub action: Option<String>,
    pub custom_args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Function {
    GetResults,
    RunAction,
}

pub fn get_parameters() -> Parameters {
    let mut parameters_file =
        File::open(get_extension_parameters_path()).expect("Error opening parameters file");
    let mut parameters_json = String::from("");

    parameters_file
        .read_to_string(&mut parameters_json)
        .expect("Error reading parameters file");
    parameters_file
        .flush()
        .expect("Error closing parameters file");

    let parameters = serde_json::from_str(&parameters_json).expect("Error getting parameters");
    return parameters;
}

pub fn return_results(results: Vec<SimpleKLResult>) {
    let mut results_path_file =
        File::create(get_extension_results_path()).expect("Error opening extension results");
    let results_json = serde_json::to_string(&results).expect("Error converting results to json");

    results_path_file
        .write_all(&results_json.as_bytes())
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
    pub description: String,
    pub icon: String,
    pub os: Vec<String>,
    pub keyword: String,
    pub settings: ExtensionSettings,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExtensionSettings {
    pub any: Vec<ExtensionSetting>,
    pub linux: Vec<ExtensionSetting>,
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

