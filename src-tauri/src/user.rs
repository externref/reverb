use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Seek;
use std::io::{Read, Write};
use tauri::{self, Manager};
use tauri_plugin_dialog::DialogExt;
use zip::write::SimpleFileOptions;

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub cover: String,
    pub songs: Vec<Song>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub username: String,
    pub playlists: Vec<Playlist>,
    pub songs: Vec<Song>,
}

#[tauri::command]
pub fn get_user(app: tauri::AppHandle) -> Option<AppConfig> {
    let config = app.path().app_local_data_dir().unwrap().join("user.json");
    if !config.exists() {
        return None;
    } else {
        let usertr = std::fs::read_to_string(config).unwrap();
        let configdata: AppConfig = serde_json::from_str(&usertr).unwrap();
        return Some(configdata);
    }
}

#[tauri::command]
pub fn create_user(app: tauri::AppHandle, username: String) {
    let config_dir = app.path().app_local_data_dir().unwrap();
    let config_path = config_dir.join("user.json");

    let configdata = AppConfig {
        username,
        playlists: vec![],
        songs: vec![],
    };
    let usertr = serde_json::to_string(&configdata).unwrap();
    std::fs::write(config_path, usertr).unwrap();

    let media_path = config_dir.join("media");
    let subfolders = ["covers", "audio", "playlist_image"];
    for subfolder in &subfolders {
        let folder_path = media_path.join(subfolder);
        fs::create_dir_all(&folder_path).unwrap();
    }
}

#[tauri::command]
pub fn backup_user(app: tauri::AppHandle) {
    let user = app.path().app_local_data_dir().unwrap().join("user.json");
    let media = app.path().app_local_data_dir().unwrap().join("media");
    let user_data: AppConfig =
        serde_json::from_str(&std::fs::read_to_string(&user).unwrap()).unwrap();
    let file_name = format!("{}.zip", user_data.username);
    let file_path = app.path().app_local_data_dir().unwrap().join(&file_name);
    let file = File::create(&file_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    fn add_directory_to_zip<W: Write + Seek>(
        zip: &mut zip::ZipWriter<W>,
        path: &std::path::Path,
        options: zip::write::SimpleFileOptions,
        base_path: &std::path::Path,
    ) -> zip::result::ZipResult<()> {
        if path.is_dir() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                let name_in_zip = entry_path.strip_prefix(base_path).unwrap();
                if entry_path.is_file() {
                    let mut f = File::open(&entry_path)?;
                    let mut buffer = Vec::new();
                    f.read_to_end(&mut buffer)?;
                    zip.start_file(name_in_zip.to_str().unwrap(), options)?;
                    zip.write_all(&buffer)?;
                } else if entry_path.is_dir() {
                    zip.add_directory(name_in_zip.to_str().unwrap(), options)?;
                    add_directory_to_zip(zip, &entry_path, options, base_path)?;
                }
            }
        }
        Ok(())
    }
    if user.exists() {
        let mut f = File::open(&user).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        zip.start_file("user.json", options).unwrap();
        zip.write_all(&buffer).unwrap();
    }
    if media.exists() {
        add_directory_to_zip(
            &mut zip,
            &media,
            options,
            &app.path().app_local_data_dir().unwrap(),
        )
        .unwrap();
    }
    zip.finish().unwrap();
    let filepath = app
        .dialog()
        .file()
        .set_file_name(format!("{}.zip", user_data.username))
        .set_title("Save Backup")
        .blocking_save_file();
    if let Some(savepath) = filepath {
        std::fs::copy(&file_path, savepath.as_path().unwrap()).unwrap();
    }
}

#[tauri::command]
pub fn restore_user(app: tauri::AppHandle) {
    let filepath = app
        .dialog()
        .file()
        .set_title("Open Backup")
        .blocking_pick_file();
    if let Some(openpath) = filepath {
        let file = File::open(openpath.as_path().unwrap()).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        let config_dir = app.path().app_local_data_dir().unwrap();

        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            let outpath = config_dir.join(file.name());

            if file.is_file() {
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                let mut outfile = File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            } else if file.name().ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            }
        }
    }
}
