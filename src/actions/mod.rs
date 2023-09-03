use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ResultAction {
    OpenApp(OpenApp),
    OpenInBrowser(OpenInBrowser),
    CopyToClipboard(CopyToClipboard),
    ExtensionAction(ExtensionAction),
    DoNothingAction(DoNothingAction)
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
    pub extension_id: String,
    pub action: String,
    pub args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DoNothingAction{}

impl OpenApp {
    pub fn new(desktop_path: String) -> Self {
        return OpenApp { desktop_path };
    }
}

impl ExtensionAction {
    pub fn new(extension_id: &str, action: &str) -> Self {
        return ExtensionAction {
            extension_id: extension_id.to_owned(),
            action: action.to_owned(),
            args: None,
        };
    }

    pub fn new_with_args(extension_id: &str, action: String, args: Vec<String>) -> Self {

        return ExtensionAction {
            extension_id: extension_id.to_owned(),
            action,
            args: Some(args),
        };
    }
}
