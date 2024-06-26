use std::fs;

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

use crate::{
    paths::{get_autostart_dir, get_settings_path},
    settings::{get_default_settings, Settings},
};

#[cfg(target_os = "windows")]
use {
    crate::paths::{get_app_dir, get_app_resources_dir},
    mslnk::ShellLink,
    std::{env, path::Path}
};

pub fn get_settings() -> Settings {
    let settings_path = get_settings_path();

    if !settings_path.parent().unwrap().exists() {
        fs::create_dir_all(&settings_path.parent().unwrap())
            .expect("Error creating settings directory");
    }

    if !settings_path.exists() {
        fs::write(
            &settings_path,
            bincode::serialize(&get_default_settings()).expect("Error serializing settings"),
        )
        .expect("Error writing settings");
    }

    let settings_bytes = fs::read(get_settings_path()).expect("Error reading settings");
    let decoded_settings = bincode::deserialize::<Settings>(&settings_bytes);

    match decoded_settings {
        Ok(settings) => settings,
        Err(_) => get_default_settings(),
    }
}

pub fn write_settings(settings: Settings) {
    let current_settings = get_settings();

    if current_settings.auto_start != settings.auto_start {
        #[cfg(target_os = "linux")]
        {
            let desktop_content = r#"[Desktop Entry]
Type=Application
Name=Whiskers Launcher Companion
Comment=Whiskers Launcher companion app
Terminal=false
StartupNotify=false
Icon=/usr/share/pixmaps/whiskers-launcher.png
Exec=whiskers-launcher-companion"#;

            let path = get_autostart_dir();

            if !path.exists() {
                fs::create_dir_all(&path).expect("Error creating autostart directory");
            }

            let mut desktop_file_path = path.to_owned();
            desktop_file_path.push("whiskers-launcher.desktop");

            if settings.auto_start {
                fs::write(&desktop_file_path, &desktop_content)
                    .map_err(|_| ())
                    .unwrap();

                // Gives read and write permissions so that it can be executed on autostart
                fs::set_permissions(&desktop_file_path, fs::Permissions::from_mode(0o755))
                    .map_err(|_| ())
                    .unwrap();
            } else {
                if desktop_file_path.exists() {
                    fs::remove_file(&desktop_file_path).map_err(|_| ()).unwrap();
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            let mut shortcut_path = get_autostart_dir();
            shortcut_path.push("Whiskers-Launcher.lnk");

            if settings.auto_start {
                let mut target_path = get_app_dir();
                target_path.push("whiskers-launcher-companion.exe");

                let link = ShellLink::new(target_path).expect("Error initializing link");

                link.create_lnk(shortcut_path).expect("Error creating link");
            } else {
                if shortcut_path.exists() {
                    fs::remove_file(shortcut_path).expect("Error removing shortcut");
                }
            }
        }
    }

    let bytes = bincode::serialize(&settings).expect("Error encoding settings");
    fs::write(&get_settings_path(), bytes).expect("Error writing settings");
}
