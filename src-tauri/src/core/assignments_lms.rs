use serde::{Deserialize, Serialize};
use crate::login::ClientState;

#[derive(Serialize, Deserialize)]
struct Assignment {
    title: String,
    description: String,
    id: String,
    opened_date: String,
    due_date: String,
    file_url: Vec<(String, String)>,
}

impl Assignment {
    pub fn new(id: String,title: String, description: String, opened_date: String, due_date: String, file_url: Vec<(String, String)>) -> Self {
        Self {
            id,
            title,
            description,
            opened_date,
            due_date,
            file_url,
        }
    }
}
impl ClientState {
    pub async fn fetch_assignments(&self) -> Result<String, String> {
        let sesskey = match self.fetch_sesskey().await {
            Some(s) => s,
            None => return Err("no key found in sesskey".to_string()),
        };

        let url = format!("https://lms.vit.ac.in/lib/ajax/service.php?sesskey={}&info=core_calendar_get_action_events_by_timesort", sesskey);
        let request_body = r#"
        [
            {
                "index": 0,
                "methodname": "core_calendar_get_action_events_by_timesort",
                "args": {
                "limitnum": 26,
                "timesortfrom": 1740249000,
                "limittononsuspendedevents": true
                }
            }
        ]"#;

        let request_json: serde_json::Value = serde_json::from_str(request_body)
            .map_err(|_| "Failed to parse JSON request".to_string())?;

        let response = self.client.post(url)
            .json(&request_json)
            .send().await
            .map_err(|_| "Failed to send request to fetch recent courses".to_string())?;

        let response_text = response.text().await
            .map_err(|_| "Failed to read response body".to_string())?;

        Ok(response_text)
    }

    pub async fn open_assignment_lms(&self, id: String) -> Result<String, String> {
        let url = format!("https://lms.vit.ac.in/mod/assign/view.php?id={}", id);
        let request = self.client.get(url);
        let response = request.send().await.map_err(|_| format!("Failed to send request to fetch course id {}", id))?;
        let response_text = response.text().await.map_err(|_| format!("Failed to read response body for course id {}", id))?;
        let document = scraper::Html::parse_document(&response_text);

        //get file urls for downloads
        let selector = scraper::Selector::parse(".fileuploadsubmission a").unwrap();

        let file_urls: Vec<(String, String)> = document
            .select(&selector)
            .filter_map(|e| {
                let href = e.value().attr("href")?.to_string();
                let text = e.text().collect::<String>();
                Some((href, text))
            })
            .collect();

        //get open and closing dates for assignments
        let date_selector = scraper::Selector::parse("[data-region='activity-dates'] div").unwrap();
        let dates: Vec<String> = document
            .select(&date_selector)
            .filter_map(|e| {
                let text_parts: Vec<_> = e.text().collect();
                text_parts.get(2).map(|&date| date.trim().to_string()) // Skip "Opened:" / "Due:"
            })
            .collect();

        let open_date = dates.get(0).cloned().unwrap_or_else(|| "Not Found".to_string());
        let due_date = dates.get(1).cloned().unwrap_or_else(|| "Not Found".to_string());

        //getting title and description
        let title_selector = scraper::Selector::parse("div[role='main'] h2").unwrap();
        let title = document.select(&title_selector).next().unwrap().text().collect::<Vec<_>>().join(" ");
        let description_selector = scraper::Selector::parse("#intro p").unwrap();
        let description = document.select(&description_selector).next().unwrap().text().collect::<Vec<_>>().join(" ");

        let assignment = Assignment::new(id, title, description, open_date, due_date, file_urls);
        Ok(serde_json::to_string(&assignment).unwrap_or_else(|_| {
            "Failed to serialize assignment".to_string()
        }))
    }
}

#[cfg(test)]
mod assignment_tests {
    use std::env;
    use super::*;
    use std::sync::Arc;
    use serde_json::json;
    use tauri_plugin_http::reqwest;
    #[tokio::test]
    async fn check_assignments() {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        let client = ClientState { client: Arc::new(client) };
        let result = client.fetch_assignments().await.unwrap_or_else(|e| e);
        assert_eq!(result, "no key found in sesskey");
    }

    #[tokio::test]
    async fn check_assignment_lms() {
        let username: String = env::var("USERNAME").unwrap();
        let password: String = env::var("PASSWORD").unwrap();

        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        let payload = json!({
            "username": username,
            "password": password,
        }).to_string();

        let client = ClientState { client: Arc::new(client) };
        client.login_lms(&payload).await.unwrap_or_else(|e| e);
       assert!(client.open_assignment_lms("61827".to_string()).await.is_ok());
    }
}
