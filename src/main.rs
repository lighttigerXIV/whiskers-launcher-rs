use whiskers_launcher_rs::paths::get_settings_path;

fn main() {
    let settings_path = get_settings_path();
    println!("path: {:?}", &settings_path);
}
