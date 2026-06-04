use std::path::PathBuf;
use std::time::Duration;

use chrono::NaiveDate;
use serde::Deserialize;
use seqdiff::ratio;

use crate::DB;

const HTTP_TIMEOUT: Duration = Duration::from_secs(12);
/// User-recorded degree/wind clips are often quieter than intro phrases.
const WEATHER_CLIP_GAIN_DB: f32 = 12.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeatherDay {
    Today,
    Tomorrow,
}

struct KnownCity {
    /// Lowercase aliases (Russian + English id)
    names: &'static [&'static str],
    lat: f64,
    lon: f64,
    timezone: &'static str,
}

const KNOWN_CITIES: &[KnownCity] = &[
    KnownCity {
        names: &["москва", "moscow"],
        lat: 55.75204,
        lon: 37.61781,
        timezone: "Europe/Moscow",
    },
    KnownCity {
        names: &["челябинск", "chelyabinsk"],
        lat: 55.1611,
        lon: 61.42877,
        timezone: "Asia/Yekaterinburg",
    },
    KnownCity {
        names: &["якутск", "yakutsk"],
        lat: 62.03114,
        lon: 129.72289,
        timezone: "Asia/Yakutsk",
    },
];

fn weather_sound_file(name: &str) -> PathBuf {
    crate::SOUND_DIR.join("weather").join(name)
}

fn play_weather_sound(name: &str) -> bool {
    let path = weather_sound_file(name);
    if path.exists() {
        crate::audio::play_sound_blocking(&path);
        true
    } else {
        warn!("Weather sound missing: {}", path.display());
        false
    }
}

pub fn play_intro() {
    play_weather_sound("nachinayu_iskat.mp3");
}

pub fn play_day_intro(day: WeatherDay) {
    let clip = match day {
        WeatherDay::Today => "pogoda_na_segodnya.mp3",
        WeatherDay::Tomorrow => "pogoda_na_zavtra.mp3",
    };
    play_weather_sound(clip);
}

pub fn play_clip_blocking(path: &PathBuf) {
    crate::audio::play_sound_blocking_boosted(path, WEATHER_CLIP_GAIN_DB);
}

/// Which forecast to play — today/tomorrow are separate builtin commands.
pub fn day_for_command(cmd_id: &str, phrase: Option<&str>) -> WeatherDay {
    match cmd_id {
        "builtin_weather_tomorrow" => WeatherDay::Tomorrow,
        "builtin_weather" => WeatherDay::Today,
        _ => day_from_phrase(phrase),
    }
}

/// «погода завтра», «какая погода на завтра» — прогноз на следующий день.
pub fn day_from_phrase(phrase: Option<&str>) -> WeatherDay {
    let raw = phrase.unwrap_or("").trim();
    if raw.is_empty() {
        return WeatherDay::Today;
    }

    let text = crate::speech_normalize::normalize(&raw.to_lowercase());
    let day = resolve_weather_day(&text);
    info!(
        "Weather day: {:?} (recognized phrase: '{}', normalized: '{}')",
        day, raw, text
    );
    day
}

fn resolve_weather_day(text: &str) -> WeatherDay {
    if mentions_today(text) {
        return WeatherDay::Today;
    }
    if mentions_tomorrow(text) {
        return WeatherDay::Tomorrow;
    }

    if let Some(day) = day_from_configured_phrases(text) {
        return day;
    }

    WeatherDay::Today
}

fn mentions_today(text: &str) -> bool {
    text.split_whitespace().any(|w| {
        let t = trim_token(w);
        t == "сегодня" || t.starts_with("сегодн")
    })
}

pub fn is_tomorrow_request(text: &str) -> bool {
    let text = crate::speech_normalize::normalize(&text.trim().to_lowercase());
    mentions_tomorrow(&text)
}

fn mentions_tomorrow(text: &str) -> bool {
    if text.contains("послезавтра") {
        return false;
    }
    if text.contains("на завтра") {
        return true;
    }
    // Vosk иногда обрезает до «завт» / «завтраш»
    if text.contains("завт") && !text.contains("завтрак") {
        return true;
    }
    text.split_whitespace().any(|w| {
        let t = trim_token(w);
        t == "завтра" || matches!(t, "завтро" | "завтраа" | "завтрашний" | "завтрашняя")
    })
}

fn trim_token(word: &str) -> &str {
    word.trim_matches(|c: char| !c.is_alphanumeric())
}

