use serde::{Deserialize, Serialize};
use crate::actions::Action;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WhiskersResult {
    Text(Text),
    TitleAndText(TitleAndText),
    Divider,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    pub icon: Option<String>,
    pub tint_icon: bool,
    pub tint_color: Option<String>,
    pub text: String,
    pub action: Action,
}


impl Text {
    pub fn new(text: impl Into<String>, action: Action) -> Self {
        return Self {
            icon: None,
            tint_icon: false,
            tint_color: None,
            text: text.into(),
            action,
        };
    }

    pub fn icon(&mut self, icon_path: impl Into<String>) -> Self {
        self.icon = Some(icon_path.into());
        return self.to_owned();
    }

    pub fn tint_icon(&mut self, tint_icon: bool) -> Self{
        self.tint_icon = tint_icon;
        return self.to_owned();
    }

    pub fn tint_color(&mut self, tint_color: impl Into<String>) -> Self{
        self.tint_color = Some(tint_color.into());
        return self.to_owned();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TitleAndText {
    pub icon: Option<String>,
    pub tint_icon: bool,
    pub tint_color: Option<String>,
    pub title: String,
    pub text: String,
    pub action: Action,
}

impl TitleAndText {
    pub fn new(title: impl Into<String>, text: impl Into<String>, action: Action) -> Self {
        return Self {
            icon: None,
            tint_icon: false,
            tint_color: None,
            title: title.into(),
            text: text.into(),
            action,
        };
    }

    pub fn icon(&mut self, icon_path: impl Into<String>) -> TitleAndText {
        self.icon = Some(icon_path.into());
        return self.to_owned();
    }

     pub fn tint_icon(&mut self, tint_icon: bool) -> Self{
        self.tint_icon = tint_icon;
        return self.to_owned();
    }

    pub fn tint_color(&mut self, tint_color: impl Into<String>) -> Self{
        self.tint_color = Some(tint_color.into());
        return self.to_owned();
    }
}
