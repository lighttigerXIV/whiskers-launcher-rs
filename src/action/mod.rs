use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub action_type: ActionType,
    pub open_app: Option<OpenAppAction>,
    pub open_url: Option<OpenURLAction>,
    pub copy: Option<CopyAction>,
    pub extension: Option<ExtensionAction>,
    pub dialog: Option<DialogAction>,
    pub ignore: bool,
    pub ask_confirmation: bool,
}

impl Action {
    pub fn new_open_app(action: OpenAppAction) -> Self {
        Self {
            action_type: ActionType::OpenApp,
            open_app: Some(action),
            open_url: None,
            copy: None,
            extension: None,
            dialog: None,
            ignore: false,
            ask_confirmation: false,
        }
    }

    pub fn new_open_url(action: OpenURLAction) -> Self {
        Self {
            action_type: ActionType::OpenURL,
            open_app: None,
            open_url: Some(action),
            copy: None,
            extension: None,
            dialog: None,
            ignore: false,
            ask_confirmation: false,
        }
    }

    pub fn new_copy(action: CopyAction) -> Self {
        Self {
            action_type: ActionType::Copy,
            open_app: None,
            open_url: None,
            copy: Some(action),
            extension: None,
            dialog: None,
            ignore: false,
            ask_confirmation: false,
        }
    }

    pub fn new_extension(action: ExtensionAction) -> Self {
        Self {
            action_type: ActionType::Extension,
            open_app: None,
            open_url: None,
            copy: None,
            extension: Some(action),
            dialog: None,
            ignore: false,
            ask_confirmation: false,
        }
    }

    pub fn new_dialog(action: DialogAction) -> Self {
        Self {
            action_type: ActionType::Dialog,
            open_app: None,
            open_url: None,
            copy: None,
            extension: None,
            dialog: Some(action),
            ignore: false,
            ask_confirmation: false,
        }
    }

    pub fn new_ignore() -> Self {
        Self {
            action_type: ActionType::Ignore,
            open_app: None,
            open_url: None,
            copy: None,
            extension: None,
            dialog: None,
            ignore: true,
            ask_confirmation: false,
        }
    }

