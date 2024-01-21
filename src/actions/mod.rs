use serde::{Deserialize, Serialize};
use crate::dialog::DialogField;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Action {
    OpenApp(OpenApp),
    OpenUrl(OpenUrl),
    CopyToClipboard(CopyToClipboard),
    Extension(Extension),
    Dialog(Dialog),
    Nothing
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenApp {
    path: String,
}

impl OpenApp {
    pub fn new(path: impl Into<String>) -> Self {
        return Self {
            path: path.into()
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenUrl {
    url: String,
}

impl OpenUrl {
    pub fn new(url: impl Into<String>) -> Self {
        return Self {
            url: url.into()
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyToClipboard {
    text: String,
}

impl CopyToClipboard {
    pub fn new(text: impl Into<String>) -> Self {
        return Self {
            text: text.into()
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Extension {
    extension_id: String,
    extension_action: String,
    args: Option<Vec<String>>,
}

impl Extension {
    pub fn new(extension_id: impl Into<String>, extension_action: impl Into<String>) -> Self {
        return Self {
            extension_id: extension_id.into(),
            extension_action: extension_action.into(),
            args: None,
        };
    }

    pub fn args(&mut self, args: Vec<String>) -> Self {
        self.args = Some(args.into());
        return self.to_owned();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dialog {
    pub extension_id: String,
    pub extension_action: String,
    pub fields: Vec<DialogField>,
    pub custom_args: Option<Vec<String>>,
}

