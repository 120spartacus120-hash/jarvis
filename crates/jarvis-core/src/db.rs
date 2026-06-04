pub mod structs;
pub mod manager;

use crate::{config, APP_CONFIG_DIR};

use log::info;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub use manager::SettingsManager;

fn get_db_file_path() -> PathBuf {
    PathBuf::from(format!(
        "{}/{}",
        APP_CONFIG_DIR.get().unwrap().display(),
        config::DB_FILE_NAME
    ))
}

pub fn settings_file_path() -> PathBuf {
    get_db_file_path()
}

fn settings_file_mtime(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path).ok().and_then(|m| m.modified().ok())
}

/// While jarvis-app runs, reload settings when GUI saves settings.json (no restart needed).
#[cfg(feature = "jarvis_app")]
pub fn spawn_settings_file_watcher() {
    let path = settings_file_path();

    std::thread::Builder::new()
        .name("jarvis-settings-watch".into())
        .spawn(move || {
            let mut last_mtime = settings_file_mtime(&path);

            loop {
                std::thread::sleep(Duration::from_millis(800));

                let mtime = settings_file_mtime(&path);
                if mtime.is_none() || mtime == last_mtime {
                    continue;
                }

                last_mtime = mtime;

                match reload_global_settings() {
                    Ok(()) => info!(
                        "Settings reloaded after file change ({})",
                        path.display()
                    ),
                    Err(e) => warn!("Settings auto-reload failed: {}", e),
                }
            }
        })
        .ok();
}

pub fn init_settings() -> structs::Settings {
    let db_file_path = get_db_file_path();

    info!(
        "Loading settings db file located at: {}",
        db_file_path.display()
    );

    if db_file_path.exists() {
        if let Ok(db_file) = File::open(&db_file_path) {
            let reader = BufReader::new(db_file);
            if let Ok(settings) = serde_json::from_reader(reader) {
                info!("Settings loaded.");
                return settings;
            }
        }
    }

    warn!("No settings file found or there was an error parsing it. Creating default struct.");
    structs::Settings::default()
}

/// init settings and return a SettingsManager ready to use
pub fn init() -> SettingsManager {
    let settings = init_settings();
    SettingsManager::new(settings)
}

/// Reload settings from disk into the global `DB` (jarvis-app process).
pub fn reload_global_settings() -> Result<(), String> {
    let fresh = init_settings();
    let db = crate::DB
        .get()
        .ok_or_else(|| "DB not initialized".to_string())?;
    *db.write() = fresh;
    Ok(())
}

pub fn save_settings(settings: &structs::Settings) -> Result<(), std::io::Error> {
    let db_file_path = get_db_file_path();

    std::fs::write(
        &db_file_path,
        serde_json::to_string_pretty(&settings).unwrap(),
    )?;

    info!("Settings saved to: {:#}", db_file_path.display());
    Ok(())
}
