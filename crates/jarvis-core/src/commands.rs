use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::Duration;
use std::process::{Child, Command};

use seqdiff::ratio;

mod structs;
mod yaml;
pub use structs::*;

use crate::{config, i18n, APP_DIR};

#[cfg(feature = "lua")]
use crate::lua::{self, SandboxLevel, CommandContext};

fn load_toml_commands(cmd_path: &Path) -> Option<Vec<JCommand>> {
    let toml_file = cmd_path.join("command.toml");
    if !toml_file.exists() {
        return None;
    }

    let content = match fs::read_to_string(&toml_file) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to read {}: {}", toml_file.display(), e);
            return None;
        }
    };

    let file: JCommandsList = match toml::from_str(&content) {
        Ok(f) => f,
        Err(e) => {
            warn!("Failed to parse {}: {}", toml_file.display(), e);
            return None;
        }
    };

    Some(file.commands)
}

pub fn parse_commands() -> Result<Vec<JCommandsList>, String> {
    let mut commands: Vec<JCommandsList> = Vec::new();

    let commands_path = APP_DIR.join(config::COMMANDS_PATH);
    let cmd_dirs = fs::read_dir(&commands_path)
        .map_err(|e| format!("Error reading commands directory {:?}: {}", commands_path, e))?;

    for entry in cmd_dirs.flatten() {
        let cmd_path = entry.path();
        if !cmd_path.is_dir() {
            continue;
        }

        if let Some(cmds) = load_toml_commands(&cmd_path) {
            commands.push(JCommandsList {
                path: cmd_path,
                commands: cmds,
            });
            continue;
        }

        if let Some(cmds) = yaml::load_yaml_commands(&cmd_path) {
            commands.push(JCommandsList {
                path: cmd_path,
                commands: cmds,
            });
        }
    }

    if commands.is_empty() {
        Err("No commands found".into())
    } else {
        info!("Loaded {} command pack(s)", commands.len());
        Ok(commands)
    }
}


pub fn commands_hash(commands: &[JCommandsList]) -> String {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    
    let lang = i18n::get_language();
    hasher.update(lang.as_bytes());
    hasher.update(b"|");

    // collect all command ids and phrases for current language, sorted
    let mut all_data: Vec<(&str, _)> = commands.iter()
        .flat_map(|ac| ac.commands.iter().map(|c| (c.id.as_str(), c.get_phrases(&lang))))
        .collect();
    all_data.sort_by_key(|(id, _)| *id);
    
    for (id, phrases) in all_data {
        hasher.update(id.as_bytes());
        for phrase in phrases.iter() {
            hasher.update(phrase.as_bytes());
        }
    }
    
    format!("{:x}", hasher.finalize())
}


pub fn fetch_command<'a>(
    phrase: &str,
    commands: &'a [JCommandsList],
) -> Option<(&'a PathBuf, &'a JCommand)> {
    let lang = i18n::get_language();

    let phrase = crate::speech_normalize::normalize(&phrase.trim().to_lowercase());
    if phrase.is_empty() {
        return None;
    }

    let phrase_chars: Vec<char> = phrase.chars().collect();
    let phrase_words: Vec<&str> = phrase.split_whitespace().collect();

    let mut result: Option<(&PathBuf, &JCommand)> = None;
    let mut best_score = config::CMD_RATIO_THRESHOLD;

    for cmd_list in commands {
        for cmd in &cmd_list.commands {
            let cmd_phrases = cmd.get_phrases(&lang);
            
            for cmd_phrase in cmd_phrases.iter() {
                let cmd_phrase_lower = cmd_phrase.trim().to_lowercase();
                let cmd_phrase_chars: Vec<char> = cmd_phrase_lower.chars().collect();
                
                // character-level similarity
                let char_ratio = ratio(&phrase_chars, &cmd_phrase_chars);
                
                // word-level similarity
                let cmd_words: Vec<&str> = cmd_phrase_lower.split_whitespace().collect();
                let word_score = word_overlap_score(&phrase_words, &cmd_words);
                
                // combined score
                let score = (char_ratio * 0.6) + (word_score * 0.4);
                
                // early exit on perfect match
                if score >= 99.0 {
                    debug!("Perfect match: '{}' -> '{}'", phrase, cmd_phrase_lower);
                    return Some((&cmd_list.path, cmd));
                }
                
                if score > best_score {
                    best_score = score;
                    result = Some((&cmd_list.path, cmd));
                }
            }
        }
    }

    if let Some((_, cmd)) = result {
        info!("Fuzzy match: '{}' -> cmd '{}' (score: {:.1}%)", phrase, cmd.id, best_score);
    } else {
        debug!("No match for '{}' (best: {:.1}%)", phrase, best_score);
    }
    
    result
}

