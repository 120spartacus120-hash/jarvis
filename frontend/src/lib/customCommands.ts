import { invoke } from "@tauri-apps/api/core"
import { get } from "svelte/store"
import { isJarvisRunning, reloadCommands } from "@/stores"

export type CommandType = "open_program" | "open_website"

export interface UserCommand {
    id: string
    name: string
    type: CommandType
    program_path: string
    website_url: string
    phrases: string[]
    user_line: string
    jarvis_line: string
}

export interface CustomCommandsConfig {
    thanks_phrases: string[]
    shutdown_phrases: string[]
    weather_phrases: string[]
    user_commands: UserCommand[]
}

export async function loadCustomCommands(): Promise<CustomCommandsConfig> {
    return invoke<CustomCommandsConfig>("get_custom_commands")
}

export async function saveCustomCommands(config: CustomCommandsConfig): Promise<void> {
    await invoke("save_custom_commands", { config })
    if (get(isJarvisRunning)) {
        reloadCommands()
    }
}

export function normalizePhrases(phrases: string[]): string[] {
    return phrases.map((p) => p.trim().toLowerCase()).filter(Boolean)
}

export function normalizeWebsiteUrl(url: string): string {
    const trimmed = url.trim()
    if (!trimmed) return ""
    const lower = trimmed.toLowerCase()
    if (lower.startsWith("http://") || lower.startsWith("https://")) {
        return trimmed
    }
    return `https://${trimmed}`
}

export function isValidWebsiteUrl(url: string): boolean {
    const normalized = normalizeWebsiteUrl(url)
    if (!normalized) return false
    try {
        const parsed = new URL(normalized)
        return parsed.protocol === "http:" || parsed.protocol === "https:"
    } catch {
        return false
    }
}

export function newCommandId(): string {
    return crypto.randomUUID()
}

export const COMMAND_TYPES = [
    { value: "open_program", labelKey: "commands-type-open-program" },
    { value: "open_website", labelKey: "commands-type-open-website" },
] as const

export async function getCustomSoundsDir(): Promise<string> {
    return invoke<string>("get_custom_sounds_dir")
}

export async function importCustomSound(path: string): Promise<string> {
    return invoke<string>("import_custom_sound", { path })
}
