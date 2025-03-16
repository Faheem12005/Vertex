use crate::types::ClientState;
use scraper::{Html, Selector};

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
    pub async fn fetch_sesskey(&self) -> Option<String> {
        let body = self.client.get("https://lms.vit.ac.in/my/")
            .send().await
            .unwrap()
            .text().await
            .unwrap();
        let document = Html::parse_document(&body);
        let selector = Selector::parse(r#"input[name="sesskey"]"#).unwrap();
        let element = match document.select(&selector).next() {
            Some(e) => e,
            None => return None,
        };
        match element.value().attr("value") {
            Some(value) => Some(value.to_string()),
            None => {
                eprintln!("failed to fetch session key!");
                None
            }
        }
    }
}