// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod core;

use crate::core::errors::Error::AuthError;
use crate::core::login::ClientState;
use chrono::{Local, TimeZone};
use core::errors::Error;
use core::{cmds, login};
use keyring::Entry;
use regex::Regex;
use serde::Deserializer;
use serde_json::json;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use tauri_plugin_notification::NotificationExt;
use tokio::time::{sleep, Duration};
use tauri_plugin_store::StoreExt;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
#[cfg_attr(mobile, tauri::mobile_entry_point)]

async fn check_lms_assignment(app: &AppHandle) -> Result<(), Error> {
    let store = app.store("vertex.json").map_err(|_| Error::StoreError)?;
    println!("retrieved store...");
    let username = store.get("username").ok_or(Error::StoreError)?;
    let username_str = username.as_str().unwrap();
    let entry = Entry::new("Vertex", username_str).unwrap();
    match entry.get_password() {
        Ok(password) => {
            let payload = json!({
                "username": username,
                "password": password,
            })
            .to_string();
            let client = reqwest::Client::builder()
                .cookie_store(true)
                .timeout(Duration::from_secs(10))
                .build()?;
            let client = ClientState {
                client: Arc::new(client),
            };
            client.login_lms(&payload, app.clone()).await?;
            let assignments = client.fetch_assignments().await?;
            let assignments_json: serde_json::Value = serde_json::from_str(&assignments)?;
            let events_array = assignments_json[0]
                .get("data")
                .unwrap()
                .get("events")
                .unwrap()
                .as_array()
                .unwrap();

            events_array.iter().for_each(|event| {
                let url: String = event.get("formattedtime").unwrap().to_string();
                let re = Regex::new(r"time=(\d+)").unwrap();
                let timestamp_str = re.captures(&url).unwrap().get(1).unwrap().as_str();
                let timestamp = timestamp_str.parse::<i64>().unwrap();

                let event_time = Local.timestamp_opt(timestamp, 0).unwrap();
                let now = Local::now();
                let duration = event_time - now;
                let hours_diff = duration.num_hours();

                if hours_diff < 0 {
                    println!("This event was {} hours ago!", -hours_diff);
                } else {
                    println!("This event is in {} hours!", hours_diff);
                }
                let assignment_name = event.get("name").unwrap().as_str().unwrap();
                let line = format!("assignment {} in {} hours.", assignment_name, hours_diff);
                //fetching course name and title for notification
                let course_name = event.get("course")
                    .unwrap().as_object()
                    .unwrap().get("summary")
                    .unwrap().as_str().unwrap();
                app.notification()
                    .builder()
                    .title(course_name)
                    .body(line)
                    .icon("vertex30x30.png")
                    .show()
                    .unwrap();
            });
            Ok(())
        }
        Err(_) => Err(AuthError("failed to get entry for user".to_string())),
    }
}
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            let client = reqwest::Client::builder()
                .cookie_store(true)
                .use_native_tls()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap();
            app.manage(login::ClientState {
                client: Arc::new(client),
            });
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_secs(60)).await;
                    check_lms_assignment(&handle).await.unwrap_or_else(|error| {
                        println!("Error checking LMS: {}, retrying...", error);
                    });
                }
            });

            //setting up system tray
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let check_lms = MenuItem::with_id(app, "check_lms", "Check LMS", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit, &check_lms])?;

            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)

                //defining event handlers for tray
                .on_menu_event(move |handle, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        handle.cleanup_before_exit();
                        handle.exit(0);
                    }
                    "check_lms" => {
                        println!("check_lms menu item was clicked");
                        let app_handle = handle.clone();
                        tauri::async_runtime::spawn(async move {
                            check_lms_assignment(&app_handle).await.unwrap_or_else(|error| {
                                println!("Error checking LMS from tray event: {}", error);
                            });
                        });
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmds::login_lms,
            cmds::fetch_assignments,
            cmds::logout_lms,
            cmds::open_assignment_lms,
            cmds::auto_login_lms,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod background_test {
    use super::*;
    // #[tokio::test]
    // async fn test_fetch_assignments() {
    //     match check_lms().await {
    //         Ok(i64) => { assert!(true) }
    //         Err(error) => {
    //             assert!(false, "error occurred {:?}", error);
    //         }
    //     }
    // }
}