fn day_from_configured_phrases(text: &str) -> Option<WeatherDay> {
    let phrases = crate::custom_commands::weather_phrases_for_day_detect();
    let text_chars: Vec<char> = text.chars().collect();

    let mut best_tomorrow = 0.0f64;
    let mut best_today = 0.0f64;

    for phrase in phrases {
        let p = phrase.trim().to_lowercase();
        if p.is_empty() {
            continue;
        }
        let p_chars: Vec<char> = p.chars().collect();
        let score = ratio(&text_chars, &p_chars);
        if p.contains("завтра") {
            best_tomorrow = best_tomorrow.max(score);
        } else if p.contains("сегодня") {
            best_today = best_today.max(score);
        }
    }

    const MIN_SCORE: f64 = 55.0;

    if best_tomorrow >= MIN_SCORE && best_tomorrow > best_today + 5.0 {
        return Some(WeatherDay::Tomorrow);
    }
    if best_today >= MIN_SCORE && best_today > best_tomorrow + 5.0 {
        return Some(WeatherDay::Today);
    }

    None
}

fn percent_encode_query(value: &str) -> String {
    let mut out = String::with_capacity(value.len() * 3);
    for &byte in value.as_bytes() {
        let unreserved = byte.is_ascii_alphanumeric() || matches!(byte, 45 | 95 | 46 | 126);
        if unreserved {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{:02X}", byte));
        }
    }
    out
}

fn http_client() -> Result<reqwest::blocking::Client, String> {
    reqwest::blocking::Client::builder()
        .timeout(HTTP_TIMEOUT)
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))
}

fn normalize_city_key(city: &str) -> String {
    city.trim().to_lowercase()
}

fn resolve_coords(city: &str) -> Result<(f64, f64, String), String> {
    let key = normalize_city_key(city);
    for known in KNOWN_CITIES {
        if known.names.iter().any(|name| *name == key.as_str()) {
            info!(
                "Weather: preset coords for '{}' -> {}, {} ({})",
                city, known.lat, known.lon, known.timezone
            );
            return Ok((known.lat, known.lon, known.timezone.to_string()));
        }
    }

    geocode_city(city)
}

fn geocode_city(city: &str) -> Result<(f64, f64, String), String> {
    let client = http_client()?;
    let city_q = percent_encode_query(city);
    let geo_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=5&language=ru&countryCode=RU&format=json",
        city_q
    );
    let geo_resp: GeocodingResult = client
        .get(&geo_url)
        .send()
        .map_err(|e| format!("Ошибка сети: {}", e))?
        .json()
        .map_err(|e| format!("Ошибка парсинга геоданных: {}", e))?;

    let location = geo_resp
        .results
        .and_then(|r| r.into_iter().next())
        .ok_or_else(|| format!("Город '{}' не найден", city))?;

    info!(
        "Weather: geocoded '{}' -> {} ({}, {})",
        city,
        location.name.as_deref().unwrap_or(city),
        location.latitude,
        location.longitude
    );

    Ok((location.latitude, location.longitude, "auto".to_string()))
}

#[derive(Deserialize, Debug)]
struct GeocodingResult {
    results: Option<Vec<GeocodingLocation>>,
}

#[derive(Deserialize, Debug)]
struct GeocodingLocation {
    name: Option<String>,
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize, Debug)]
struct ForecastResponse {
    current: Option<CurrentWeather>,
    daily: Option<DailyWeather>,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature_2m: f64,
    wind_speed_10m: f64,
    weather_code: u8,
}

#[derive(Deserialize, Debug)]
struct DailyWeather {
    time: Vec<String>,
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>,
    weather_code: Vec<u8>,
    wind_speed_10m_max: Vec<f64>,
}

fn tomorrow_daily_index(daily: &DailyWeather) -> Option<usize> {
    if daily.time.len() < 2 {
        return None;
    }

    let today = NaiveDate::parse_from_str(daily.time.first()?, "%Y-%m-%d").ok()?;
    let tomorrow = today.succ_opt()?;
    let tomorrow_str = tomorrow.format("%Y-%m-%d").to_string();

    daily
        .time
        .iter()
        .position(|d| d == &tomorrow_str)
        .or(Some(1))
}

struct WeatherSnapshot {
    temp_c: i32,
    wind_ms: f64,
    weather_code: u8,
}

pub fn get_weather(day: WeatherDay) -> Result<Vec<PathBuf>, String> {
    let city = {
        let db = DB.get().ok_or("DB not initialized")?;
        let settings = db.read();
        settings.weather_city.trim().to_string()
    };

    info!("Weather fetch {:?}, city from settings: '{}'", day, city);

    if city.is_empty() {
        return Err("Город не указан в настройках".to_string());
    }

    let (lat, lon, timezone) = resolve_coords(&city)?;
    let client = http_client()?;

    // `time` в daily= ломает API (400) — даты приходят в daily.time автоматически
    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}\
