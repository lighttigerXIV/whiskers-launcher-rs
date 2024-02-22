use std::{fs, io, vec};

use crate::{
    indexing,
    paths::{get_indexing_dir, get_indexing_extensions_path, get_user_extensions_dir},
    settings::{self, update_settings, Extension, ExtensionSetting},
};

pub fn index_extensions() -> io::Result<()> {
    let dir = get_user_extensions_dir().ok_or(()).unwrap();

    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|_| ()).unwrap();
    }

    let extensions = indexing::get_user_extensions().ok_or(()).unwrap();
    let extensions_json = serde_json::to_string(&extensions).map_err(|_| ()).unwrap();
    let extensions_indexing_path = get_indexing_extensions_path().ok_or(()).unwrap();
    let indexing_dir = get_indexing_dir().ok_or(()).unwrap();
    
    if !indexing_dir.exists(){
        fs::create_dir_all(&indexing_dir).map_err(|_|()).unwrap();
    }

    fs::write(&extensions_indexing_path, &extensions_json)
        .map_err(|_| ())
        .unwrap();

    let mut settings = settings::get_settings().ok_or(()).unwrap();
    let mut new_extensions: Vec<Extension> = vec![];

    for extension in extensions {
        if extension.settings.is_some() {
            if settings.extensions.iter().any(|e| e.id == extension.id) {
                let index = settings
                    .extensions
                    .iter()
                    .position(|e| e.id == extension.id)
                    .unwrap();

                let extension_settings = settings.extensions[index].to_owned().settings;
                let mut new_extension_settings = settings.extensions[index].to_owned().settings;

                for setting in extension.settings.unwrap() {
                    if !extension_settings.iter().any(|s| s.id == setting.id) {
                        new_extension_settings.push(ExtensionSetting {
                            id: setting.id,
                            value: setting.default_value,
                        });
                    }
                }

                new_extensions.push(Extension {
                    id: extension.id,
                    keyword: settings.extensions[index].to_owned().keyword,
                    settings: new_extension_settings,
                });
            } else {
                let mut extension_settings: Vec<ExtensionSetting> = vec![];

                for setting in extension.settings.unwrap() {
                    extension_settings.push(ExtensionSetting {
                        id: setting.id,
                        value: setting.default_value,
                    });
                }

                new_extensions.push(Extension {
                    id: extension.id,
                    keyword: extension.keyword,
                    settings: extension_settings,
                });
            }
        }
    }

    settings.extensions = new_extensions;

    update_settings(settings).map_err(|_| ()).unwrap();

    Ok(())
}
