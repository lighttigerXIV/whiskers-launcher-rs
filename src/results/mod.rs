use crate::actions::ResultAction;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub enum SimpleKLResult{
    Text(TextResult),
    IconWithText(IconWithTextResult),
    TitleAndDescription(TitleAndDescriptionResult),
    IconWithTitleAndDescription(IconWithTitleAndDescriptionResult)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextResult {
    pub text: String,
    pub action: ResultAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IconWithTextResult {
    pub icon: String,
    pub text: String,
    pub action: ResultAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TitleAndDescriptionResult {
    pub title: String,
    pub description: String,
    pub action: ResultAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IconWithTitleAndDescriptionResult {
    pub icon: String,
    pub title: String,
    pub description: String,
    pub action: ResultAction,
}

impl TextResult {
    pub fn new(text: String, action: ResultAction) -> Self {
        return TextResult { text, action };
    }
}

impl IconWithTextResult {
    pub fn new(icon: String, text: String, action: ResultAction) -> Self {
        return IconWithTextResult { icon, text, action };
    }
}

impl TitleAndDescriptionResult {
    pub fn new(title: String, description: String, action: ResultAction) -> Self {
        return TitleAndDescriptionResult {
            title,
            description,
            action,
        };
    }
}

impl IconWithTitleAndDescriptionResult {
    pub fn new(icon:String, title: String, description: String, action: ResultAction) -> Self {
        return IconWithTitleAndDescriptionResult {
            icon,
            title,
            description,
            action,
        };
    }
}

