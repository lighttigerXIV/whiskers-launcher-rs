use serde::{Deserialize, Serialize};

use crate::action::Action;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WLResult {
    pub result_type: ResultType,
    pub text: Option<TextResult>,
    pub title_and_description: Option<TitleAndDescriptionResult>,
    pub divider: bool,
}

impl WLResult {
    pub fn new_text(result: TextResult) -> Self {
        Self {
            result_type: ResultType::Text,
            text: Some(result),
            title_and_description: None,
            divider: false,
        }
    }

    pub fn new_title_and_description(result: TitleAndDescriptionResult) -> Self {
        Self {
            result_type: ResultType::TitleAndDescription,
            text: None,
            title_and_description: Some(result),
            divider: false,
        }
    }

    pub fn new_divider() -> Self {
        Self {
            result_type: ResultType::Divider,
            text: None,
            title_and_description: None,
            divider: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResultType {
    Text,
    TitleAndDescription,
    Divider,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextResult {
    pub icon: Option<String>,
    pub tint: Option<String>,
    pub text: String,
    pub action: Action,
}

impl TextResult {
    pub fn new(text: impl Into<String>, action: Action) -> Self {
        Self {
            icon: None,
            tint: None,
            text: text.into(),
            action,
        }
    }

    pub fn icon(&mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self.to_owned()
    }

    pub fn tint(&mut self, tint: impl Into<String>) -> Self {
        self.tint = Some(tint.into());
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TitleAndDescriptionResult {
    pub icon: Option<String>,
    pub tint: Option<String>,
    pub title: String,
    pub description: String,
    pub action: Action,
}

impl TitleAndDescriptionResult {
    pub fn new(title: impl Into<String>, description: impl Into<String>, action: Action) -> Self {
        Self {
            icon: None,
            tint: None,
            title: title.into(),
            description: description.into(),
            action,
        }
    }

    pub fn icon(&mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self.to_owned()
    }

    pub fn tint(&mut self, tint: impl Into<String>) -> Self {
        self.tint = Some(tint.into());
        self.to_owned()
    }
}
