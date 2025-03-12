use tauri_plugin_http::reqwest;

pub struct Client {
    pub client: reqwest::blocking::Client,
}