use scraper::{Html,Selector};
use tauri::{AppHandle, Manager};
use crate::core::types;

#[tauri::command]
pub fn fetch_assignments(app: AppHandle) -> Result<String, String> {
    let client = &app.state::<types::Client>().inner().client;
    let sesskey = fetch_sesskey(client).unwrap();
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
    .map_err(|_| "Failed to parse JSON".to_string())?;

    let response = client.post(url)
        .json(&request_json)
        .send()
        .map_err(|_| "Failed to send request to fetch recent courses".to_string())?;

    let response_text = response.text()
        .map_err(|_| "Failed to read response body".to_string())?;

    Ok(response_text)
}



pub fn fetch_sesskey(client: &reqwest::blocking::Client) -> Option<String> {
    let body = client.get("https://lms.vit.ac.in/my/")
        .send()
        .unwrap()
        .text()
        .unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse(r#"input[name="sesskey"]"#).unwrap();
    let element = document.select(&selector).next().unwrap();
    match element.value().attr("value") {
        Some(value) => Some(value.to_string()),
        None => {
            eprintln!("failed to fetch session key!");
            None
        }
    }
}
