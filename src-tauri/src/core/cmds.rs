use crate::core::types::ClientState;
use tauri::State;
#[tauri::command]
pub async fn login_lms(state: State<'_, ClientState>, payload: &str) -> Result<String, String> {
    state.login_lms(payload).await
}

#[tauri::command]
pub async fn fetch_assignments(state: State<'_, ClientState>) -> Result<String, String> {
    state.fetch_assignments().await
}