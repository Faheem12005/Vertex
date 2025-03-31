use tauri::AppHandle;
use crate::core::assignments_lms::Assignment;
use crate::core::login::{get_credentials, ClientState, Error};
use crate::core::types::Service;
use crate::core::types;
#[tauri::command]
pub async fn login_lms(state: tauri::State<'_, ClientState>, payload: types::LoginPayload, app_handle: AppHandle) -> Result<String, Error> {
    state.login_moodle(payload, app_handle, &Service::LMS).await
}

#[tauri::command]
pub async fn fetch_assignments(state: tauri::State<'_, ClientState>, app_handle: AppHandle) -> Result<String, Error> {
    state.fetch_assignments(app_handle, &Service::LMS).await
}

#[tauri::command]
pub async fn logout_lms(state: tauri::State<'_, ClientState>, app_handle: AppHandle) -> Result<String, Error> {
    state.logout_moodle(app_handle, &Service::LMS).await
}

#[tauri::command]
pub async fn open_assignment_lms(state: tauri::State<'_, ClientState>, id: String, app_handle: AppHandle, ) -> Result<Assignment, Error> {
    state.open_assignment_lms(id, app_handle, &Service::LMS).await
}

#[tauri::command]
pub async fn auto_login_lms(state: tauri::State<'_, ClientState>, app_handle: AppHandle) -> Result<String, Error> {
    let payload = get_credentials(&app_handle, &Service::LMS)?;
    state.login_moodle(payload, app_handle, &Service::LMS).await
}