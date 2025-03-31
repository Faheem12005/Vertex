use crate::core::errors::Error;
use crate::login::ClientState;
use serde::{Deserialize, Serialize};
use tokio::task;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::AppHandle;
use crate::core::types::Service;

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
    pub async fn fetch_assignments(&self, app: AppHandle, service: &Service) -> Result<String, Error> {
        self.relogin(app, service).await?;
        let sesskey = self.fetch_sesskey(service).await?;
        let url = format!("{}/lib/ajax/service.php?sesskey={}&info=core_calendar_get_action_events_by_timesort",service.base_url(), sesskey);
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

    pub async fn open_assignment_lms(&self, id: String, app: AppHandle, service: &Service) -> Result<Assignment, Error> {
        self.relogin(app, service).await?;
        let url = format!("{}/mod/assign/view.php?id={}",service.base_url(), id);
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