/// Pick today vs tomorrow weather command by phrase (not always `builtin_weather`).
#[cfg(feature = "jarvis_app")]
pub fn fetch_weather_command<'a>(
    phrase: &str,
    commands: &'a [JCommandsList],
) -> Option<(&'a PathBuf, &'a JCommand)> {
    let phrase = crate::speech_normalize::normalize(&phrase.trim().to_lowercase());
    if phrase.is_empty() {
        return None;
    }

    if crate::weather::is_tomorrow_request(&phrase) {
        if let Some(found) = get_command_by_id(commands, "builtin_weather_tomorrow") {
            info!("Weather: tomorrow request detected in '{}'", phrase);
            return Some(found);
        }
    }

    if phrase.split_whitespace().any(|w| {
        let t = w.trim_matches(|c: char| !c.is_alphanumeric());
        t == "сегодня" || t.starts_with("сегодн")
    }) {
        if let Some(found) = get_command_by_id(commands, "builtin_weather") {
            info!("Weather: today request detected in '{}'", phrase);
            return Some(found);
        }
    }

    let phrase_chars: Vec<char> = phrase.chars().collect();
    let phrase_words: Vec<&str> = phrase.split_whitespace().collect();
    let lang = i18n::get_language();

    let mut result: Option<(&PathBuf, &JCommand)> = None;
    let mut best_score = config::CMD_RATIO_THRESHOLD;

    for cmd_list in commands {
        for cmd in &cmd_list.commands {
            if cmd.cmd_type != "weather" {
                continue;
            }

            let cmd_phrases = cmd.get_phrases(&lang);
            for cmd_phrase in cmd_phrases.iter() {
                let cmd_phrase_lower = cmd_phrase.trim().to_lowercase();
                let cmd_phrase_chars: Vec<char> = cmd_phrase_lower.chars().collect();
                let char_ratio = ratio(&phrase_chars, &cmd_phrase_chars);
                let cmd_words: Vec<&str> = cmd_phrase_lower.split_whitespace().collect();
                let word_score = word_overlap_score(&phrase_words, &cmd_words);
                let mut score = (char_ratio * 0.6) + (word_score * 0.4);

                if cmd.id == "builtin_weather_tomorrow" && cmd_phrase_lower.contains("завтра") {
                    score += 8.0;
                }

                if score >= 99.0 {
                    info!(
                        "Weather match: '{}' -> '{}' ({})",
                        phrase, cmd_phrase_lower, cmd.id
                    );
                    return Some((&cmd_list.path, cmd));
                }

                if score > best_score {
                    best_score = score;
                    result = Some((&cmd_list.path, cmd));
                }
            }
        }
    }

    if let Some((_, cmd)) = result {
        info!(
            "Weather fuzzy: '{}' -> {} (score: {:.1}%)",
            phrase, cmd.id, best_score
        );
    }

    result.or_else(|| get_command_by_id(commands, "builtin_weather"))
}

