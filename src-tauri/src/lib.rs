pub mod media;
pub mod misc;
pub mod user;
pub mod youtube;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            user::get_user,
            user::create_user,
            youtube::say_hello,
            misc::init_discord,
            misc::update_discord_presence
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
