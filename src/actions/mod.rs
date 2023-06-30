use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ResultAction {
    OpenApp(OpenApp),
    OpenInBrowser(OpenInBrowser),
    CopyToClipboard(CopyToClipboard),
    ExtensionAction(ExtensionAction),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenApp {
    pub desktop_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenInBrowser {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyToClipboard {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtensionAction {
    pub extension_name: String,
    pub args: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunCommandAction{
    pub main_command: String,
    pub args: String
}

impl OpenApp {
    pub fn new(desktop_path: String) -> Self {
        return OpenApp { desktop_path };
    }
}
