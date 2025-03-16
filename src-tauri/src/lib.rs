// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod core;
use core::{types,cmds};
use tauri::{Manager, async_runtime};
use tauri_plugin_http::reqwest;
use std::sync::Arc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let client = reqwest::Client::builder()
                .cookie_store(true)
                .danger_accept_invalid_certs(true)
                .use_native_tls()
                .build()
                .unwrap();
            app.manage(types::ClientState { client: Arc::new(client) });

            let app_handle = app.handle();
            async_runtime::spawn({
                let handle = app_handle.clone();
                async move {
                    let state: tauri::State<types::ClientState> = handle.state();
                    state.refresh_session().await;
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            cmds::login_lms,
            cmds::fetch_assignments,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
