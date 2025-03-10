use tauri;

#[tauri::command]
pub fn say_hello(name: String) -> String {
    format!("Hello, {}!", name)
}
