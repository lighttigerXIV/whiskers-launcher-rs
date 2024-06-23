#[cfg(target_os = "linux")]
use {
    crate::paths::get_autostart_dir,
    std::{fs, os::unix::fs::PermissionsExt},
};

use crate::{paths::get_settings_path, settings::{get_default_settings, Settings}};

#[cfg(target_os = "windows")]
use crate::get_app_resources_dir;


pub fn get_settings() -> Settings {
    let settings_path = get_settings_path();

    if !settings_path.parent().unwrap().exists() {
        fs::create_dir_all(&settings_path.parent().unwrap())
            .expect("Error creating settings directory");
    }

    if !settings_path.exists() {
        write_settings(get_default_settings());
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
            let mut path = get_app_resources_dir();
            path.push("scripts");
            path.push(if settings.auto_start {
                "enable-autostart.ps1"
            } else {
                "disable-autostart.ps1"
            });

            let script_content = fs::read_to_string(&path).expect("Error reading script");

            powershell_script::run(&script_content).expect("Error running script");
        }
    }

    let bytes = bincode::serialize(&settings).expect("Error encoding settings");
    fs::write(&get_settings_path(), bytes).expect("Error writing settings");
}