/// Match greeting phrases (exact or fuzzy) against `builtin_greeting`.
#[cfg(feature = "jarvis_app")]
pub fn fetch_greeting_command<'a>(
    phrase: &str,
    commands: &'a [JCommandsList],
) -> Option<(&'a PathBuf, &'a JCommand)> {
    let phrase = crate::speech_normalize::normalize(&phrase.trim().to_lowercase());
    if phrase.is_empty() {
        return None;
    }

    let config = crate::custom_commands::load();
    let greeting_phrases = config.greeting_phrases;
    if greeting_phrases.is_empty() {
        return None;
    }

    if greeting_phrases.iter().any(|p| p == &phrase) {
        info!("Greeting exact match: '{}'", phrase);
        return get_command_by_id(commands, "builtin_greeting");
    }

    let phrase_chars: Vec<char> = phrase.chars().collect();
    let phrase_words: Vec<&str> = phrase.split_whitespace().collect();
    let lang = crate::i18n::get_language();

    let mut result: Option<(&PathBuf, &JCommand)> = None;
    let mut best_score = config::CMD_RATIO_THRESHOLD;

    for cmd_list in commands {
        for cmd in &cmd_list.commands {
            if cmd.id != "builtin_greeting" {
                continue;
            }

            for cmd_phrase in cmd.get_phrases(&lang).iter() {
                let cmd_phrase_lower = cmd_phrase.trim().to_lowercase();
                let cmd_phrase_chars: Vec<char> = cmd_phrase_lower.chars().collect();
                let char_ratio = ratio(&phrase_chars, &cmd_phrase_chars);
                let cmd_words: Vec<&str> = cmd_phrase_lower.split_whitespace().collect();
                let word_score = word_overlap_score(&phrase_words, &cmd_words);
                let score = (char_ratio * 0.6) + (word_score * 0.4);

                if score >= 99.0 {
                    info!(
                        "Greeting match: '{}' -> '{}'",
                        phrase, cmd_phrase_lower
                    );
                    return Some((&cmd_list.path, cmd));
                }

                if score > best_score {
                    best_score = score;
                    result = Some((&cmd_list.path, cmd));
                }
            }
        }
    }

    if let Some((_, cmd)) = result {
        info!(
            "Greeting fuzzy: '{}' -> {} (score: {:.1}%)",
            phrase, cmd.id, best_score
        );
    }

    result
}

fn word_overlap_score(input_words: &[&str], cmd_words: &[&str]) -> f64 {
    if input_words.is_empty() || cmd_words.is_empty() {
        return 0.0;
    }

    let mut matched = 0.0;
    
    // pre-compute cmd word chars to avoid repeated allocations
    let cmd_word_chars: Vec<Vec<char>> = cmd_words
        .iter()
        .map(|w| w.chars().collect())
        .collect();
    
    for input_word in input_words {
        let input_chars: Vec<char> = input_word.chars().collect();
        
        let best_word_match = cmd_word_chars
            .iter()
            .map(|cw| ratio(&input_chars, cw))
            .fold(0.0_f64, f64::max);
        
        if best_word_match > 70.0 {
            matched += best_word_match / 100.0;
        }
    }

    let max_words = input_words.len().max(cmd_words.len()) as f64;
    (matched / max_words) * 100.0
}




pub fn execute_exe(exe: &str, args: &[String]) -> std::io::Result<Child> {
    Command::new(exe).args(args).spawn()
}

/// Resolve AHK script path: config may say `.exe` while repo ships `.ahk`.
fn resolve_ahk_script_path(cmd_path: &Path, exe_path_config: &str) -> PathBuf {
    let absolute = Path::new(exe_path_config);
    if absolute.is_absolute() && absolute.exists() {
        return absolute.to_path_buf();
    }

    let local = cmd_path.join(exe_path_config);
    if local.exists() {
        return local;
    }

    if exe_path_config.ends_with(".exe") {
        let ahk = cmd_path.join(exe_path_config.replace(".exe", ".ahk"));
        if ahk.exists() {
            return ahk;
        }
    }

    local
}

#[cfg(target_os = "windows")]
fn find_autohotkey_executable() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from(r"C:\Program Files\AutoHotkey\AutoHotkey.exe"),
        PathBuf::from(r"C:\Program Files\AutoHotkey\v1.1.37.02\AutoHotkeyU64.exe"),
        PathBuf::from(r"C:\Program Files\AutoHotkey\v1.1.37.02\AutoHotkeyA32.exe"),
        PathBuf::from(r"C:\Program Files\AutoHotkey\UX\AutoHotkeyUX.exe"),
    ];

    for path in candidates {
        if path.is_file() {
            return Some(path);
        }
    }

    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths).find_map(|dir| {
            let exe = dir.join("AutoHotkey.exe");
            if exe.is_file() {
                Some(exe)
            } else {
                None
            }
        })
    })
}

