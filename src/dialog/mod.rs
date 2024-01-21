use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum DialogField{
    Input(Input),
    Toggle(Toggle),
    Select(Select)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    pub id: String,
    pub value: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub custom_args: Option<Vec<String>>
}

impl Input{
    pub fn new(id: impl Into<String>, value: impl Into<String>) -> Self{
        return Self{
            id: id.into(),
            value: value.into(),
            title: None,
            description: None,
            placeholder: None,
            custom_args: None
        }
    }

    pub fn title(&mut self, title: impl Into<String>) -> Self{
        self.title = Some(title.into());
        return self.to_owned();
    }

    pub fn description(&mut self, description: impl Into<String>) -> Self{
        self.description = Some(description.into());
        return self.to_owned();
    }

    pub fn placeholder(&mut self, description: impl Into<String>) -> Self{
        self.description = Some(description.into());
        return self.to_owned();
    }

    pub fn custom_args(&mut self, custom_args: Vec<String>) -> Self{
        self.custom_args = Some(custom_args);
        return self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Toggle {
    pub id: String,
    pub toggled: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub custom_args: Option<Vec<String>>
}

impl Toggle {

    pub fn new(id: impl Into<String>, toggled: bool) -> Self{
        return Self{
            id: id.into(),
            toggled,
            title: None,
            description: None,
            custom_args: None
        }
    }

    pub fn title(&mut self, title: impl Into<String>) -> Self{
        self.title = Some(title.into());
        return self.to_owned();
    }

    pub fn description(&mut self, description: impl Into<String>) -> Self{
        self.description = Some(description.into());
        return self.to_owned();
    }

    pub fn custom_args(&mut self, custom_args: Vec<String>) -> Self{
        self.custom_args = Some(custom_args);
        return self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Select {
    pub id: String,
    pub default_field_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub fields: Vec<SelectField>,
    pub custom_args: Option<Vec<String>>
}

impl Select {
    pub fn new(id: impl Into<String>, default_field_id: impl Into<String>, fields: Vec<SelectField>) -> Self{
        return Self{
            id: id.into(),
            default_field_id: default_field_id.into(),
            title: None,
            description: None,
            fields,
            custom_args: None
        }
    }

    pub fn title(&mut self, title: impl Into<String>) -> Self{
        self.title = Some(title.into());
        return self.to_owned();
    }

    pub fn description(&mut self, description: impl Into<String>) -> Self{
        self.description = Some(description.into());
        return self.to_owned();
    }

    pub fn custom_args(&mut self, custom_args: Vec<String>) -> Self{
        self.custom_args = Some(custom_args);
        return self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectField{
    pub id: String,
    pub text: String
}

impl SelectField {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self{
        return Self{
            id: id.into(),
            text: text.into()
        }
    }
}