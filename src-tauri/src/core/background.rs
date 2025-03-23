use std::time::Duration;
use crate::core::login::ClientState;
use tokio::time::sleep;
impl ClientState {
    pub async fn refresh_session(&self) {
        loop {

            sleep(Duration::from_secs(120)).await;
            let sesskey = match self.fetch_sesskey().await {
                Some(s) => s,
                None => {
                    eprintln!("failed to fetch sesskey");
                    continue;
                }
            };
            let url = format!("https://lms.vit.ac.in/lib/ajax/service.php?sesskey={}&info=core_session_touch", sesskey);

            let request_body = r#"
                [
                  {
                    "index": 0,
                    "methodname": "core_session_touch",
                    "args": {}
                  }
                ]"#;

            let request_json: serde_json::Value = serde_json::from_str(request_body).unwrap();
            let response = match self.client.post(url)
                .json(&request_json)
                .send().await {
                Ok(r) => r,
                Err(_) => {
                    eprintln!("error while fetching refresh url");
                    continue;
                }
            };
            let response_text = response.text().await.unwrap();

            let response_json: Vec<serde_json::Value> = match serde_json::from_str(&response_text) {
                Ok(j) => j,
                Err(_) => {
                    eprintln!("error while parsing response");
                    continue;
                }
            };
            if let Some(first) = response_json.first() {
                if first["error"].as_bool().unwrap_or(true) {
                    eprintln!("error while refreshing session");
                    continue;
                }
                println!("refreshed session");
            }
        }
    }
}

