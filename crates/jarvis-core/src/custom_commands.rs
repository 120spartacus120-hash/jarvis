use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::commands::{JCommand, JCommandsList};
use crate::i18n;
use crate::APP_CONFIG_DIR;

const CONFIG_FILE: &str = "custom_commands.json";
const CUSTOM_SOUNDS_DIR: &str = "custom_sounds";
const AUDIO_EXTENSIONS: &[&str] = &["wav", "mp3", "ogg"];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCommand {
    pub id: String,
    pub name: String,
    #[serde(rename = "type", default = "default_open_program")]
    pub command_type: String,
    #[serde(default)]
    pub program_path: String,
    #[serde(default)]
    pub website_url: String,
    #[serde(default)]
    pub phrases: Vec<String>,
    #[serde(default)]
    pub user_line: String,
    #[serde(default)]
    pub jarvis_line: String,
    #[serde(default = "default_volume_percent")]
    pub volume_percent: u32,
    /// Legacy fields — read for migration, not written back.
    #[serde(default, skip_serializing)]
    pub volume_up_steps: u32,
    #[serde(default, skip_serializing)]
    pub volume_down_steps: u32,
    #[serde(default)]
    pub volume_up_phrases: Vec<String>,
    #[serde(default)]
    pub volume_down_phrases: Vec<String>,
}

