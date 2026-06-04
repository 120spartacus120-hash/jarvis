import { writable, get } from "svelte/store"
import { invoke } from "@tauri-apps/api/core"
import { getCurrentWindow } from "@tauri-apps/api/window"

// ### IPC STORES ###

export type JarvisState = "disconnected" | "idle" | "listening" | "processing"

export const jarvisState = writable<JarvisState>("disconnected")
export const ipcConnected = writable(false)
export const lastRecognizedText = writable("")
export const lastExecutedCommand = writable("")
export const lastError = writable("")

// ### CONNECTION ###

const IPC_URL = "ws://127.0.0.1:9712"
const RECONNECT_DELAY = 5000

let ws: WebSocket | null = null
let reconnectTimer: ReturnType<typeof setTimeout> | null = null
let manualDisconnect = false
let enabled = false  // only connect when enabled

export function enableIpc() {
    enabled = true
    connectIpc()
}

export function disableIpc() {
    enabled = false
    disconnectIpc()
}

export function connectIpc(port: number = 9712) {
    if (ws?.readyState === WebSocket.OPEN) return

    ws = new WebSocket(`ws://127.0.0.1:${port}`)

    ws.onopen = () => {
        ipcConnected.set(true)
        jarvisState.set("idle")
        console.log("[IPC] connected")
    }

    ws.onclose = () => {
        ipcConnected.set(false)
        console.log("[IPC] disconnected")
    }

    ws.onerror = (err) => {
        console.error("[IPC] error:", err)
    }

    ws.onmessage = (event) => {
        try {
            const msg = JSON.parse(event.data)
            handleEvent(msg)
        } catch (e) {
            console.error("[IPC] failed to parse message:", e)
        }
    }
}

function scheduleReconnect() {
    if (reconnectTimer || manualDisconnect || !enabled) return

    console.log(`IPC: Will retry in ${RECONNECT_DELAY / 1000}s...`)
    reconnectTimer = setTimeout(() => {
        reconnectTimer = null
        connectIpc()
    }, RECONNECT_DELAY)
}

export function disconnectIpc() {
    manualDisconnect = true

    if (reconnectTimer) {
        clearTimeout(reconnectTimer)
        reconnectTimer = null
    }

    if (ws) {
        ws.close()
        ws = null
    }

    ipcConnected.set(false)
    jarvisState.set("disconnected")
}

// ### EVENT HANDLING ###

function handleEvent(data: any) {
    console.log("IPC: Event", data.event, data)

    switch (data.event) {
        case "wake_word_detected":
        case "listening":
            jarvisState.set("listening")
            break

        case "speech_recognized":
            lastRecognizedText.set(data.text || "")
            jarvisState.set("processing")
            break

        case "command_executed":
            lastExecutedCommand.set(data.id || "")
            break

        case "idle":
            jarvisState.set("idle")
            break

        case "error":
            lastError.set(data.message || "Unknown error")
            break

        case "started":
            jarvisState.set("idle")
            break

        case "stopping":
            jarvisState.set("disconnected")
            break

        case "quit_application":
            void quitEntireApplication()
            break

        case "pong":
            // connection verified
            break

        case "reveal_window":
            // bring window to foreground
            revealWindow()
            break
    }
}

// ### ACTIONS ###

export function sendAction(action: string, payload: Record<string, any> = {}) {
    if (ws?.readyState !== WebSocket.OPEN) {
        return false
    }

    ws.send(JSON.stringify({ action, ...payload }))
    return true
}

export function stopJarvisApp() {
    return sendAction("stop")
}

export function reloadCommands() {
    return sendAction("reload_commands")
}

/** Push settings.json into jarvis-app immediately (jarvis-app also watches the file). */
export async function reloadSettings(attempts = 6): Promise<boolean> {
    for (let i = 0; i < attempts; i++) {
        if (sendAction("reload_settings")) {
            return true
        }
        if (!get(ipcConnected) && i === 0) {
            enableIpc()
        }
        await new Promise((r) => setTimeout(r, 300))
    }
    console.warn("[IPC] reload_settings: assistant not connected (file watcher will apply soon)")
    return false
}

export function sendIpcMessage(message: object): Promise<void> {
    return new Promise((resolve, reject) => {
        if (!ws || ws.readyState !== WebSocket.OPEN) {
            reject(new Error("IPC not connected"))
            return
        }

        try {
            ws.send(JSON.stringify(message))
            resolve()
        } catch (err) {
            reject(err)
        }
    })
}

export function sendTextCommand(text: string): boolean {
    return sendAction("text_command", { text })
}

async function quitEntireApplication() {
    disableIpc()
    disconnectIpc()
    try {
        await invoke("quit_application")
    } catch (e) {
        console.error("[IPC] quit_application failed, closing window:", e)
        try {
            const window = getCurrentWindow()
            await window.close()
        } catch (closeErr) {
            console.error("[IPC] window.close failed:", closeErr)
        }
    }
}

async function revealWindow() {
    try {
        const window = getCurrentWindow()
        await window.show()
        await window.unminimize()
        await window.setFocus()
    } catch (e) {
        console.error("[IPC] Failed to reveal window:", e)
    }
}
