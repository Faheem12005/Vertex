use crate::core::login::ClientState;
use tauri::State;
#[tauri::command]
pub async fn login_lms(state: State<'_, ClientState>, payload: &str) -> Result<String, String> {
    state.login_lms(payload).await
}

#[tauri::command]
pub async fn fetch_assignments(state: State<'_, ClientState>) -> Result<String, String> {
    state.fetch_assignments().await
}

#[tauri::command]
pub async fn logout_lms(state: State<'_, ClientState>) -> Result<String, String> {
    state.logout_lms().await
}
#[tauri::command]
pub async fn open_assignment_lms(state: State<'_, ClientState>, id: String) -> Result<String, String> {
    state.open_assignment_lms(id).await
}