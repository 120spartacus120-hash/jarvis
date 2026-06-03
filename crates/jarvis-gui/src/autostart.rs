use std::path::{Path, PathBuf};

use jarvis_core::APP_CONFIG_DIR;

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "JARVIS";

fn launcher_script_path() -> Result<PathBuf, String> {
    let dir = APP_CONFIG_DIR
        .get()
        .ok_or_else(|| "Config directory is not initialized".to_string())?;
    Ok(dir.join("autostart.vbs"))
}

#[cfg(windows)]
fn write_launcher_script(exe: &Path) -> Result<PathBuf, String> {
    let exe_dir = exe
        .parent()
        .ok_or_else(|| "Не удалось определить папку JARVIS.".to_string())?;
    let script_path = launcher_script_path()?;

    let exe_dir = exe_dir.to_string_lossy().replace('"', "\"\"");
    let exe_path = exe.to_string_lossy().replace('"', "\"\"");

    let script = format!(
        r#"Set sh = CreateObject("WScript.Shell")
Set fso = CreateObject("Scripting.FileSystemObject")
exeDir = "{exe_dir}"
exePath = "{exe_path}"
If Not fso.FileExists(exePath) Then WScript.Quit 1
sh.CurrentDirectory = exeDir
sh.Run """" & exePath & """", 1, False
"#,
        exe_dir = exe_dir,
        exe_path = exe_path,
    );

    std::fs::write(&script_path, script).map_err(|e| e.to_string())?;
    Ok(script_path)
}

#[cfg(windows)]
pub fn is_enabled() -> bool {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let Ok(run) = hkcu.open_subkey(RUN_KEY) else {
        return false;
    };

    run.get_value::<String, _>(VALUE_NAME).is_ok()
}

#[cfg(windows)]
pub fn set_enabled(enabled: bool) -> Result<(), String> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (run, _) = hkcu
        .create_subkey(RUN_KEY)
        .map_err(|e| format!("Не удалось открыть автозагрузку Windows: {}", e))?;

    if enabled {
        let exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let script = write_launcher_script(&exe)?;
        let command = format!(
            "wscript.exe \"{}\"",
            script.to_string_lossy().replace('"', "\"\"")
        );
        run.set_value(VALUE_NAME, &command)
            .map_err(|e| format!("Не удалось включить автозапуск: {}", e))?;
    } else if let Err(e) = run.delete_value(VALUE_NAME) {
        log::warn!("autostart registry remove: {}", e);
    }

    if !enabled {
        if let Ok(path) = launcher_script_path() {
            let _ = std::fs::remove_file(path);
        }
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn is_enabled() -> bool {
    false
}

#[cfg(not(windows))]
pub fn set_enabled(_enabled: bool) -> Result<(), String> {
    Err("Автозапуск доступен только в Windows.".into())
}
