use std::path::PathBuf;

use rand::seq::SliceRandom;

const GREETING_CLIPS: &[&str] = &[
    "greet1.mp3",
    "greet2.mp3",
    "greet3.mp3",
    "greet4.mp3",
    "greet5.mp3",
];

fn greeting_sound_file(name: &str) -> PathBuf {
    crate::SOUND_DIR.join("greeting").join(name)
}

/// Play a random bundled greeting clip (blocking until finished).
pub fn play_random_greeting() -> bool {
    let mut available: Vec<PathBuf> = GREETING_CLIPS
        .iter()
        .map(|name| greeting_sound_file(name))
        .filter(|path| path.exists())
        .collect();

    if available.is_empty() {
        warn!("No greeting sound files found in {}", greeting_sound_file("").parent().map(|p| p.display().to_string()).unwrap_or_default());
        crate::voices::play_greet();
        return false;
    }

    let mut rng = rand::thread_rng();
    let path = available.choose_mut(&mut rng).unwrap().clone();
    crate::audio::play_sound_blocking(&path);
    true
}