&current=temperature_2m,weather_code,wind_speed_10m\
&daily=temperature_2m_max,temperature_2m_min,weather_code,wind_speed_10m_max\
&wind_speed_unit=ms&timezone={tz}&forecast_days=2",
        lat = lat,
        lon = lon,
        tz = percent_encode_query(&timezone),
    );

    let http_resp = client
        .get(&weather_url)
        .send()
        .map_err(|e| format!("Ошибка сети: {}", e))?;

    if !http_resp.status().is_success() {
        let status = http_resp.status();
        let body = http_resp.text().unwrap_or_default();
        warn!("Open-Meteo HTTP {}: {}", status, body);
        return Err(format!("Сервис погоды недоступен ({})", status));
    }

    let weather_resp: ForecastResponse = http_resp
        .json()
        .map_err(|e| format!("Ошибка парсинга погоды: {}", e))?;

    let snapshot = match day {
        WeatherDay::Today => {
            let current = weather_resp
                .current
                .ok_or_else(|| "Нет текущих данных погоды".to_string())?;
            WeatherSnapshot {
                temp_c: current.temperature_2m.round() as i32,
                wind_ms: current.wind_speed_10m,
                weather_code: current.weather_code,
            }
        }
        WeatherDay::Tomorrow => {
            let daily = weather_resp
                .daily
                .ok_or_else(|| "Нет дневного прогноза".to_string())?;
            let idx = tomorrow_daily_index(&daily)
                .ok_or_else(|| "Прогноз на завтра недоступен".to_string())?;

            let max = *daily
                .temperature_2m_max
                .get(idx)
                .ok_or_else(|| "Нет температуры на завтра".to_string())?;
            let min = *daily
                .temperature_2m_min
                .get(idx)
                .ok_or_else(|| "Нет температуры на завтра".to_string())?;
            let code = daily.weather_code.get(idx).copied().unwrap_or(0);
            let wind = daily.wind_speed_10m_max.get(idx).copied().unwrap_or(0.0);

            info!(
                "Weather tomorrow index {} date {:?}: max={:.1} min={:.1}",
                idx,
                daily.time.get(idx),
                max,
                min
            );

            WeatherSnapshot {
                // Дневной прогноз: озвучиваем ожидаемый максимум (типично «днём до …»)
                temp_c: max.round() as i32,
                wind_ms: wind,
                weather_code: code,
            }
        }
    };

    info!(
        "Weather {} for '{}': {}°C, wind {:.1} m/s, WMO code {}",
        match day {
            WeatherDay::Today => "today",
            WeatherDay::Tomorrow => "tomorrow",
        },
        city,
        snapshot.temp_c,
        snapshot.wind_ms,
        snapshot.weather_code
    );

    build_playlist(snapshot)
}

fn build_playlist(snapshot: WeatherSnapshot) -> Result<Vec<PathBuf>, String> {
    let temp = snapshot.temp_c;
    let wind = snapshot.wind_ms;
    let code = snapshot.weather_code;

    let mut playlist = Vec::new();
    let sound_dir = crate::SOUND_DIR.join("weather");

    let temp_intro = sound_dir.join("temperatura.mp3");
    if temp_intro.exists() {
        playlist.push(temp_intro);
    }

    if (0..=30).contains(&temp) {
        let temp_file = sound_dir.join(format!("t_{}.mp3", temp));
        if temp_file.exists() {
            playlist.push(temp_file);
        }
    } else {
        warn!(
            "Weather: no audio clip for {}°C (only 0..30 supported)",
            temp
        );
    }

    let wind_file = if wind < 2.0 {
        "veter_net.mp3"
    } else if wind < 6.0 {
        "veter_slabiy.mp3"
    } else if wind < 12.0 {
        "veter_umerenny.mp3"
    } else {
        "veter_silniy.mp3"
    };
    let wind_path = sound_dir.join(wind_file);
    if wind_path.exists() {
        playlist.push(wind_path);
    }

    let precip_file = match code {
        0..=48 => "bez_osadkov.mp3",
        51..=67 | 80..=82 => "dozhd.mp3",
        71..=77 | 85..=86 => "sneg.mp3",
        95..=99 => "dozhd.mp3",
        _ => "bez_osadkov.mp3",
    };
    let precip_path = sound_dir.join(precip_file);
    if precip_path.exists() {
        playlist.push(precip_path);
    }

    Ok(playlist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_tomorrow_from_phrase() {
        assert_eq!(
            day_from_phrase(Some("погода на завтра")),
            WeatherDay::Tomorrow
        );
        assert_eq!(
            day_from_phrase(Some("погода на завтро")),
            WeatherDay::Tomorrow
        );
        assert_eq!(
            day_from_phrase(Some("погода на сегодня")),
            WeatherDay::Today
        );
        assert_eq!(day_from_phrase(Some("какая погода")), WeatherDay::Today);
    }
}