/// Fallback when AutoHotkey is not installed: emulate media keys via PowerShell.
#[cfg(target_os = "windows")]
fn send_volume_media_key(vk: u8, presses: u32) -> Result<(), String> {
    let ps = format!(
        r#"$s='[DllImport("user32.dll")]public static extern void keybd_event(byte bVk,byte bScan,uint dwFlags,int dwExtraInfo);'; Add-Type -MemberDefinition $s -Name W -Namespace D; 1..{presses} | %{{ [D.W]::keybd_event({vk},0,0,0); [D.W]::keybd_event({vk},0,2,0) }}"#,
        presses = presses,
        vk = vk
    );

    Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-WindowStyle",
            "Hidden",
            "-Command",
            &ps,
        ])
        .spawn()
        .map_err(|e| format!("Volume media key error: {}", e))?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn spawn_ahk_script(script_path: &Path, args: &[String]) -> Result<(), String> {
    const VK_VOLUME_MUTE: u8 = 0xAD;
    const VK_VOLUME_DOWN: u8 = 0xAE;
    const VK_VOLUME_UP: u8 = 0xAF;

    if script_path.extension() == Some(OsStr::new("exe")) && script_path.is_file() {
        execute_exe(
            script_path.to_str().ok_or("Invalid AHK exe path")?,
            args,
        )
        .map_err(|e| format!("AHK process spawn error: {}", e))?;
        return Ok(());
    }

    if script_path.extension() == Some(OsStr::new("ahk")) && script_path.is_file() {
        if let Some(ahk) = find_autohotkey_executable() {
            let mut cmd = Command::new(ahk);
            cmd.arg(script_path);
            cmd.args(args);
            cmd.spawn()
                .map_err(|e| format!("AutoHotkey spawn error: {}", e))?;
            return Ok(());
        }

        let name = script_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        if name.contains("volume up") {
            return send_volume_media_key(VK_VOLUME_UP, 2);
        }
        if name.contains("volume down") {
            return send_volume_media_key(VK_VOLUME_DOWN, 2);
        }
        if name.contains("mute") {
            return send_volume_media_key(VK_VOLUME_MUTE, 1);
        }

        return Err(format!(
            "AutoHotkey not found for {}. Install AutoHotkey v1 (volume up/down work without it).",
            script_path.display()
        ));
    }

    Err(format!("AHK script not found: {}", script_path.display()))
}

#[cfg(not(target_os = "windows"))]
fn spawn_ahk_script(_script_path: &Path, _args: &[String]) -> Result<(), String> {
    Err("AHK commands are only supported on Windows.".into())
}

pub fn execute_cli(cmd: &str, args: &[String]) -> std::io::Result<Child> {
    debug!("Spawning: cmd /C {} {:?}", cmd, args);

    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(cmd).args(args).spawn()
    } else {
        Command::new("sh").arg("-c").arg(cmd).args(args).spawn()
    }
}

