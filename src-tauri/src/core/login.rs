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
use tauri::ipc::IpcResponse;
use tokio::io::AsyncWriteExt;
use crate::core::types::{Service, LoginPayload};
pub struct ClientState {
    pub client: Arc<reqwest::Client>,
}

pub fn store_credentials(login_info: LoginPayload, service: &Service, app: &AppHandle) -> Result<(), Error> {
    let username = login_info.username.as_str();
    let password = login_info.password.as_str();
    let entry = Entry::new("Vertex", username).map_err(|_| Error::CredentialsError("Failed to create keyring entry".to_string()))?;

    if entry.get_password().is_err() {
        entry.set_password(password).map_err(|_| Error::CredentialsError("Failed to set keyring entry password".to_string()))?;
    }
    let store = app.store("vertex.json").map_err(|_| Error::StoreError)?;
    store.set("username", username.to_string());
    let _value = store.get("username").expect("Failed to get value from store");

    Ok(())
}

pub fn delete_credentials(app: &AppHandle) -> Result<(), Error> {
    let store = app.store("vertex.json").map_err(|_| Error::StoreError)?;
    let username = store.get("username").ok_or_else(|| Error::StoreError)?.as_str().unwrap().to_string();
    let entry = Entry::new("Vertex", &username).unwrap();
    if let Err(e) = entry.delete_credential() {
        if !matches!(e, NoEntry) {
            return Err(Error::CredentialsError("credentials are not present to be deleted".to_string()));
        }
    }
    store.close_resource();
    Ok(())
}

pub fn get_credentials(app: &AppHandle, service: &Service) -> Result<LoginPayload, Error> {
    match service {
        Service::LMS => {
            let store = app.store("vertex.json").map_err(|_| Error::StoreError)?;
            let username = store.get("username").ok_or_else(|| Error::StoreError)?.as_str().unwrap().to_string();
            let entry = Entry::new("Vertex", &username).unwrap();
            let password = entry.get_password().map_err(|_| Error::CredentialsError("password not found in credential manager".to_string()))?;
            Ok(LoginPayload::new(username, password))
        }
        Service::Vitol => {
            Err(Error::InvalidFormat("not implemented yet".to_string()))
        }
    }

}
impl ClientState {
    pub async fn get_logintoken(&self, service: &Service) -> Result<String, Error> {
        let body = &self
            .client
            .get(format!("{}/login/index.php", service.base_url()))
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
    pub async fn login_moodle(&self, login_info: LoginPayload, app: AppHandle, service: &Service) -> Result<String, Error> {
        let logintoken = match self.get_logintoken(service).await {
            Ok(logintoken) => logintoken,
            Err(Error::InvalidRequestError(_)) => return Ok("user is already logged in".to_string()),
            Err(e) => return Err(e),
        };
        let login_details: HashMap<String, String> = login_info.add_token(logintoken);
        let response = self
            .client
            .post(format!("{}/login/index.php", service.base_url()))
            .form(&login_details)
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
            store_credentials(login_info, service, &app)?;
            Ok(userinfo)
        }
    }
    pub async fn fetch_sesskey(&self, service: &Service) -> Result<String, Error> {
        let body = self
            .client
            .get(format!("{}/my/", service.base_url()))
            .send()
            .await?
            .text()
            .await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse(r#"input[name="sesskey"]"#).unwrap();
        let element = document
            .select(&selector)
            .next()
            .ok_or_else(|| Error::InvalidRequestError("failed to find session key in document".to_string()))?;
        match element.value().attr("value") {
            Some(value) => Ok(value.to_string()),
            None => Err(Error::InvalidFormat(
                "no attribute value for sesskey".to_string(),
            )),
        }
    }
    pub async fn logout_moodle(&self, app: AppHandle, service: &Service) -> Result<String, Error> {
        let sesskey = match self.fetch_sesskey(service).await {
            Ok(s) => s,
            Err(Error::InvalidRequestError(_)) => return Ok("already logged out".to_string()),
            Err(e) => return Err(e),
        };

        let url = format!("{}/login/logout.php?sesskey={}",service.base_url(), sesskey);
        let response = self
            .client
            .get(url)
            .send()
            .await?;
        if response.text().await?.contains("You are not logged in.") {
            delete_credentials(&app)?;
            return Ok("successfully logged out".to_string());
        };
        Err(Error::AuthError(
            "logout failed due to unknown reasons".to_string(),
        ))
    }

    pub async fn relogin(&self, app: AppHandle, service: &Service) -> Result<(), Error> {
        let payload = get_credentials(&app, service)?;
        if let Err(e) = self.login_moodle(payload, app, service).await {
            if !matches!(e, Error::InvalidRequestError(_)) {
                return Err(e);
            }
        }
        Ok(())
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
        match client.get_logintoken(&Service::LMS).await {
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
