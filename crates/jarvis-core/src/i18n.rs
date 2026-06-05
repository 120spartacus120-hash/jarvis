use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use fluent_bundle::concurrent::FluentBundle as ConcurrentFluentBundle;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use serde::Serialize;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

const LOCALE_RU: &str = include_str!("i18n/locales/ru.ftl");
const LOCALE_EN: &str = include_str!("i18n/locales/en.ftl");
const LOCALE_UA: &str = include_str!("i18n/locales/ua.ftl");
const LOCALE_DE: &str = include_str!("i18n/locales/de.ftl");
const LOCALE_FR: &str = include_str!("i18n/locales/fr.ftl");
const LOCALE_ES: &str = include_str!("i18n/locales/es.ftl");

pub const SUPPORTED_LANGUAGES: &[&str] = &["ru", "en", "ua", "de", "fr", "es"];
pub const DEFAULT_LANGUAGE: &str = "ru";

pub const LANGUAGE_LABELS: &[(&str, &str)] = &[
    ("ru", "Русский"),
    ("en", "English"),
    ("ua", "Українська"),
    ("de", "Deutsch"),
    ("fr", "Français"),
    ("es", "Español"),
];

#[derive(Serialize, Clone)]
pub struct LanguageOption {
    pub code: String,
    pub name: String,
}

pub fn language_options() -> Vec<LanguageOption> {
    LANGUAGE_LABELS
        .iter()
        .map(|(code, name)| LanguageOption {
            code: (*code).to_string(),
            name: (*name).to_string(),
        })
        .collect()
}

pub fn detect_system_language() -> &'static str {
    let lang = sys_locale::get_locale()
        .and_then(|locale| locale.split(&['-', '_'][..]).next().map(str::to_lowercase))
        .unwrap_or_else(|| DEFAULT_LANGUAGE.to_string());

    match lang.as_str() {
        "ru" => "ru",
        "en" => "en",
        "uk" | "ua" => "ua",
        "de" => "de",
        "fr" => "fr",
        "es" => "es",
        _ => DEFAULT_LANGUAGE,
    }
}

type Bundle = ConcurrentFluentBundle<FluentResource>;

static BUNDLES: OnceCell<HashMap<String, Bundle>> = OnceCell::new();
static CURRENT_LANG: OnceCell<RwLock<String>> = OnceCell::new();

pub fn init(lang: &str) {
    let bundles = create_bundles();
    BUNDLES.set(bundles).ok();

    let lang = normalize_language(lang);
    CURRENT_LANG.set(RwLock::new(lang.clone())).ok();

    info!("i18n initialized with language: {}", lang);
}

fn normalize_language(lang: &str) -> String {
    if SUPPORTED_LANGUAGES.contains(&lang) {
        lang.to_string()
    } else {
        DEFAULT_LANGUAGE.to_string()
    }
}

fn locale_source(lang: &str) -> &'static str {
    match lang {
        "ru" => LOCALE_RU,
        "en" => LOCALE_EN,
        "ua" => LOCALE_UA,
        "de" => LOCALE_DE,
        "fr" => LOCALE_FR,
        "es" => LOCALE_ES,
        _ => LOCALE_EN,
    }
}

fn bcp47_tag(lang: &str) -> &str {
    match lang {
        "ua" => "uk",
        _ => lang,
    }
}

fn create_bundles() -> HashMap<String, Bundle> {
    let mut bundles = HashMap::new();
    for lang in SUPPORTED_LANGUAGES {
        bundles.insert(lang.to_string(), create_bundle(lang, locale_source(lang)));
    }
    bundles
}

fn create_bundle(lang_code: &str, source: &str) -> Bundle {
    let langid: LanguageIdentifier = bcp47_tag(lang_code)
        .parse()
        .unwrap_or_else(|_| "en".parse().unwrap());
    let mut bundle = ConcurrentFluentBundle::new_concurrent(vec![langid]);

    let resource = FluentResource::try_new(source.to_string())
        .unwrap_or_else(|e| panic!("Failed to parse FTL for {lang_code}: {e:?}"));

    bundle
        .add_resource(resource)
        .unwrap_or_else(|e| panic!("Failed to add FTL resource for {lang_code}: {e:?}"));
    bundle
}

pub fn set_language(lang: &str) {
    if let Some(current) = CURRENT_LANG.get() {
        let lang = normalize_language(lang);
        *current.write() = lang.clone();
        info!("Language changed to: {}", lang);
    }
}

pub fn get_language() -> String {
    CURRENT_LANG
        .get()
        .map(|l| l.read().clone())
        .unwrap_or_else(|| DEFAULT_LANGUAGE.to_string())
}

pub fn t(key: &str) -> String {
    t_with_args(key, None)
}

pub fn t_with_args(key: &str, args: Option<&FluentArgs>) -> String {
    let lang = get_language();

    let bundles = match BUNDLES.get() {
        Some(b) => b,
        None => return key.to_string(),
    };

    let bundle = match bundles.get(&lang) {
        Some(b) => b,
        None => bundles.get(DEFAULT_LANGUAGE).unwrap(),
    };

    let msg = match bundle.get_message(key) {
        Some(m) => m,
        None => return key.to_string(),
    };

    let pattern = match msg.value() {
        Some(p) => p,
        None => return key.to_string(),
    };

    let mut errors = vec![];
    let result = bundle.format_pattern(pattern, args, &mut errors);

    if !errors.is_empty() {
        warn!("i18n errors for key '{}': {:?}", key, errors);
    }

    result.to_string()
}

pub fn t_arg(key: &str, arg_name: &str, arg_value: &str) -> String {
    let mut args = FluentArgs::new();
    args.set(arg_name, FluentValue::from(arg_value));
    t_with_args(key, Some(&args))
}

pub fn t_count(key: &str, count: i64) -> String {
    let mut args = FluentArgs::new();
    args.set("count", FluentValue::from(count));
    t_with_args(key, Some(&args))
}

pub fn get_all_translations() -> HashMap<String, String> {
    get_translations_for(&get_language())
}

pub fn get_translations_for(lang: &str) -> HashMap<String, String> {
    let lang = normalize_language(lang);
    let mut result = extract_translations_from_bundle("en");
    if lang != "en" {
        result.extend(extract_translations_from_bundle(&lang));
    }
    result
}

fn extract_translations_from_bundle(lang: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    let bundles = match BUNDLES.get() {
        Some(b) => b,
        None => return result,
    };

    let bundle = match bundles.get(lang) {
        Some(b) => b,
        None => return result,
    };

    let source = locale_source(lang);

    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('-') {
            continue;
        }

        if let Some(key) = line.split('=').next() {
            let key = key.trim();
            if !key.is_empty() && !key.contains(' ') {
                if let Some(msg) = bundle.get_message(key) {
                    if let Some(pattern) = msg.value() {
                        let mut errors = vec![];
                        let value = bundle.format_pattern(pattern, None, &mut errors);
                        result.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }
    }

    result
}
