use tauri_plugin_http::reqwest;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid login: {0}")]
    AuthError(String),
    #[error("invalid format received: {0}")]
    InvalidJSONFormat(#[from] serde_json::Error),
    #[error("unavailable to fetch from url: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("encountered a store error")]
    StoreError,
    #[error("invalid format received: {0}")]
    InvalidFormat(String),
    #[error("error fetching credentials: {0}")]
    CredentialsError(String),
    #[error("invalid Request Error: {0}")]
    InvalidRequestError(String)
}
