use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;
use std::sync::Arc;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Login {
    logintoken: String,
    username: String,
    password: String,
}
impl Login {
    fn new(username: String, password: String, logintoken: String) -> Self {
        Self {
            username,
            password,
            logintoken,
        }
    }
}

pub struct ClientState {
    pub client: Arc<reqwest::Client>,
}

impl ClientState {
    async fn get_logintoken(&self) -> Option<String> {
        let body = &self.client.get("https://lms.vit.ac.in/login/index.php")
            .send().await.unwrap_or_else(|_| panic!("failed to get lms token"))
            .text().await
            .unwrap();
        let document = Html::parse_document(&body);

        let selector = Selector::parse(r#"input[name="logintoken"]"#).unwrap();

        let element = document.select(&selector).next()
            .unwrap_or_else(|| panic!("failed to find hidden input for logintoken!"));

        match element.value().attr("value") {
            Some(value) => Some(value.to_string()),
            None => {
                eprintln!("failed to find login token");
                None
            }
        }
    }
    pub async fn login_lms(&self, payload: &str) -> Result<String, String> {
        let response: serde_json::Value = serde_json::from_str(payload).expect("expected a valid JSON object!");
        let logintoken = if let Some(value) = self.get_logintoken().await {
            value
        } else {
            return Err(String::from("failed to fetch token"));
        };

        let login_info = Login::new(
            response["username"].as_str().unwrap().to_string(),
            response["password"].as_str().unwrap().to_string(),
            logintoken);

        let login = self.client.post("https://lms.vit.ac.in/login/index.php").form(&login_info).send().await;
        match login {
            Err(_) => Err("failed to login".to_string()),
            Ok(response) => {
                let response_string = response.text().await.unwrap();
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
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
    use serde_json::json;
    use std::sync::Arc;
    #[tokio::test]
    async fn check_logintoken() {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        let client = ClientState { client: Arc::new(client) };
        let token = client.get_logintoken().await;
        assert!(token.is_some(), "Login token should exist");
        assert!(!token.unwrap().is_empty(), "Login token should not be empty");
    }

    #[tokio::test]
    async fn check_login() {

        let username: String = env::var("USERNAME").unwrap();
        let password: String = env::var("PASSWORD").unwrap();

        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        let client = ClientState { client: Arc::new(client) };
        let logintoken = client.get_logintoken().await.unwrap();

        let payload = json!({
            "username": username,
            "password": password,
        }).to_string();

        let response = client.login_lms(&payload).await.unwrap();
        assert!(response.contains(&username));

        let sesskey = client.fetch_sesskey().await;
        assert!(sesskey.is_some());
    }
}
