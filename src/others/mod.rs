use notify_rust::Notification;

/// Show a notification in the screen.
/// It requires a title and a notification text
///
/// ### Example
/// ```notify("Add bookmark", "Bookmark added successfully");```
pub fn notify(title: &str, message: &str) {
    Notification::new()
        .summary(title)
        .body(message)
        .show()
        .expect("Error showing notification");
}