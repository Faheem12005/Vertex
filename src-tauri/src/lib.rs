// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod core;
use core::{login,types};
use tauri::Manager;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let client = reqwest::blocking::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap();
            app.manage(types::Client { client });
            Ok(())
    })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![login::login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
