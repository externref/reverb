use crate::user::AppConfig;
use serde_json::{from_str, to_string_pretty};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

fn get_bin_path(app: tauri::AppHandle, binary_name: &str) -> PathBuf {
    let mut bin_path = app.path().resource_dir().unwrap().join("bin");
    #[cfg(target_os = "windows")]
    let binary_name = format!("{}.exe", binary_name);

    bin_path.push(binary_name);
    bin_path
}

#[tauri::command]
pub async fn download_youtube_video(app: tauri::AppHandle, video_id: String) {
    let config_path = app.path().app_local_data_dir().unwrap();
    let audio_path = config_path.join("media/audio");
    let user_file = config_path.join("user.json");
    if let Ok(content) = fs::read_to_string(&user_file) {
        if let Ok(config_data) = from_str::<AppConfig>(&content) {
            if config_data
                .songs
                .iter()
                .any(|song| song.url.contains(&video_id))
            {
                println!("Song with video ID {} already exists", video_id);
                return;
            }
        }
    }
    let ytdlp_path = get_bin_path(app.clone(), "yt-dlp");
    if !audio_path.exists() {
        std::fs::create_dir_all(&audio_path).unwrap();
    }
    let output_file = audio_path.join(format!("{}.mp3", video_id));
    let resource_bin_path = app.path().resource_dir().unwrap().join("bin");
    let current_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{};{}", resource_bin_path.to_str().unwrap(), current_path);
    std::env::set_var("PATH", new_path);
    let status = std::process::Command::new(ytdlp_path.clone())
        .args([
            "-f",
            "bestaudio",
            "-o",
            output_file.to_str().unwrap(),
            &format!("https://www.youtube.com/watch?v={}", video_id),
        ])
        .status()
        .expect("Failed to execute yt-dlp");
    if !status.success() {
        eprintln!("yt-dlp failed with status: {}", status);
        return;
    }
    let json_output = std::process::Command::new(ytdlp_path)
        .args([
            "-j",
            &format!("https://www.youtube.com/watch?v={}", video_id),
        ])
        .output()
        .expect("Failed to execute yt-dlp for metadata");
    if !json_output.status.success() {
        eprintln!("Failed to get video metadata");
        return;
    }
    let json_str = String::from_utf8_lossy(&json_output.stdout);
    let video_info: serde_json::Value = serde_json::from_str(&json_str).unwrap_or_default();
    let title = video_info["title"]
        .as_str()
        .unwrap_or("Unknown Title")
        .to_string();
    let artist = video_info["uploader"]
        .as_str()
        .unwrap_or("Unknown Artist")
        .to_string();
    let url = format!("https://www.youtube.com/watch?v={}", video_id);
    if let Ok(content) = fs::read_to_string(&user_file) {
        if let Ok(mut config_data) = from_str::<AppConfig>(&content) {
            let new_song = crate::user::Song {
                title,
                artist,
                album: String::new(),
                url,
            };
            config_data.songs.push(new_song);
            if let Ok(updated_config) = to_string_pretty(&config_data) {
                fs::write(&user_file, updated_config).unwrap();
            }
        }
    }
}
