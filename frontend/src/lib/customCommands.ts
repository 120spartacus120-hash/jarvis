import { invoke } from "@tauri-apps/api/core"
import { get } from "svelte/store"
import { isJarvisRunning, reloadCommands } from "@/stores"

export type CommandType = "open_program" | "open_website" | "volume_control"

export interface UserCommand {
    id: string
    name: string
    type: CommandType
    program_path: string
    website_url: string
    phrases: string[]
    user_line: string
    jarvis_line: string
    volume_percent?: number
    /** @deprecated migrated to volume_percent */
    volume_up_steps?: number
    /** @deprecated migrated to volume_percent */
    volume_down_steps?: number
    volume_up_phrases?: string[]
    volume_down_phrases?: string[]
}

export interface CustomCommandsConfig {
    thanks_phrases: string[]
    shutdown_phrases: string[]
    weather_phrases: string[]
    user_commands: UserCommand[]
}

export const DEFAULT_VOLUME_UP_PHRASES = [
    "прибавь звук",
    "сделай громче",
    "громче",
    "увеличь громкость",
    "прибавь громкость",
    "сделай погромче",
    "добавь громкость",
    "звук громче",
]

export const DEFAULT_VOLUME_DOWN_PHRASES = [
    "убавь звук",
    "сделай потише",
    "тише",
    "уменьши громкость",
    "убавь громкость",
    "сделай потише звук",
    "звук тише",
]

export async function loadCustomCommands(): Promise<CustomCommandsConfig> {
    return invoke<CustomCommandsConfig>("get_custom_commands")
}

export async function saveCustomCommands(
    partial: Partial<CustomCommandsConfig>
): Promise<void> {
    const current = await loadCustomCommands()
    const config: CustomCommandsConfig = { ...current, ...partial }
    await invoke("save_custom_commands", { config })
    if (get(isJarvisRunning)) {
        reloadCommands()
    }
}

export function normalizeVolumePercent(percent: number, fallback = 2): number {
    let n = Math.floor(Number(percent))
    if (!Number.isFinite(n) || n < 2) n = fallback
    n = Math.min(100, n)
    if (n % 2 !== 0) n += 1
    return Math.min(100, n)
}

export function resolveVolumePercent(cmd: Pick<UserCommand, "volume_percent" | "volume_up_steps" | "volume_down_steps">): number {
    if (cmd.volume_percent != null && cmd.volume_percent > 0) {
        return normalizeVolumePercent(cmd.volume_percent)
    }
    const legacy = Math.max(cmd.volume_up_steps ?? 0, cmd.volume_down_steps ?? 0)
    return legacy > 0 ? normalizeVolumePercent(legacy) : 2
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
    { value: "volume_control", labelKey: "commands-type-volume-control" },
] as const

export async function getCustomSoundsDir(): Promise<string> {
    return invoke<string>("get_custom_sounds_dir")
}

export async function importCustomSound(path: string): Promise<string> {
    return invoke<string>("import_custom_sound", { path })
}