    pub fn ask_confirmation(&mut self, ask_confirmation: bool) -> Self {
        self.ask_confirmation = ask_confirmation;
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionType {
    OpenApp,
    OpenURL,
    Copy,
    Extension,
    Dialog,
    Ignore,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAppAction {
    pub id: String,
}

impl OpenAppAction {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenURLAction {
    pub url: String,
}

impl OpenURLAction {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyAction {
    pub text: String,
}

impl CopyAction {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionAction {
    pub extension_id: String,
    pub action: String,
    pub args: Option<Vec<String>>,
}

impl ExtensionAction {
    pub fn new(extension_id: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            extension_id: extension_id.into(),
            action: action.into(),
            args: None,
        }
    }

    pub fn args(&mut self, args: impl Into<Vec<String>>) -> Self {
        self.args = Some(args.into());
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DialogAction {
    pub extension_id: String,
    pub action: String,
    pub title: String,
    pub action_text: String,
    pub fields: Vec<Field>,
    pub args: Option<Vec<String>>,
}

impl DialogAction {
    pub fn new(
        extension_id: impl Into<String>,
        action: impl Into<String>,
        title: impl Into<String>,
        action_text: impl Into<String>,
        fields: Vec<Field>,
    ) -> Self {
        Self {
            extension_id: extension_id.into(),
            action: action.into(),
            title: title.into(),
            action_text: action_text.into(),
            fields,
            args: None,
        }
    }

    pub fn args(&mut self, args: impl Into<Vec<String>>) -> Self {
        self.args = Some(args.into());
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub id: String,
    pub field_type: FieldType,
    pub input_field: Option<InputField>,
    pub text_area_field: Option<TextAreaField>,
    pub toggle_field: Option<ToggleField>,
    pub select_field: Option<SelectField>,
    pub file_picker_field: Option<FilePickerField>,
    pub args: Option<Vec<String>>,
}

impl Field {
    pub fn new_input(id: impl Into<String>, field: InputField) -> Self {
        Self {
            id: id.into(),
            field_type: FieldType::Input,
            input_field: Some(field),
            text_area_field: None,
            toggle_field: None,
            select_field: None,
            file_picker_field: None,
            args: None,
        }
    }

    pub fn new_text_area(id: impl Into<String>, field: TextAreaField) -> Self {
        Self {
            id: id.into(),
            field_type: FieldType::TextArea,
            input_field: None,
            text_area_field: Some(field),
            toggle_field: None,
            select_field: None,
            file_picker_field: None,
            args: None,
        }
    }

    pub fn new_toggle(id: impl Into<String>, field: ToggleField) -> Self {
        Self {
            id: id.into(),
            field_type: FieldType::Toggle,
            input_field: None,
            text_area_field: None,
            toggle_field: Some(field),
            select_field: None,
            file_picker_field: None,
            args: None,
        }
    }

    pub fn new_select(id: impl Into<String>,field: SelectField) -> Self {
        Self {
            id: id.into(),
            field_type: FieldType::Select,
            input_field: None,
            text_area_field: None,
            toggle_field: None,
            select_field: Some(field),
            file_picker_field: None,
            args: None,
        }
    }

    pub fn new_file_picker(id: impl Into<String>,field: FilePickerField) -> Self {
        Self {
            id: id.into(),
            field_type: FieldType::FilePicker,
            input_field: None,
            text_area_field: None,
            toggle_field: None,
            select_field: None,
            file_picker_field: Some(field),
            args: None,
        }
    }

    pub fn args(&mut self, args: Vec<String>) -> Self {
        self.args = Some(args);
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FieldType {
    Input,
    TextArea,
    Toggle,
    Select,
    FilePicker,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputField {
    pub default_value: String,
    pub title: String,
    pub description: String,
    pub placeholder: String,
}

impl InputField {
    pub fn new(
        default_value: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            default_value: default_value.into(),
            title: title.into(),
            description: description.into(),
            placeholder: String::from(""),
        }
    }

    pub fn placeholder(&mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextAreaField {
    pub default_value: String,
    pub title: String,
    pub description: String,
    pub placeholder: String,
}

impl TextAreaField {
    pub fn new(
        default_value: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            default_value: default_value.into(),
            title: title.into(),
            description: description.into(),
            placeholder: String::from(""),
        }
    }

    pub fn placeholder(&mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToggleField {
    pub default_value: bool,
    pub title: String,
    pub description: String,
}

impl ToggleField {
    pub fn new(
        default_value: bool,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            default_value: default_value.into(),
            title: title.into(),
            description: description.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectField {
    pub default_value: String,
    pub title: String,
    pub description: String,
    pub options: Vec<SelectOption>,
}

impl SelectField {
    pub fn new(
        default_value: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        options: Vec<SelectOption>,
    ) -> Self {
        Self {
            default_value: default_value.into(),
            title: title.into(),
            description: description.into(),
            options,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectOption {
    pub id: String,
    pub value: String,
}

impl SelectOption {
    pub fn new(id: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilePickerField {
    pub title: String,
    pub description: String,
    pub default_path: Option<String>,
    pub filters: Option<Vec<FileFilter>>,
    pub pick_directory: bool,
}

impl FilePickerField {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            default_path: None,
            filters: None,
            pick_directory: false,
        }
    }

    pub fn default_path(&mut self, default_path: impl Into<String>) -> Self {
        self.default_path = Some(default_path.into());
        self.to_owned()
    }

    pub fn filters(&mut self, filters: Vec<FileFilter>) -> Self {
        self.filters = Some(filters);
        self.to_owned()
    }

    pub fn pick_directory(&mut self, pick_directory: bool) -> Self {
        self.pick_directory = pick_directory;
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

impl FileFilter {
    pub fn new(name: impl Into<String>, extensions: Vec<String>) -> Self {
        Self {
            name: name.into(),
            extensions,
        }
    }
}
