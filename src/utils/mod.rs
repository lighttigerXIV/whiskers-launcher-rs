use std::{env, process::Command};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use notify_rust::Notification;

#[derive(Debug, Clone)]
pub struct Search {
    pub keyword: Option<String>,
    pub search_text: String,
}

pub fn get_search(text: impl Into<String>) -> Search {
    let text = text.into();

    let mut keyword = "".to_string();
    let mut search_text = "".to_string();
    let mut has_keyword = false;

    for char in text.chars() {
        if char == ' ' && !has_keyword {
            has_keyword = true;
        } else if !has_keyword {
            keyword += &char.to_string();
        } else {
            search_text += &char.to_string();
        }
    }

    if !has_keyword {
        search_text = keyword.to_owned();
    }

    search_text = search_text.trim().to_string();

    Search {
        keyword: match has_keyword {
            true => Some(keyword),
            false => None,
        },
        search_text,
    }
}

pub fn fuzzy_matches(original_text: impl Into<String>, search_text: impl Into<String>) -> bool {
    SkimMatcherV2::default()
        .fuzzy_match(&original_text.into(), &search_text.into())
        .is_some()
}

pub fn send_notification(title: impl Into<String>, message: impl Into<String>) {
    let title = title.into();
    let message = message.into();

    if on_linux() {
        Command::new("sh")
            .arg("-c")
            .arg(format!("notify-send {} {}", title, message))
            .spawn()
            .expect("Error sending notification");
    } else {

        #[cfg(target_os = "linux")]
        {
            Notification::new()
            .summary(&title)
            .body(&message)
            .show()
            .expect("Error showing notification")
            .icon("/usr/share/pixmaps/whiskers-launcher.png");
        }
        
        #[cfg(target_os = "windows")]
        {
            Notification::new()
                .summary(&title)
                .body(&message)
                .show()
                .expect("Error showing notification")
        }
    }
}

pub fn on_linux() -> bool {
    env::consts::OS == "linux"
}

pub fn on_windows() -> bool {
    env::consts::OS == "windows"
}

pub const FLAG_NO_WINDOW: u32 = 0x08000000;
pub const FLAG_DETACHED_PROCESS: u32 = 0x00000008;
