use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub enum Service {
    Vitol,
    LMS,
}
impl Service {
    pub fn base_url(&self) -> &'static str {
        match self {
            Service::Vitol => "https://vitolcc.vit.ac.in",
            Service::LMS => "https://lms.vit.ac.in",
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

impl LoginPayload {
    pub fn new(username: String, password: String) -> LoginPayload {
        LoginPayload { username, password }
    }

    pub fn add_token(&self, logintoken: String) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("logintoken".to_string(), logintoken);
        map.insert("username".to_string(), self.username.clone());
        map.insert("password".to_string(), self.password.clone());
        map
    }
}
