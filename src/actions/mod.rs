use serde::{Deserialize, Serialize};
use crate::extensions::get_parameters;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ResultAction {
    OpenApp(OpenApp),
    OpenInBrowser(OpenInBrowser),
    CopyToClipboard(CopyToClipboard),
    ExtensionAction(ExtensionAction),
    DialogAction(DialogAction),
    DoNothingAction(DoNothingAction),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenApp {
    pub desktop_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenInBrowser {
    pub url: String,
}

impl OpenInBrowser {
    pub fn new(url: &str) -> Self {
        return OpenInBrowser {
            url: url.to_owned()
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyToClipboard {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionAction {
    pub extension_id: String,
    pub action: String,
    pub args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DialogAction {
    pub extension_id: String,
    pub title: String,
    pub action: String,
    pub button_text: String,
    pub fields: Vec<DialogField>,
    pub args: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DialogResult {
    pub extension_id: String,
    pub action: String,
    pub results: Vec<DialogFieldResult>,
}

impl DialogResult {
    pub fn new(extension_id: &str, action: &str, results: Vec<DialogFieldResult>) -> Self {
        return DialogResult {
            extension_id: extension_id.to_owned(),
            action: action.to_owned(),
            results,
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DialogFieldResult {
    pub field_id: String,
    pub value: String,
}

impl DialogFieldResult {
    pub fn new(field_id: &str, value: &str) -> Self {
        return DialogFieldResult {
            field_id: field_id.to_owned(),
            value: value.to_owned(),
        };
    }
}

impl DialogAction {
    pub fn new(extension_id: &str, title: &str, action: &str) -> Self {
        return DialogAction {
            extension_id: extension_id.to_owned(),
            title: title.to_owned(),
            action: action.to_owned(),
            button_text: "OK".to_owned(),
            fields: vec![],
            args: None
        };
    }

    pub fn button_text(&mut self, button_text: &str) -> Self {
        self.button_text = button_text.to_owned();
        self.to_owned()
    }

    pub fn fields(&mut self, fields: Vec<DialogField>) -> Self {
        self.fields = fields.to_owned();
        self.to_owned()
    }

    pub fn args(&mut self, args: Option<Vec<String>>) -> Self{
        self.args = args.to_owned();
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum DialogField {
    Input(InputField),
    Toggle(ToggleField),
    Select(SelectField),
    CheckGroup(CheckGroup),
}

impl DialogField {
    pub fn new_input(field: InputField) -> Self {
        return DialogField::Input(field);
    }

    pub fn new_toggle(field: ToggleField) -> Self {
        return DialogField::Toggle(field);
    }

    pub fn new_select(field: SelectField) -> Self {
        return DialogField::Select(field);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputField {
    pub id: String,
    pub value: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub placeholder: Option<String>,
}


impl InputField {
    pub fn new() -> Self {
        return InputField {
            id: "".to_owned(),
            value: "".to_owned(),
            title: Some("".to_owned()),
            description: Some("".to_owned()),
            placeholder: Some("".to_owned()),
        };
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.id = id.to_owned();
        self
    }

    pub fn value(&mut self, value: &str) -> &mut Self {
        self.value = value.to_owned();
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_owned());
        self
    }


    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn placeholder(&mut self, placeholder: &str) -> &mut Self {
        self.placeholder = Some(placeholder.to_owned());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToggleField {
    pub id: String,
    pub value: bool,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl ToggleField {
    pub fn new(id: &str, value: bool, title: &str, description: &str) -> Self {
        return ToggleField {
            id: id.to_owned(),
            value: value.to_owned(),
            title: match title.is_empty() {
                false => Some(title.to_owned()),
                true => None
            },
            description: match description.is_empty() {
                false => Some(description.to_owned()),
                true => None
            },
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectField {
    pub id: String,
    pub default_value: String,
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<SelectOption>,
}

impl SelectField {
    pub fn new(id: &str, default_value: &str, title: &str, description: &str, options: Vec<SelectOption>) -> Self {
        return SelectField {
            id: id.to_owned(),
            default_value: default_value.to_owned(),
            title: title.to_owned(),
            description: match description.is_empty() {
                false => Some(description.to_owned()),
                true => None
            },
            options,
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckGroup {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<CheckOption>,
}

impl CheckGroup {
    pub fn new(id: &str, title: &str) -> Self {
        return CheckGroup {
            id: id.to_owned(),
            title: title.to_owned(),
            description: None,
            options: vec![],
        };
    }

    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn options(&mut self, options: Vec<CheckOption>) -> &mut Self {
        self.options = options;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckOption {
    id: String,
    title: String,
    description: Option<String>,
    checked: bool,
}

impl CheckOption {
    pub fn new(id: &str, title: &str) -> Self {
        return Self {
            id: id.to_owned(),
            title: title.to_owned(),
            description: None,
            checked: false,
        };
    }

    pub fn checked(&mut self, checked: bool) -> &mut Self {
        self.checked = checked;
        self
    }

    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_owned());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckResult {
    pub id: String,
    pub checked: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectOption {
    pub id: String,
    pub value: String,
}

impl SelectOption {
    pub fn new(id: &str, value: &str) -> Self {
        return SelectOption {
            id: id.to_owned(),
            value: value.to_owned(),
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoNothingAction {}

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

    pub fn new_with_args(extension_id: &str, action: &str, args: Vec<String>) -> Self {
        return ExtensionAction {
            extension_id: extension_id.to_owned(),
            action: action.to_owned(),
            args: Some(args),
        };
    }
}


pub fn get_dialog_results() -> Option<Vec<DialogFieldResult>> {
    let parameters = get_parameters()?;
    let results = serde_yaml::from_str(&parameters.custom_args?[0]).ok()?;

    return Some(results);
}

pub fn get_dialog_result(field_id: &str) -> Option<DialogFieldResult> {
    let results = get_dialog_results()?;
    let result = results.iter().find(|f| f.field_id == field_id.to_owned()).unwrap().to_owned();

    return Some(result);
}

pub fn get_check_group_results(field_id: &str) -> Option<Vec<CheckResult>>{

    let result = get_dialog_result(field_id).unwrap();
    let results: Vec<CheckResult> = serde_json::from_str(&result.value).unwrap();

    return Some(results)
}

pub fn get_check_group_result(field_id: &str, option_id: &str)->Option<CheckResult>{

    let results = get_check_group_results(field_id).unwrap();
    let result = results.iter().find(|r|r.id == option_id).unwrap().to_owned();

    return Some(result);
}