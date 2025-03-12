// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod core;
use core::{login,types,cmds};
use tauri::Manager;
use tauri_plugin_http::reqwest;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let client = reqwest::blocking::Client::builder()
                .cookie_store(true)
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap();
            app.manage(types::Client { client });
            Ok(())
    })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            login::login,
            cmds::fetch_assignments,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
