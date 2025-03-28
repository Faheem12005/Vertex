use crate::core::errors::Error;
use crate::login::ClientState;
use serde::{Deserialize, Serialize};
use tokio::task;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct Assignment {
    id: String,
    opened_date: String,
    due_date: String,
    file_url: Vec<(String, String)>,
}

impl Assignment {
    pub fn new(
        id: String,
        opened_date: String,
        due_date: String,
        file_url: Vec<(String, String)>,
    ) -> Self {
        Self {
            id,
            opened_date,
            due_date,
            file_url,
        }
    }
}
impl ClientState {
    ///function fetches assignments and returns it as a json serializable string
    pub async fn fetch_assignments(&self) -> Result<String, Error> {
        let sesskey = self.fetch_sesskey().await?;
        let url = format!("https://lms.vit.ac.in/lib/ajax/service.php?sesskey={}&info=core_calendar_get_action_events_by_timesort", sesskey);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - 1209600;
        let request_body = format!(r#"
        [
            {{
                "index": 0,
                "methodname": "core_calendar_get_action_events_by_timesort",
                "args": {{
                "limitnum": 26,
                "timesortfrom": {},
                "limittononsuspendedevents": true
                }}
            }}
        ]"#, timestamp);

        let request_json: serde_json::Value = serde_json::from_str(&request_body)?;
        let response = self.client.post(url).json(&request_json).send().await?;

        let response_text = response.text().await?;

        Ok(response_text)
    }

    pub async fn open_assignment_lms(&self, id: String) -> Result<Assignment, Error> {
        let url = format!("https://lms.vit.ac.in/mod/assign/view.php?id={}", id);
        let request = self.client.get(url);
        let response = request.send().await?;
        let response_text = response.text().await?;
        let document = scraper::Html::parse_document(&response_text);

        //get file urls for downloads
        let selector = scraper::Selector::parse(".fileuploadsubmission a").unwrap();

        let file_urls: Vec<(String, String)> = document
            .select(&selector)
            .filter_map(|e| {
                let href = e.value().attr("href")?.to_string();
                let text = e.text().collect::<String>();
                Some((href, text)) // Returns a tuple (String, String)
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

        let open_date = dates
            .get(0)
            .cloned()
            .unwrap_or_else(|| "Not Found".to_string());
        let due_date = dates
            .get(1)
            .cloned()
            .unwrap_or_else(|| "Not Found".to_string());
        let assignment = Assignment::new(id, open_date, due_date, file_urls);
        Ok(assignment)
    }
}

// #[cfg(test)]
// mod assignment_tests {
//     use super::*;
//     use serde_json::json;
//     use std::env;
//     use std::sync::Arc;
//     use tauri_plugin_http::reqwest;
//     #[tokio::test]
//     async fn check_assignments() {
//         let client = reqwest::Client::builder()
//             .cookie_store(true)
//             .build()
//             .unwrap();
//
//         let client = ClientState {
//             client: Arc::new(client),
//         };
//         let result = client.fetch_assignments().await.unwrap_or_else(|e| e);
//         assert_eq!(result, "no key found in sesskey");
//     }
//
//     #[tokio::test]
//     async fn check_assignment_lms() {
//         let username: String = env::var("USERNAME").unwrap();
//         let password: String = env::var("PASSWORD").unwrap();
//
//         let client = reqwest::Client::builder()
//             .cookie_store(true)
//             .build()
//             .unwrap();
//
//         let payload = json!({
//             "username": username,
//             "password": password,
//         })
//         .to_string();
//
//         let client = ClientState {
//             client: Arc::new(client),
//         };
//         client.login_lms(&payload).await.unwrap_or_else(|e| e);
//         assert!(client
//             .open_assignment_lms("61827".to_string())
//             .await
//             .is_ok());
//     }
// }
