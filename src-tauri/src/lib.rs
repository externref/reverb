pub mod configs;
pub mod youtube;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            configs::get_configs,
            configs::create_config,
            youtube::say_hello
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
