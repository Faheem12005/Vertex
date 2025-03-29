pub(crate) use crate::core::errors::Error;
use keyring::Entry;
use scraper::{Html, Selector};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{App, AppHandle};
use tauri_plugin_http::reqwest;
use serde_json::json;
use tauri_plugin_store::StoreExt;
use keyring::Error::NoEntry;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct ClientState {
    pub client: Arc<reqwest::Client>,
}

impl ClientState {
    pub async fn get_logintoken(&self) -> Result<String, Error> {
        let body = &self
            .client
            .get("https://lms.vit.ac.in/login/index.php")
            .send()
            .await?
            .text()
            .await?;
        if body.contains("You are logged in as") {
            return Err(Error::InvalidRequestError("User is already logged in".to_string()));
        }
        let document = Html::parse_document(&body);
        let selector = Selector::parse(r#"input[name="logintoken"]"#).unwrap();

        let element = document.select(&selector).next().ok_or_else(|| {
            Error::InvalidFormat("no hidden input present for logintoken".to_string())
        })?;

        match element.value().attr("value") {
            Some(value) => Ok(value.to_string()),
            None => Err(Error::InvalidFormat(
                "no attribute value for logintoken".to_string(),
            )),
        }
    }
    pub async fn login_lms(&self, payload: &str, app: AppHandle) -> Result<String, Error> {
        let response: serde_json::Value = serde_json::from_str(payload)?;
        let logintoken = match self.get_logintoken().await {
            Ok(logintoken) => logintoken,
            Err(Error::InvalidRequestError(_)) => return Ok("user is already logged in".to_string()),
            Err(e) => return Err(e),
        };
        let username = response["username"].as_str().unwrap().to_string().clone();
        let password = response["password"].as_str().unwrap().to_string().clone();
        let mut login_info = HashMap::new();
        login_info.insert("username".to_string(), username.clone());
        login_info.insert("password".to_string(), password.clone());
        login_info.insert("logintoken".to_string(), logintoken);
        let response = self
            .client
            .post("https://lms.vit.ac.in/login/index.php")
            .form(&login_info)
            .send()
            .await?;
        let response_string = response.text().await?;

        if !response_string.contains("You are logged in as") {
            Err(Error::AuthError("login credentials invalid".to_string()))
        } else {
            let document = Html::parse_document(&response_string);
            let selector = Selector::parse(".logininfo a").unwrap();
            let tag = document.select(&selector).next().unwrap();
            let userinfo = tag.text().collect::<Vec<_>>().join(" ");
            let entry = Entry::new("Vertex", &username).unwrap();
            match entry.get_password() {
                Ok(_) => {}
                Err(_) => {
                    entry.set_password(&password).unwrap();
                }
            }
            let store = app.store("vertex.json").map_err(|_| Error::StoreError)?;
            store.set("username", username.to_string());
            let value = store.get("username").expect("Failed to get value from store");
            println!("{}", value);
            Ok(userinfo)
        }
    }
    pub async fn fetch_sesskey(&self) -> Result<String, Error> {
        let body = self
            .client
            .get("https://lms.vit.ac.in/my/")
            .send()
            .await?
            .text()
            .await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse(r#"input[name="sesskey"]"#).unwrap();
        let element = document
            .select(&selector)
            .next()
            .ok_or_else(|| Error::InvalidFormat("failed to find session key in document".to_string()))?;
        match element.value().attr("value") {
            Some(value) => Ok(value.to_string()),
            None => Err(Error::InvalidFormat(
                "no attribute value for sesskey".to_string(),
            )),
        }
    }
    pub async fn logout_lms(&self, app: AppHandle) -> Result<String, Error> {
        let sesskey = match self.fetch_sesskey().await {
            Ok(s) => s,
            Err(Error::InvalidFormat(_)) => return Ok("already logged out".to_string()),
            Err(e) => return Err(e),
        };

        let url = format!("https://lms.vit.ac.in/login/logout.php?sesskey={}", sesskey);
        let response = self
            .client
            .get(url)
            .send()
            .await?;
        if response.text().await?.contains("You are not logged in.") {
            let store = app.store("vertex.json").map_err(|_| Error::StoreError)?;
            let username = store.get("username").ok_or_else(|| Error::StoreError)?.as_str().unwrap().to_string();
            let entry = Entry::new("Vertex", &username).unwrap();
            if let Err(e) = entry.delete_credential() {
                if !matches!(e, NoEntry) {
                    return Err(Error::CredentialsError("credentials are not present to be deleted".to_string()));
                }
            }
            store.close_resource();
            println!("reached the return");
            return Ok("successfully logged out".to_string());
        };
        Err(Error::AuthError(
            "logout failed due to unknown reasons".to_string(),
        ))
    }

    pub async fn lms_return_logininfo(&self, app: AppHandle) -> Result<String, Error> {
        let store = app.store("vertex.json").map_err(|_| Error::StoreError)?;
        let username = store.get("username").ok_or_else(|| Error::StoreError)?.as_str().unwrap().to_string();
        let entry = Entry::new("Vertex", &username).unwrap();
        let password = entry.get_password().map_err(|_| Error::CredentialsError("password not found in credential manager".to_string()))?;
        let payload = json!({
            "username": username,
            "password": password
        });
        Ok(payload.to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use keyring::{mock, set_default_credential_builder};
    use serde_json::json;
    use std::env;
    use std::sync::Arc;
    #[tokio::test]
    async fn check_logintoken() {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        let client = ClientState {
            client: Arc::new(client),
        };
        match client.get_logintoken().await {
            Ok(token) => {
                assert!(!token.is_empty(), "Token should not be empty")
            }
            Err(Error::AuthError(s)) => {
                assert!(false, "Auth Error: {}", s)
            }
            Err(Error::NetworkError(e)) => {
                assert!(false, "Failed to fetch login token: {}", e)
            }
            Err(e) => {
                panic!("Failed to fetch login token: {}", e)
            }
        }
    }
}
