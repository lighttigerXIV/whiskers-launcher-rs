use std::path::PathBuf;

use crate::actions::ResultAction;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SimpleKLResult {
    IconWithTitleAndDescription(IconWithTitleAndDescriptionResult),
    TitleAndDescription(TitleAndDescriptionResult),
    IconWithText(IconWithTextResult),
    Text(TextResult),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextResult {
    pub text: String,
    pub action: ResultAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IconWithTextResult {
    pub icon: String,
    pub icon_color: Option<String>,
    pub text: String,
    pub action: ResultAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TitleAndDescriptionResult {
    pub title: String,
    pub description: String,
    pub action: ResultAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IconWithTitleAndDescriptionResult {
    pub icon: String,
    pub icon_color: Option<String>,
    pub title: String,
    pub description: String,
    pub action: ResultAction,
}

impl TextResult {
    pub fn new(text: &str, action: ResultAction) -> Self {
        return TextResult { text: text.to_owned(), action };
    }
}

impl IconWithTextResult {
    pub fn new(icon: PathBuf, text: &str, action: ResultAction) -> Self {
        return IconWithTextResult {
            icon: icon.into_os_string().into_string().unwrap(),
            icon_color: None,
            text: text.to_owned(),
            action,
        };
    }

    pub fn new_with_color(icon: PathBuf, text: &str, action: ResultAction) -> Self {
        return IconWithTextResult {
            icon: icon.into_os_string().into_string().unwrap(),
            icon_color: Some("accent".to_owned()),
            text: text.to_owned(),
            action,
        };
    }

    pub fn color(&mut self, icon_color: &str) -> &mut Self {
        self.icon_color = Some(icon_color.to_owned());
        self
    }
}

impl TitleAndDescriptionResult {
    pub fn new(title: &str, description: &str, action: ResultAction) -> Self {
        return TitleAndDescriptionResult {
            title: title.to_owned(),
            description: description.to_owned(),
            action,
        };
    }
}

impl IconWithTitleAndDescriptionResult {
    pub fn new(icon: PathBuf, title: &str, description: &str, action: ResultAction) -> Self {
        return IconWithTitleAndDescriptionResult {
            icon: icon.into_os_string().into_string().unwrap(),
            icon_color: None,
            title: title.to_owned(),
            description: description.to_owned(),
            action,
        };
    }

    pub fn new_with_color(icon: PathBuf, icon_color: &str, title: &str, description: &str, action: ResultAction) -> Self {
        return IconWithTitleAndDescriptionResult {
            icon: icon.into_os_string().into_string().unwrap(),
            icon_color: Some(icon_color.to_owned()),
            title: title.to_owned(),
            description: description.to_owned(),
            action,
        };
    }
}
