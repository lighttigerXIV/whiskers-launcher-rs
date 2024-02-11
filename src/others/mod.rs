use notify_rust::Notification;

/// Show a notification in the screen.
/// It requires a title and a notification text
///
/// ### Example
/// ```notify("Add bookmark", "Bookmark added successfully");```
pub fn send_notification(title: impl Into<String>, message: impl Into<String>) {

    let title = title.into();
    let message = message.into();

    Notification::new()
        .summary(&title)
        .body(&message)
        .show()
        .expect("Error showing notification");
}

pub const FLAG_NO_WINDOW: u32 = 0x08000000;
pub const FLAG_DETACHED_PROCESS: u32 = 0x00000008;