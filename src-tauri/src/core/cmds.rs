use tauri::AppHandle;
use crate::core::assignments_lms::Assignment;
use crate::core::login::{ClientState, Error};
use tokio::time::{sleep, Duration};
#[tauri::command]
pub async fn login_lms(state: tauri::State<'_, ClientState>, payload: &str, app_handle: AppHandle) -> Result<String, Error> {
    state.login_lms(payload, app_handle).await
}

#[tauri::command]
pub async fn fetch_assignments(state: tauri::State<'_, ClientState>) -> Result<String, Error> {
    state.fetch_assignments().await
}

#[tauri::command]
pub async fn logout_lms(state: tauri::State<'_, ClientState>, app_handle: AppHandle) -> Result<String, Error> {
    state.logout_lms(app_handle).await
}

#[tauri::command]
pub async fn open_assignment_lms(
    state: tauri::State<'_, ClientState>,
    id: String,
) -> Result<Assignment, Error> {
    state.open_assignment_lms(id).await
}

#[tauri::command]
pub async fn auto_login_lms(state: tauri::State<'_, ClientState>, app_handle: AppHandle) -> Result<String, Error> {
    let payload = state.lms_return_logininfo(app_handle.clone()).await?;
    state.login_lms(&payload, app_handle).await
}