pub fn execute_command(cmd_path: &PathBuf, cmd_config: &JCommand, phrase: Option<&str>, slots: Option<&HashMap<String, SlotValue>>) -> Result<bool, String> {
    // execute command by the type
    match cmd_config.cmd_type.as_str() {
        "open_program" => {
            let path = cmd_config.cli_cmd.trim();
            if path.is_empty() {
                return Err("Путь к программе не задан.".into());
            }
            crate::custom_commands::launch_program(path).map(|_| true)
        }

        "open_website" | "open_browser" => {
            let url = cmd_config.cli_cmd.trim();
            if url.is_empty() {
                return Err("Адрес сайта не задан.".into());
            }
            crate::custom_commands::launch_website(url).map(|_| true)
        }

        // BRUH
        "voice" | "dialog" => Ok(true),
        
        // LUA command
        #[cfg(feature = "lua")]
        "lua" => {
            execute_lua_command(cmd_path, cmd_config, phrase, slots)
        }

        // AutoHotkey command (.exe or .ahk; PowerShell fallback for volume keys)
        "ahk" => {
            let script_path = resolve_ahk_script_path(cmd_path, &cmd_config.exe_path);
            spawn_ahk_script(&script_path, &cmd_config.exe_args).map(|_| true)
        }
        
        // CLI command type
        // @TODO: Consider security restrictions
        "cli" => {
            execute_cli(&cmd_config.cli_cmd, &cmd_config.cli_args)
                .map(|_| true)
                .map_err(|e| format!("CLI command error: {}", e))
        }
        
        // TERMINATOR command (T1000) — exit is handled in jarvis-app after voice reply
        "terminate" => Ok(true),
        
        "volume_change" => {
            #[cfg(target_os = "windows")]
            {
                const VK_VOLUME_UP: u8 = 0xAF;
                const VK_VOLUME_DOWN: u8 = 0xAE;
                let percent = cmd_config
                    .exe_args
                    .first()
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(2)
                    .clamp(1, 100);
                
                // 1 media key press = 2% volume change in Windows
                let presses = (percent as f32 / 2.0).ceil() as u32;
                
                match cmd_config.cli_cmd.as_str() {
                    // Ok(false): do not chain — STT may re-recognize the same phrase and run twice.
                    "up" => send_volume_media_key(VK_VOLUME_UP, presses).map(|_| false),
                    "down" => send_volume_media_key(VK_VOLUME_DOWN, presses).map(|_| false),
                    other => Err(format!("Unknown volume direction: {}", other)),
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                Err("Volume commands are only supported on Windows.".into())
            }
        }

        #[cfg(feature = "jarvis_app")]
        "greeting" => {
            crate::greeting::play_random_greeting();
            Ok(false)
        }

        #[cfg(feature = "jarvis_app")]
        "weather" => {
            let day = crate::weather::day_for_command(&cmd_config.id, phrase);
            crate::weather::play_intro();
            match crate::weather::get_weather(day) {
                Ok(playlist) => {
                    crate::weather::play_day_intro(day);
                    if playlist.is_empty() {
                        crate::voices::play_error();
                    } else {
                        for path in playlist {
                            crate::weather::play_clip_blocking(&path);
                        }
                    }
                    Ok(false)
                }
                Err(e) => {
                    error!("Weather error: {}", e);
                    crate::voices::play_error();
                    Err(e)
                }
            }
        }

        // STOP CHANING
        "stop_chaining" => Ok(false),

        // other
        _ => {
            error!("Command type unknown: {}", cmd_config.cmd_type);
            Err(format!("Command type unknown: {}", cmd_config.cmd_type).into())
        }
    }
}

// look up a command by its ID
pub fn get_command_by_id<'a>(
    commands: &'a [JCommandsList],
    id: &str,
) -> Option<(&'a PathBuf, &'a JCommand)> {
    for cmd_list in commands {
        for cmd in &cmd_list.commands {
            if cmd.id == id {
                return Some((&cmd_list.path, cmd));
            }
        }
    }
    None
}

pub fn list_paths(commands: &[JCommandsList]) -> Vec<&Path> {
    commands.iter().map(|x| x.path.as_path()).collect()
}

#[cfg(feature = "lua")]
fn execute_lua_command(
    cmd_path: &PathBuf,
    cmd_config: &JCommand,
    phrase: Option<&str>,
    slots: Option<&HashMap<String, SlotValue>>
) -> Result<bool, String> {
    // get script path

    let script_name = if cmd_config.script.is_empty() {
        "script.lua"
    } else {
        &cmd_config.script
    };
    
    let script_path = cmd_path.join(script_name);
    
    if !script_path.exists() {
        return Err(format!("Lua script not found: {}", script_path.display()));
    }
    
    // parse sandbox level
    let sandbox = SandboxLevel::from_str(&cmd_config.sandbox);

    // create context
    let context = CommandContext {
        phrase: phrase.unwrap_or("").to_string(),
        command_id: cmd_config.id.clone(),
        command_path: cmd_path.clone(),
        language: i18n::get_language(),
        slots: slots.map(|s| s.clone()),
    };
    
    // get timeout
    let timeout = Duration::from_millis(cmd_config.timeout);
    
    info!("Executing Lua command: {} (sandbox: {:?}, timeout: {:?})", 
          cmd_config.id, sandbox, timeout);
    
    // execute
    match lua::execute(&script_path, context, sandbox, timeout) {
        Ok(result) => {
            info!("Lua command {} completed (chain: {})", cmd_config.id, result.chain);
            Ok(result.chain)
        }
        Err(e) => {
            error!("Lua command {} failed: {}", cmd_config.id, e);
            Err(e.to_string())
        }
    }
}