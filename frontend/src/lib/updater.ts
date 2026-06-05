import { check, type Update } from "@tauri-apps/plugin-updater"
import { relaunch } from "@tauri-apps/plugin-process"
import { invoke } from "@tauri-apps/api/core"
import { get, writable } from "svelte/store"
import { stopJarvisApp, disableIpc, disconnectIpc } from "@/lib/ipc"

export type UpdateInfo = {
    version: string
    notes: string
    date: string
}

export type UpdateDownloadState = "idle" | "downloading" | "ready" | "failed"

export const updateDownloadState = writable<UpdateDownloadState>("idle")
export const updateReadyInfo = writable<UpdateInfo | null>(null)

let pendingUpdate: Update | null = null
let backgroundFlowRunning = false

/** Check for updates and download silently; notify UI only when the package is ready. */
export async function startBackgroundUpdateFlow(): Promise<void> {
    if (backgroundFlowRunning) return
    if (get(updateDownloadState) === "ready" && get(updateReadyInfo)) return

    backgroundFlowRunning = true
    updateDownloadState.set("idle")
    updateReadyInfo.set(null)

    try {
        const update = await check()
        if (!update) {
            pendingUpdate = null
            return
        }

        pendingUpdate = update
        updateDownloadState.set("downloading")

        await update.download((event) => {
            if (event.event === "Progress") {
                /* background only — no UI */
            }
        })

        updateDownloadState.set("ready")
        updateReadyInfo.set({
            version: update.version,
            notes: update.body ?? "",
            date: update.date ?? "",
        })
    } catch (err) {
        console.error("[Updater] background download failed:", err)
        pendingUpdate = null
        updateDownloadState.set("failed")
        updateReadyInfo.set(null)
    } finally {
        backgroundFlowRunning = false
    }
}

export async function installPendingUpdate(): Promise<void> {
    if (!pendingUpdate) {
        throw new Error("No pending update")
    }
    if (get(updateDownloadState) !== "ready") {
        throw new Error("Update is not downloaded yet")
    }

    disableIpc()
    disconnectIpc()
    try {
        stopJarvisApp()
    } catch {
        /* assistant may already be stopped */
    }
    await new Promise((r) => setTimeout(r, 800))

    const update = pendingUpdate
    await update.install()

    pendingUpdate = null
    updateDownloadState.set("idle")
    updateReadyInfo.set(null)
    await relaunch()
}

export async function openReleasesPage(): Promise<void> {
    const link = await invoke<string>("get_repository_link").catch(
        () => "https://github.com/120spartacus120-hash/jarvis/releases"
    )
    const { open } = await import("@tauri-apps/plugin-shell")
    await open(link + "/releases")
}