fn default_open_program() -> String {
    "open_program".into()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomCommandsConfig {
    #[serde(default = "default_thanks_phrases")]
    pub thanks_phrases: Vec<String>,
    #[serde(default = "default_shutdown_phrases")]
    pub shutdown_phrases: Vec<String>,
    #[serde(default = "default_weather_phrases")]
    pub weather_phrases: Vec<String>,
    #[serde(default)]
    pub user_commands: Vec<UserCommand>,
}

fn default_volume_percent() -> u32 {
    2
}

fn normalize_volume_percent(percent: u32) -> u32 {
    let mut p = if percent == 0 { 2 } else { percent.clamp(2, 100) };
    if p % 2 != 0 {
        p += 1;
    }
    p.min(100)
}

fn migrate_volume_percent(cmd: &mut UserCommand) {
    if cmd.volume_up_steps > 0 || cmd.volume_down_steps > 0 {
        let legacy = cmd.volume_up_steps.max(cmd.volume_down_steps);
        if cmd.volume_percent == 0 || legacy > cmd.volume_percent {
            cmd.volume_percent = legacy;
        }
    }
    cmd.volume_percent = normalize_volume_percent(cmd.volume_percent);
    cmd.volume_up_steps = 0;
    cmd.volume_down_steps = 0;
}

fn default_thanks_phrases() -> Vec<String> {
    vec![
        "спасибо".into(),
        "молодец".into(),
        "респект".into(),
        "ты супер".into(),
        "отличная работа".into(),
        "ты крут".into(),
    ]
}

fn default_shutdown_phrases() -> Vec<String> {
    vec![
        "выключись".into(),
        "вырубись".into(),
        "закройся".into(),
        "отключись".into(),
        "на сегодня хватит".into(),
        "пора спать".into(),
    ]
}

/// Phrases used to detect today vs tomorrow (includes defaults if user removed them).
pub fn weather_phrases_for_day_detect() -> Vec<String> {
    let config = load();
    let mut phrases = config.weather_phrases.clone();
    let has_tomorrow = phrases.iter().any(|p| p.contains("завтра"));
    let has_today = phrases.iter().any(|p| p.contains("сегодня"));

    if !has_tomorrow || !has_today {
        for p in default_weather_phrases() {
            let need = (p.contains("завтра") && !has_tomorrow)
                || (p.contains("сегодня") && !has_today);
            if need && !phrases.iter().any(|x| x == &p) {
                phrases.push(p);
            }
        }
    }

    phrases
}

fn default_weather_phrases() -> Vec<String> {
    let mut phrases = default_weather_today_phrases();
    phrases.extend(default_weather_tomorrow_phrases());
    phrases
}

fn default_weather_today_phrases() -> Vec<String> {
    vec![
        "какая погода".into(),
        "какая сегодня погода".into(),
        "какая погода сегодня".into(),
        "погода".into(),
        "погода сегодня".into(),
        "погода на сегодня".into(),
        "скажи погоду".into(),
        "узнай погоду".into(),
    ]
}

fn default_weather_tomorrow_phrases() -> Vec<String> {
    vec![
        "какая погода завтра".into(),
        "погода на завтра".into(),
        "погода завтра".into(),
        "прогноз на завтра".into(),
        "погода завтрашнего дня".into(),
    ]
}

fn split_weather_phrases(phrases: &[String]) -> (Vec<String>, Vec<String>) {
    let mut today = Vec::new();
    let mut tomorrow = Vec::new();

    for p in phrases {
        let s = p.trim().to_lowercase();
        if s.is_empty() {
            continue;
        }
        if s.contains("завтра") && !s.contains("послезавтра") {
            tomorrow.push(s);
        } else {
            today.push(s);
        }
    }

    if today.is_empty() {
        today = default_weather_today_phrases();
    }
    if tomorrow.is_empty() {
        tomorrow = default_weather_tomorrow_phrases();
    }

    (today, tomorrow)
}

impl Default for CustomCommandsConfig {
    fn default() -> Self {
        Self {
            thanks_phrases: default_thanks_phrases(),
            shutdown_phrases: default_shutdown_phrases(),
            weather_phrases: default_weather_phrases(),
            user_commands: Vec::new(),
        }
    }
}

pub fn config_path() -> Result<PathBuf, String> {
    APP_CONFIG_DIR
        .get()
        .map(|dir| dir.join(CONFIG_FILE))
        .ok_or_else(|| "Config directory is not initialized".into())
}

pub fn custom_sounds_dir() -> Result<PathBuf, String> {
    APP_CONFIG_DIR
        .get()
        .map(|dir| dir.join(CUSTOM_SOUNDS_DIR))
        .ok_or_else(|| "Config directory is not initialized".into())
}

pub fn ensure_custom_sounds_dir() -> Result<PathBuf, String> {
    let dir = custom_sounds_dir()?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

pub fn import_custom_sound(source: &Path) -> Result<String, String> {
    if !source.exists() {
        return Err(format!("Файл не найден: {}", source.display()));
    }

    let ext = source
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .ok_or_else(|| "Укажите файл .wav, .mp3 или .ogg".to_string())?;

    if !AUDIO_EXTENSIONS.contains(&ext.as_str()) {
        return Err("Поддерживаются только .wav, .mp3 и .ogg".into());
    }

    let file_name = source
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Некорректное имя файла".to_string())?;

    let dir = ensure_custom_sounds_dir()?;
    let target = dir.join(file_name);
    fs::copy(source, &target).map_err(|e| format!("Не удалось скопировать звук: {}", e))?;

    // Store stem in commands (e.g. "th_n_n_noe") — extension is resolved at playback.
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name);
    Ok(stem.to_string())
}

pub fn load() -> CustomCommandsConfig {
    let path = match config_path() {
        Ok(path) => path,
        Err(e) => {
            warn!("{}", e);
            return CustomCommandsConfig::default();
        }
    };

    if !path.exists() {
        let default = CustomCommandsConfig::default();
        if let Err(e) = save(&default) {
            warn!("Failed to write default custom commands: {}", e);
        }
        return default;
    }

    match fs::read_to_string(&path) {
        Ok(content) => {
            let config: CustomCommandsConfig = serde_json::from_str(&content).unwrap_or_else(|e| {
                warn!("Failed to parse {}: {}", path.display(), e);
                CustomCommandsConfig::default()
            });
            let before = config.user_commands.len();
            let normalized = normalize_config(config);
            if normalized.user_commands.len() != before {
                let _ = save(&normalized);
            }
            normalized
        }
        Err(e) => {
            warn!("Failed to read {}: {}", path.display(), e);
            CustomCommandsConfig::default()
        }
    }
}

pub fn save(config: &CustomCommandsConfig) -> Result<(), String> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let normalized = normalize_config(config.clone());
    let json = serde_json::to_string_pretty(&normalized).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn commands_count(config: &CustomCommandsConfig) -> usize {
    4 + normalize_config(config.clone()).user_commands.len()
}

fn normalize_phrases(phrases: Vec<String>, fallback: fn() -> Vec<String>) -> Vec<String> {
    let normalized: Vec<String> = phrases
        .into_iter()
        .map(|p| p.trim().to_lowercase())
        .filter(|p| !p.is_empty())
        .collect();

    if normalized.is_empty() {
        fallback()
    } else {
        normalized
    }
}

pub fn normalize_config(mut config: CustomCommandsConfig) -> CustomCommandsConfig {
    config.thanks_phrases = normalize_phrases(config.thanks_phrases, default_thanks_phrases);
    config.shutdown_phrases = normalize_phrases(config.shutdown_phrases, default_shutdown_phrases);
    config.weather_phrases = normalize_phrases(config.weather_phrases, default_weather_phrases);

    config.user_commands = config
        .user_commands
        .into_iter()
        .filter_map(|mut cmd| {
            cmd.id = cmd.id.trim().to_string();
            cmd.name = cmd.name.trim().to_string();
            cmd.program_path = cmd.program_path.trim().to_string();
            cmd.website_url = cmd.website_url.trim().to_string();
            cmd.command_type = cmd.command_type.trim().to_string();
            if cmd.command_type.is_empty() {
                cmd.command_type = default_open_program();
            }

            cmd.phrases = cmd
                .phrases
                .into_iter()
                .map(|p| p.trim().to_lowercase())
                .filter(|p| !p.is_empty())
                .collect();

            cmd.user_line = cmd.user_line.trim().to_lowercase();
            cmd.jarvis_line = cmd.jarvis_line.trim().to_string();
            if cmd.command_type.as_str() == "volume_control" {
                migrate_volume_percent(&mut cmd);
            }
            cmd.volume_up_phrases = cmd
                .volume_up_phrases
                .into_iter()
                .map(|p| p.trim().to_lowercase())
                .filter(|p| !p.is_empty())
                .collect();
            cmd.volume_down_phrases = cmd
                .volume_down_phrases
                .into_iter()
                .map(|p| p.trim().to_lowercase())
                .filter(|p| !p.is_empty())
                .collect();

            if cmd.id.is_empty() || cmd.name.is_empty() {
                return None;
            }

            if cmd.command_type.as_str() == "dialog" {
                return None;
            }

            match cmd.command_type.as_str() {
                "open_program" => {
                    if cmd.program_path.is_empty() {
                        return None;
                    }
                }
                "open_website" => {
                    if cmd.website_url.is_empty() && !cmd.program_path.is_empty() {
                        cmd.website_url = normalize_website_url(&cmd.program_path);
                        cmd.program_path.clear();
                    } else {
                        cmd.website_url = normalize_website_url(&cmd.website_url);
                    }
                    if cmd.website_url.is_empty() {
                        return None;
                    }
                }
                "volume_control" => {
                    if cmd.volume_up_phrases.is_empty() && cmd.volume_down_phrases.is_empty() {
                        return None;
                    }
                }
                _ => return None,
            }

            Some(cmd)
        })
        .collect();

    config
}

fn phrase_map(lang: &str, phrases: &[String]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    map.insert(lang.to_string(), phrases.to_vec());
    map.insert("ru".into(), phrases.to_vec());
    map
}

fn sound_map(lang: &str, sounds: &[&str]) -> HashMap<String, Vec<String>> {
    let values: Vec<String> = sounds.iter().map(|s| s.to_string()).collect();
    sound_map_from_strings(lang, &values)
}

fn sound_map_from_strings(lang: &str, sounds: &[String]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    map.insert(lang.to_string(), sounds.to_vec());
    map.insert("ru".into(), sounds.to_vec());
    map
}

fn dialog_sounds_from_response(response: &str) -> Vec<String> {
    let trimmed = response.trim();

    if trimmed.ends_with(".mp3")
        || trimmed.ends_with(".wav")
        || trimmed.ends_with(".ogg")
    {
        return vec!["reply1".into(), "reply2".into(), "ok1".into()];
    }

    if !trimmed.contains(' ') && trimmed.len() <= 32 {
        return vec![trimmed.to_lowercase()];
    }

    let lower = trimmed.to_lowercase();
    if lower.contains("спасиб") {
        return vec!["thanks".into()];
    }
    if lower.contains("конечно") || lower.contains("да, ") || lower == "да" {
        return vec!["ok1".into(), "ok2".into(), "ok3".into()];
    }
    if lower.contains("привет") || lower.contains("здрав") {
        return vec!["greet1".into(), "reply1".into(), "reply2".into()];
    }

    vec![
        "reply1".into(),
        "reply2".into(),
        "reply3".into(),
        "ok1".into(),
    ]
}

fn is_audio_file_path(path: &Path) -> bool {
    path.exists()
        && path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| AUDIO_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
            .unwrap_or(false)
}

fn find_in_custom_sounds(name: &str) -> Option<PathBuf> {
    let dir = ensure_custom_sounds_dir().ok()?;
    let path = Path::new(name);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(name);
    let has_ext = path.extension().is_some();

    let mut candidates: Vec<PathBuf> = Vec::new();
    if has_ext {
        candidates.push(dir.join(name));
    } else {
        for ext in AUDIO_EXTENSIONS {
            candidates.push(dir.join(format!("{}.{}", stem, ext)));
        }
        candidates.push(dir.join(name));
    }

    candidates.into_iter().find(|p| is_audio_file_path(p))
}

pub fn resolve_response_audio(response: &str) -> Option<PathBuf> {
    let trimmed = response.trim();
    if trimmed.is_empty() {
        return None;
    }

    let direct = PathBuf::from(trimmed);
    if is_audio_file_path(&direct) {
        return Some(direct);
    }

    find_in_custom_sounds(trimmed)
}

pub fn dialog_audio_path(response: &str) -> Option<PathBuf> {
    resolve_response_audio(response)
}

fn sounds_for_response(response: &str, lang: &str) -> HashMap<String, Vec<String>> {
    if resolve_response_audio(response).is_some() {
        return HashMap::new();
    }

    let trimmed = response.trim();
    if trimmed.is_empty() {
        return sound_map(lang, &["ok1", "ok2", "ok3", "ok4"]);
    }

    sound_map_from_strings(lang, &dialog_sounds_from_response(trimmed))
}

fn command_description(cmd: &UserCommand) -> String {
    cmd.name.clone()
}

fn user_command_url(cmd: &UserCommand) -> String {
    if !cmd.website_url.is_empty() {
        cmd.website_url.clone()
    } else {
        cmd.program_path.clone()
    }
}

pub fn command_response_line(cmd: &crate::commands::JCommand) -> &str {
    match cmd.cmd_type.as_str() {
        "open_program" | "open_website" => cmd.script.as_str(),
        _ => cmd.cli_cmd.as_str(),
    }
}

pub fn play_command_response(response: &str, fallback_sounds: &[String]) {
    if let Some(path) = resolve_response_audio(response) {
        crate::audio::play_sound(&path);
        return;
    }

    if fallback_sounds.is_empty() {
        crate::voices::play_ok();
    } else {
        crate::voices::play_random_from(fallback_sounds);
    }
}

pub fn to_commands_list(config: &CustomCommandsConfig) -> Vec<JCommandsList> {
    let config = normalize_config(config.clone());
    let storage_path = config_path().unwrap_or_else(|_| PathBuf::from("custom"));
    let lang = i18n::get_language();

    let mut commands = Vec::new();

    commands.push(JCommand::new(
        "builtin_thanks".into(),
        "voice".into(),
        "Спасибо".into(),
        String::new(),
        vec![],
        String::new(),
        vec![],
        String::new(),
        String::new(),
        0,
        sound_map(&lang, &["thanks"]),
        phrase_map(&lang, &config.thanks_phrases),
        HashMap::new(),
    ));

    commands.push(JCommand::new(
        "builtin_shutdown".into(),
        "terminate".into(),
        "Выключение".into(),
        String::new(),
        vec![],
        String::new(),
        vec![],
        String::new(),
        String::new(),
        0,
        sound_map(&lang, &["off"]),
        phrase_map(&lang, &config.shutdown_phrases),
        HashMap::new(),
    ));

    let (weather_today, weather_tomorrow) = split_weather_phrases(&config.weather_phrases);

    commands.push(JCommand::new(
        "builtin_weather".into(),
        "weather".into(),
        "Погода сегодня".into(),
        String::new(),
        vec![],
        String::new(),
        vec![],
        String::new(),
        String::new(),
        0,
        HashMap::new(),
        phrase_map(&lang, &weather_today),
        HashMap::new(),
    ));

    commands.push(JCommand::new(
        "builtin_weather_tomorrow".into(),
        "weather".into(),
        "Погода завтра".into(),
        String::new(),
        vec![],
        String::new(),
        vec![],
        String::new(),
        String::new(),
        0,
        HashMap::new(),
        phrase_map(&lang, &weather_tomorrow),
        HashMap::new(),
    ));

    for user_cmd in &config.user_commands {
        match user_cmd.command_type.as_str() {
            "open_program" => {
                if user_cmd.program_path.is_empty() {
                    continue;
                }

                let response = user_cmd.jarvis_line.clone();
                commands.push(JCommand::new(
                    user_cmd.id.clone(),
                    "open_program".into(),
                    command_description(user_cmd),
                    String::new(),
                    vec![],
                    user_cmd.program_path.clone(),
                    vec![],
                    response.clone(),
                    String::new(),
                    0,
                    sounds_for_response(&response, &lang),
                    phrase_map(&lang, &user_cmd.phrases),
                    HashMap::new(),
                ));
            }
            "open_website" => {
                let url = user_command_url(user_cmd);
                if url.is_empty() {
                    continue;
                }

                let response = user_cmd.jarvis_line.clone();
                commands.push(JCommand::new(
                    user_cmd.id.clone(),
                    "open_website".into(),
                    command_description(user_cmd),
                    String::new(),
                    vec![],
                    url,
                    vec![],
                    response.clone(),
                    String::new(),
                    0,
                    sounds_for_response(&response, &lang),
                    phrase_map(&lang, &user_cmd.phrases),
                    HashMap::new(),
                ));
            }
            "volume_control" => {
                let response = user_cmd.jarvis_line.clone();
                let sounds = sounds_for_response(&response, &lang);

                if !user_cmd.volume_up_phrases.is_empty() {
                    commands.push(JCommand::new(
                        format!("{}__up", user_cmd.id),
                        "volume_change".into(),
                        command_description(user_cmd),
                        String::new(),
                        vec![user_cmd.volume_percent.to_string()],
                        "up".into(),
                        vec![],
                        response.clone(),
                        String::new(),
                        0,
                        sounds.clone(),
                        phrase_map(&lang, &user_cmd.volume_up_phrases),
                        HashMap::new(),
                    ));
                }

                if !user_cmd.volume_down_phrases.is_empty() {
                    commands.push(JCommand::new(
                        format!("{}__down", user_cmd.id),
                        "volume_change".into(),
                        command_description(user_cmd),
                        String::new(),
                        vec![user_cmd.volume_percent.to_string()],
                        "down".into(),
                        vec![],
                        response.clone(),
                        String::new(),
                        0,
                        sounds,
                        phrase_map(&lang, &user_cmd.volume_down_phrases),
                        HashMap::new(),
                    ));
                }
            }
            _ => {}
        }
    }

    vec![JCommandsList {
        path: storage_path,
        commands,
    }]
}

pub fn reload_commands_list() -> Vec<JCommandsList> {
    to_commands_list(&load())
}

pub fn launch_program(path: &str) -> Result<(), String> {
    let path = path.trim();
    if path.is_empty() {
        return Err("Путь к программе не задан.".into());
    }

    if !Path::new(path).exists() {
        return Err(format!("Файл не найден: {}", path));
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", path])
            .spawn()
            .map_err(|e| format!("Не удалось запустить программу: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::process::Command::new(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn normalize_website_url(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{}", trimmed)
    }
}

pub fn launch_website(url: &str) -> Result<(), String> {
    let url = normalize_website_url(url);
    if url.is_empty() {
        return Err("Адрес сайта не задан.".into());
    }

    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(format!("Некорректный адрес: {}", url));
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &url])
            .spawn()
            .map_err(|e| format!("Не удалось открыть сайт: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

