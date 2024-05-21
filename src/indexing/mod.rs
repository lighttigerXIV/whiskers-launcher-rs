use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub path: String,
}

impl App {
    pub fn new(id: impl Into<String>, title: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: None,
            path: path.into(),
        }
    }

    pub fn icon(&mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self.to_owned()
    }
}