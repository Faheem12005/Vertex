// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod core;
use core::{login, cmds};
use tauri::{Manager, async_runtime};
use tauri_plugin_http::reqwest;
use std::sync::Arc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let client = reqwest::Client::builder()
                .cookie_store(true)
                .use_native_tls()
                .tcp_keepalive(None)
                .pool_idle_timeout(None)
                .build()
                .unwrap();
            app.manage(login::ClientState { client: Arc::new(client) });

            let app_handle = app.handle();
            async_runtime::spawn({
                let handle = app_handle.clone();
                async move {
                    let state: tauri::State<login::ClientState> = handle.state();
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
            cmds::logout_lms,
            cmds::open_assignment_lms,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
