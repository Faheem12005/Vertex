use tauri_plugin_http::reqwest;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid login: {0}")]
    AuthError(String),
    #[error(transparent)]
    InvalidJSONFormat(#[from] serde_json::Error),
    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),
    #[error("encountered a store error")]
    StoreError,
    #[error("invalid format received: {0}")]
    InvalidFormat(String),
    #[error("error fetching credentials: {0}")]
    CredentialsError(String),
    #[error("invalid request error: {0}")]
    InvalidRequestError(String),
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorKind {
    AuthError(String),
    InvalidJSONFormat(String),
    NetworkError(String),
    StoreError(String),
    InvalidFormat(String),
    CredentialsError(String),
    InvalidRequestError(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::AuthError(_) => ErrorKind::AuthError(error_message),
            Self::InvalidJSONFormat(_) => ErrorKind::InvalidJSONFormat(error_message),
            Self::NetworkError(_) => ErrorKind::NetworkError(error_message),
            Self::StoreError => ErrorKind::StoreError(error_message),
            Self::InvalidFormat(_) => ErrorKind::InvalidFormat(error_message),
            Self::CredentialsError(_) => ErrorKind::CredentialsError(error_message),
            Self::InvalidRequestError(_) => ErrorKind::InvalidRequestError(error_message),
        };
        error_kind.serialize(serializer)
    }
}
