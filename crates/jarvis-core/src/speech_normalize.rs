//! Fixes common Vosk/STT misrecognitions before command matching.

/// Normalize recognized speech for command matching (lowercase expected).
pub fn normalize(text: &str) -> String {
    text.split_whitespace()
        .map(normalize_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_token(word: &str) -> String {
    let w = word.trim_matches(|c: char| {
        !c.is_alphanumeric() && c != '+' && c != '-'
    });

    let fixed = match w {
        // YouTube — Vosk often inserts ь: «ютьюб», «ютюб»
        "ютьюб" | "ютюб" | "ютуба" | "ютубе" | "youtube" | "youtub" => "ютуб",

        // Other frequent sites
        "вконтакт" | "вконтакте" | "vk" | "вк" => "вконтакте",
        "телеграмм" | "телеге" | "телегу" => "телеграм",
        "инстаграмм" | "инсте" => "инстаграм",
        "гугл" | "гугле" | "google" => "гугл",
        "яндексе" => "яндекс",
        "дискорде" => "дискорд",
        "стиме" => "стим",
        "спотифай" => "спотифи",

        _ => w,
    };

    if fixed == w {
        word.to_string()
    } else {
        word.replacen(w, fixed, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_youtube_typos() {
        assert_eq!(normalize("открой ютьюб"), "открой ютуб");
        assert_eq!(normalize("открой ютюб"), "открой ютуб");
    }
}
