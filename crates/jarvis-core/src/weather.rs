use std::path::PathBuf;
use std::time::Duration;

use serde::Deserialize;

use crate::DB;

const HTTP_TIMEOUT: Duration = Duration::from_secs(12);
/// User-recorded degree/wind clips are often quieter than intro phrases.
const WEATHER_CLIP_GAIN_DB: f32 = 12.0;

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

pub fn play_today_intro() {
    play_weather_sound("pogoda_na_segodnya.mp3");
}

pub fn play_clip_blocking(path: &PathBuf) {
    crate::audio::play_sound_blocking_boosted(path, WEATHER_CLIP_GAIN_DB);
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

#[derive(Deserialize, Debug)]
struct GeocodingResult {
    results: Option<Vec<GeocodingLocation>>,
}

#[derive(Deserialize, Debug)]
struct GeocodingLocation {
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature_2m: f64,
    wind_speed_10m: f64,
    weather_code: u8,
}

pub fn get_weather() -> Result<Vec<PathBuf>, String> {
    let city = {
        let db = DB.get().ok_or("DB not initialized")?;
        let settings = db.read();
        settings.weather_city.trim().to_string()
    };

    if city.is_empty() {
        return Err("Город не указан в настройках".to_string());
    }

    let client = http_client()?;

    // 1. Geocoding
    let city_q = percent_encode_query(&city);
    let geo_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=ru&format=json",
        city_q
    );
    let geo_resp: GeocodingResult = client
        .get(&geo_url)
        .send()
        .map_err(|e| format!("Ошибка сети: {}", e))?
        .json()
        .map_err(|e| format!("Ошибка парсинга геоданных: {}", e))?;

    let location = geo_resp.results
        .and_then(|mut r| r.pop())
        .ok_or_else(|| format!("Город '{}' не найден", city))?;

    // 2. Weather
    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,weather_code,wind_speed_10m&wind_speed_unit=ms",
        location.latitude, location.longitude
    );

    let weather_resp: WeatherResponse = client
        .get(&weather_url)
        .send()
        .map_err(|e| format!("Ошибка сети: {}", e))?
        .json()
        .map_err(|e| format!("Ошибка парсинга погоды: {}", e))?;

    let current = weather_resp.current;
    
    let temp = current.temperature_2m.round() as i32;
    let wind = current.wind_speed_10m;
    let code = current.weather_code;

    // 3. Build playlist (pogoda_na_segodnya plays in commands.rs right after fetch)
    let mut playlist = Vec::new();
    let sound_dir = crate::SOUND_DIR.join("weather");

    // Temperature
    let temp_intro = sound_dir.join("temperatura.mp3");
    if temp_intro.exists() {
        playlist.push(temp_intro);
    }

    // We only have files for 0 to 30 degrees
    if temp >= 0 && temp <= 30 {
        let temp_file = sound_dir.join(format!("t_{}.mp3", temp));
        if temp_file.exists() {
            playlist.push(temp_file);
        }
    }

    // Wind
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

    // Precipitation (WMO Weather interpretation codes)
    // 0: Clear sky
    // 1, 2, 3: Mainly clear, partly cloudy, and overcast
    // 45, 48: Fog and depositing rime fog
    // 51, 53, 55: Drizzle: Light, moderate, and dense intensity
    // 56, 57: Freezing Drizzle: Light and dense intensity
    // 61, 63, 65: Rain: Slight, moderate and heavy intensity
    // 66, 67: Freezing Rain: Light and heavy intensity
    // 71, 73, 75: Snow fall: Slight, moderate, and heavy intensity
    // 77: Snow grains
    // 80, 81, 82: Rain showers: Slight, moderate, and violent
    // 85, 86: Snow showers slight and heavy
    // 95: Thunderstorm: Slight or moderate
    // 96, 99: Thunderstorm with slight and heavy hail
    let precip_file = match code {
        0..=48 => "bez_osadkov.mp3",
        51..=67 | 80..=82 => "dozhd.mp3",
        71..=77 | 85..=86 => "sneg.mp3",
        95..=99 => "dozhd.mp3", // fallback thunderstorm to rain
        _ => "bez_osadkov.mp3"
    };
    let precip_path = sound_dir.join(precip_file);
    if precip_path.exists() {
        playlist.push(precip_path);
    }

    Ok(playlist)
}
