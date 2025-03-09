use serde::{ Deserialize, Serialize};
use scraper::{Html, Selector};
use crate::core::types;
use tauri::{AppHandle, Manager};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Login {
    logintoken: String,
    username: String,
    password: String,
}

#[tauri::command]
    pub fn login(app: AppHandle, payload: &str) -> Result<String, String> {
        let client = &app.state::<types::Client>().inner().client;
        let response: serde_json::Value = serde_json::from_str(payload).expect("expected a valid JSON object!");
        let logintoken = if let Some(value) = logintoken(&client) {
            value
        } else {
            return Err(String::from("failed to fetch token"));
        };
        let login_info = Login {
            username: response["username"].as_str().unwrap().to_string(),
            password: response["password"].as_str().unwrap().to_string(),
            logintoken,
        };
        match login_lms(&client, &login_info) {
            Err(_) => Err("failed to login".to_string()),
            Ok(response) => {
                let response_string = response.text().unwrap();
                if response_string.contains("You are not logged in") {
                    Err("Login credentials invalid.".to_string())
                } else{
                    let document = Html::parse_document(&response_string);
                    let selector = Selector::parse(".logininfo a").unwrap();
                    let tag = document.select(&selector).next().unwrap();
                    let userinfo = tag.text().collect::<Vec<_>>().join(" ");
                    Ok(userinfo)
                }
            }
        }
    }

fn login_lms(client: &reqwest::blocking::Client, login_info: &Login) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let response = client.post("https://lms.vit.ac.in/login/index.php").form(&login_info).send();
    response
}
fn logintoken(client: &reqwest::blocking::Client) -> Option<String> {
    let body = client.get("https://lms.vit.ac.in/login/index.php")
        .send()
        .unwrap()
        .text()
        .unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse(r#"input[name="logintoken"]"#).unwrap();
    let element = document.select(&selector).next().unwrap();
    match element.value().attr("value") {
        Some(value) => Some(value.to_string()),
        None => {
            eprintln!("failed to find login token");
            None
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::env;

    
    #[test]
    fn check_logintoken() {
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        let token = logintoken(&client);
        assert!(token.is_some(), "Login token should exist");
        assert!(!token.unwrap().is_empty(), "Login token should not be empty");
    }
    
    #[test]
    fn check_login() {

        let username: String = env::var("USERNAME").unwrap();
        let password: String = env::var("PASSWORD").unwrap();

        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        let token = logintoken(&client).unwrap();
        let login_info = Login {
            username: username.clone(),
            password: password,
            logintoken: token,
        };
        let response = login_lms(&client, &login_info).unwrap().text().unwrap();
        assert!(response.contains(&username));
    }
}
