use once_cell::sync::OnceCell;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

// use kira::{
//     manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
//     sound::static_sound::{StaticSoundData, StaticSoundSettings},
// };

use kira::{
    AudioManager, AudioManagerSettings, Decibels, DefaultBackend,
    sound::static_sound::StaticSoundData,
};

static MANAGER: OnceCell<Mutex<AudioManager>> = OnceCell::new();

pub fn init() -> Result<(), ()> {
    if MANAGER.get().is_some() {
        return Ok(());
    }  // already initialized

    // Create an audio manager. This plays sounds and manages resources.
    match AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()) {
        Ok(manager) => {
            // store
            MANAGER.set(Mutex::new(manager)).ok();

            // success
            Ok(())
        }
        Err(msg) => {
            error!("Failed to initialize audio stream.\nError details: {}", msg);

            // failed
            Err(())
        }
    }
}

pub fn play_sound(filename: &PathBuf) {
    play_sound_internal(filename, false, None);
}

pub fn play_sound_blocking(filename: &PathBuf) {
    play_sound_internal(filename, true, None);
}

/// Boost quiet recordings (weather clips) to match louder intro phrases.
pub fn play_sound_blocking_boosted(filename: &PathBuf, gain_db: f32) {
    play_sound_internal(filename, true, Some(gain_db));
}

fn play_sound_internal(filename: &PathBuf, blocking: bool, gain_db: Option<f32>) {
    match StaticSoundData::from_file(filename) {
        Ok(sound_data) => {
            let sound_data = if let Some(db) = gain_db {
                sound_data.volume(Decibels(db))
            } else {
                sound_data
            };
            let duration = sound_data.duration();

            if let Some(manager) = MANAGER.get() {
                if let Ok(mut audio_manager) = manager.lock() {
                    if let Err(e) = audio_manager.play(sound_data) {
                        warn!("Failed to play sound: {}", e);
                        return;
                    }
                }
            } else {
                warn!("Audio manager not initialized");
                return;
            }

            if blocking && duration.is_zero() {
                std::thread::sleep(Duration::from_millis(1800));
            } else if blocking && !duration.is_zero() {
                std::thread::sleep(duration + Duration::from_millis(250));
            }
        }
        Err(err) => {
            warn!("Cannot find sound file: {} (err: {})", filename.display(), err);
        }
    }
}