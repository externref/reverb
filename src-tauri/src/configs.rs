use serde::{Deserialize, Serialize};
use tauri::{self, Manager};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    username: String,
    saved_at: String,
}

#[tauri::command]
pub fn get_configs(app: tauri::AppHandle) -> Option<AppConfig> {
    let config = app.path().app_local_data_dir().unwrap().join("user.json");
    if !config.exists() {
        return None;
    } else {
        let configstr = std::fs::read_to_string(config).unwrap();
        let configdata: AppConfig = serde_json::from_str(&configstr).unwrap();
        return Some(configdata);
    }
}

#[tauri::command]
pub fn create_config(app: tauri::AppHandle, username: String) -> bool {
    let config_dir = app.path().app_local_data_dir();
    if !config_dir.is_ok() {
        return false;
    } else {
        let config_path = config_dir.unwrap().join("user.json");
        let configdata = AppConfig {
            username,
            saved_at: config_path.to_str().unwrap().to_string(),
        };
        let configstr = serde_json::to_string(&configdata).unwrap();
        std::fs::write(config_path, configstr).unwrap();
        return true;
    }
}
