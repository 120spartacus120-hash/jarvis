use crate::autostart;
use crate::AppState;

#[tauri::command]
pub fn is_windows_autostart() -> bool {
    autostart::is_enabled()
}

#[tauri::command]
pub fn set_windows_autostart(state: tauri::State<'_, AppState>, enabled: bool) -> Result<(), String> {
    autostart::set_enabled(enabled)?;
    state
        .settings
        .write("windows_autostart", if enabled { "true" } else { "false" })?;
    Ok(())
}
