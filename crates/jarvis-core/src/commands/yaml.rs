use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::JCommand;

#[derive(Deserialize)]
struct YamlCommandsFile {
    list: Vec<YamlCommandEntry>,
}

#[derive(Deserialize)]
struct YamlCommandEntry {
    command: YamlCommandDef,
    #[serde(default)]
    voice: YamlVoice,
    #[serde(default)]
    phrases: Vec<String>,
}

#[derive(Deserialize)]
struct YamlCommandDef {
    action: String,
    #[serde(default)]
    exe_path: String,
    #[serde(default)]
    exe_args: Vec<String>,
    #[serde(default)]
    cli_cmd: String,
    #[serde(default)]
    cli_args: Vec<String>,
}

#[derive(Default, Deserialize)]
struct YamlVoice {
    #[serde(default)]
    sounds: Vec<String>,
}

pub fn load_yaml_commands(cmd_path: &Path) -> Option<Vec<JCommand>> {
    let yaml_file = cmd_path.join("command.yaml");
    if !yaml_file.exists() {
        return None;
    }

    let content = match std::fs::read_to_string(&yaml_file) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to read {}: {}", yaml_file.display(), e);
            return None;
        }
    };

    let file: YamlCommandsFile = match serde_yaml::from_str(&content) {
        Ok(f) => f,
        Err(e) => {
            warn!("Failed to parse {}: {}", yaml_file.display(), e);
            return None;
        }
    };

    let pack = cmd_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("command");

    let mut used_ids = HashMap::new();
    let mut commands = Vec::with_capacity(file.list.len());

    for (index, entry) in file.list.into_iter().enumerate() {
        let id = unique_yaml_id(pack, index, &entry, &mut used_ids);
        let mut phrases_map = HashMap::new();
        if !entry.phrases.is_empty() {
            phrases_map.insert("ru".to_string(), entry.phrases);
        }

        let mut sounds_map = HashMap::new();
        if !entry.voice.sounds.is_empty() {
            sounds_map.insert("ru".to_string(), entry.voice.sounds);
        }

        commands.push(JCommand::new(
            id,
            entry.command.action,
            String::new(),
            entry.command.exe_path,
            entry.command.exe_args,
            entry.command.cli_cmd,
            entry.command.cli_args,
            String::new(),
            String::new(),
            0,
            sounds_map,
            phrases_map,
            HashMap::new(),
        ));
    }

    if commands.is_empty() {
        warn!("No commands found in {}", yaml_file.display());
        None
    } else {
        Some(commands)
    }
}

fn unique_yaml_id(
    pack: &str,
    index: usize,
    entry: &YamlCommandEntry,
    used_ids: &mut HashMap<String, usize>,
) -> String {
    let base = yaml_id_base(pack, index, entry);
    let count = used_ids.entry(base.clone()).or_insert(0);
    *count += 1;

    if *count == 1 {
        base
    } else {
        format!("{}_{}", base, *count - 1)
    }
}

fn yaml_id_base(pack: &str, index: usize, entry: &YamlCommandEntry) -> String {
    if !entry.command.exe_path.is_empty() {
        let stem = Path::new(&entry.command.exe_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("cmd");
        return format!("{}_{}", pack, slug(stem));
    }

    if !entry.command.cli_cmd.is_empty() {
        return format!("{}_{}", pack, slug(&entry.command.cli_cmd));
    }

    format!("{}_{}_{}", pack, entry.command.action, index)
}

fn slug(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}
