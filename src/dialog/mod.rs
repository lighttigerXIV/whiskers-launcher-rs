use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum DialogField {
    Input(Input),
    Toggle(Toggle),
    Select(Select),
    TextArea(TextArea),
    SelectFile(SelectFile),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    pub id: String,
    pub value: String,
    pub title: String,
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub custom_args: Option<Vec<String>>,
}

impl Input {
    pub fn new(id: impl Into<String>, title: impl Into<String>, value: impl Into<String>) -> Self {
        return Self {
            id: id.into(),
            value: value.into(),
            title: title.into(),
            description: None,
            placeholder: None,
            custom_args: None,
        };
    }

    pub fn description(&mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        return self.to_owned();
    }

    pub fn placeholder(&mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        return self.to_owned();
    }

    pub fn custom_args(&mut self, custom_args: Vec<String>) -> Self {
        self.custom_args = Some(custom_args);
        return self.to_owned();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Toggle {
    pub id: String,
    pub toggled: bool,
    pub title: String,
    pub description: Option<String>,
    pub custom_args: Option<Vec<String>>,
}

impl Toggle {
    pub fn new(id: impl Into<String>, title: impl Into<String>, toggled: bool) -> Self {
        return Self {
            id: id.into(),
            toggled,
            title: title.into(),
            description: None,
            custom_args: None,
        };
    }

    pub fn description(&mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        return self.to_owned();
    }

    pub fn custom_args(&mut self, custom_args: Vec<String>) -> Self {
        self.custom_args = Some(custom_args);
        return self.to_owned();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Select {
    pub id: String,
    pub default_field_id: String,
    pub title: String,
    pub description: Option<String>,
    pub fields: Vec<SelectField>,
    pub custom_args: Option<Vec<String>>,
}

impl Select {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        default_field_id: impl Into<String>,
        fields: Vec<SelectField>,
    ) -> Self {
        return Self {
            id: id.into(),
            default_field_id: default_field_id.into(),
            title: title.into(),
            description: None,
            fields,
            custom_args: None,
        };
    }

    pub fn description(&mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        return self.to_owned();
    }

    pub fn custom_args(&mut self, custom_args: Vec<String>) -> Self {
        self.custom_args = Some(custom_args);
        return self.to_owned();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectField {
    pub id: String,
    pub text: String,
}

impl SelectField {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        return Self {
            id: id.into(),
            text: text.into(),
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextArea {
    pub id: String,
    pub value: String,
    pub title: String,
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub custom_args: Option<Vec<String>>,
}

impl TextArea {
    pub fn new(id: impl Into<String>, title: impl Into<String>, value: impl Into<String>) -> Self {
        return Self {
            id: id.into(),
            value: value.into(),
            title: title.into(),
            description: None,
            placeholder: None,
            custom_args: None,
        };
    }

    pub fn description(&mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        return self.to_owned();
    }

    pub fn placeholder(&mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        return self.to_owned();
    }

    pub fn custom_args(&mut self, custom_args: Vec<String>) -> Self {
        self.custom_args = Some(custom_args);
        return self.to_owned();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectFile {
    pub id: String,
    pub value: String,
    pub title: String,
    pub description: Option<String>,
    pub select_dir: bool,
    pub default_path: Option<String>,
    pub filters: Vec<FileFilter>,
    pub custom_args: Option<Vec<String>>,
}

impl SelectFile {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        return Self {
            id: id.into(),
            value: "".to_string(),
            title: title.into(),
            description: None,
            select_dir: false,
            default_path: None,
            filters: vec![],
            custom_args: None,
        };
    }

    pub fn description(&mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        return self.to_owned();
    }

    pub fn select_dir(&mut self, select_dir: bool) -> Self {
        self.select_dir = select_dir;
        return self.to_owned();
    }

    pub fn default_path(&mut self, default_path: String) -> Self {
        self.default_path = Some(default_path);
        return self.to_owned();
    }

    pub fn filters(&mut self, filters: Vec<FileFilter>) -> Self {
        self.filters = filters;
        return self.to_owned();
    }

    pub fn custom_args(&mut self, custom_args: Vec<String>) -> Self {
        self.custom_args = Some(custom_args);
        return self.to_owned();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileFilter {
    name: String,
    file_extensions: Vec<String>,
}

impl FileFilter {
    pub fn new(name: impl Into<String>) -> Self {
        return Self {
            name: name.into(),
            file_extensions: vec![],
        };
    }

    pub fn add_extension(&mut self, extension: impl Into<String>) -> Self {
        let mut extensions = self.file_extensions.to_owned();
        extensions.push(extension.into());

        self.file_extensions = extensions;

        return self.to_owned();
    }
}
