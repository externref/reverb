use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref DISCORD_CLIENT: Mutex<Option<DiscordIpcClient>> = Mutex::new(None);
}

#[tauri::command]
pub fn init_discord() {
    let mut client = DiscordIpcClient::new("1345033627737788476").unwrap();
    if client.connect().is_ok() {
        *DISCORD_CLIENT.lock().unwrap() = Some(client);
        println!("Discord Connected!");
    }
}

#[tauri::command]
pub fn update_discord_presence(state: Option<String>, details: Option<String>) {
    if let Some(client) = &mut *DISCORD_CLIENT.lock().unwrap() {
        let state_clone = state.clone().unwrap_or_default();
        let details_clone = details.clone().unwrap_or_default();
        let _ = client.set_activity(
            activity::Activity::new()
                .state(&state_clone)
                .details(&details_clone),
        );
    }
    println!("Set presence to: {:?} {:?}", state, details);
}
