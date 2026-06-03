use std::path::PathBuf;

use jarvis_core::custom_commands::{self, CustomCommandsConfig};

#[tauri::command]
pub fn get_custom_commands() -> CustomCommandsConfig {
    custom_commands::load()
}

#[tauri::command]
pub fn save_custom_commands(config: CustomCommandsConfig) -> Result<(), String> {
    custom_commands::save(&config)
}

#[tauri::command]
pub fn get_commands_count() -> usize {
    custom_commands::commands_count(&custom_commands::load())
}

#[tauri::command]
pub fn get_commands_list() -> CustomCommandsConfig {
    custom_commands::load()
}

#[tauri::command]
pub fn get_custom_sounds_dir() -> Result<String, String> {
    custom_commands::custom_sounds_dir().map(|p| p.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn import_custom_sound(path: String) -> Result<String, String> {
    custom_commands::import_custom_sound(PathBuf::from(path).as_path())
}
