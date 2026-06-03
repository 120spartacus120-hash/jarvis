use once_cell::sync::OnceCell;
use pv_recorder::{PvRecorder, PvRecorderBuilder};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::path_util::native_library_path;
use crate::APP_DIR;

static RECORDER: OnceCell<PvRecorder> = OnceCell::new();
static IS_RECORDING: AtomicBool = AtomicBool::new(false);

fn pvrecorder_library_path() -> PathBuf {
    let candidates = [
        APP_DIR.join("lib/windows/amd64/libpv_recorder.dll"),
        APP_DIR.join("libpv_recorder.dll"),
    ];

    for path in &candidates {
        if path.exists() {
            let resolved = native_library_path(path);
            info!("Using PvRecorder library: {}", resolved.display());
            return resolved;
        }
    }

    warn!(
        "PvRecorder DLL not found at {:?} or {:?}",
        candidates[0], candidates[1]
    );
    native_library_path(&candidates[0])
}

fn pvrecorder_builder(frame_length: i32, device_index: i32) -> PvRecorderBuilder {
    PvRecorderBuilder::new(frame_length)
        .device_index(device_index)
        .library_path(&pvrecorder_library_path())
}

pub fn init_microphone(device_index: i32, frame_length: u32) -> bool {
    if RECORDER.get().is_some() {
        return true;
    }

    match pvrecorder_builder(frame_length as i32, device_index).init() {
        Ok(pv) => {
            let _ = RECORDER.set(pv);
            true
        }
        Err(msg) => {
            error!("Failed to initialize pvrecorder.\nError details: {:?}", msg);
            false
        }
    }
}

pub fn read_microphone(frame_buffer: &mut [i16]) {
    if RECORDER.get().is_some() {
        let frame = RECORDER.get().unwrap().read();

        match frame {
            Ok(f) => {
                frame_buffer.copy_from_slice(f.as_slice());
            }
            Err(msg) => {
                error!("Failed to read audio frame. {:?}", msg);
            }
        }
    }
}

pub fn start_recording(device_index: i32, frame_length: u32) -> Result<(), ()> {
    if !init_microphone(device_index, frame_length) {
        return Err(());
    }

    match RECORDER.get().unwrap().start() {
        Ok(_) => {
            info!("START recording from microphone ...");
            IS_RECORDING.store(true, Ordering::SeqCst);
            Ok(())
        }
        Err(msg) => {
            error!("Failed to START audio recording: {}", msg);
            Err(())
        }
    }
}

pub fn stop_recording() -> Result<(), ()> {
    if RECORDER.get().is_some() && IS_RECORDING.load(Ordering::SeqCst) {
        match RECORDER.get().unwrap().stop() {
            Ok(_) => {
                info!("STOP recording from microphone ...");
                IS_RECORDING.store(false, Ordering::SeqCst);
                return Ok(());
            }
            Err(msg) => {
                error!("Failed to STOP audio recording: {}", msg);
                return Err(());
            }
        }
    }

    Ok(())
}

pub fn list_audio_devices() -> Vec<String> {
    match pvrecorder_builder(512, -1).get_available_devices() {
        Ok(audio_devices) => audio_devices,
        Err(err) => {
            error!("Failed to get audio devices: {}", err);
            Vec::new()
        }
    }
}

pub fn get_audio_device_name(idx: i32) -> String {
    if idx == -1 {
        return String::from("System Default");
    }

    let audio_devices = list_audio_devices();
    let mut first_device: String = String::new();

    for (_idx, device) in audio_devices.iter().enumerate() {
        if idx as usize == _idx {
            return device.to_string();
        }

        if _idx == 0 {
            first_device = device.to_string()
        }
    }

    first_device
}
