import { check, type Update } from "@tauri-apps/plugin-updater"
import { relaunch } from "@tauri-apps/plugin-process"
import { invoke } from "@tauri-apps/api/core"
import { stopJarvisApp, disableIpc, disconnectIpc } from "@/lib/ipc"

export type UpdateInfo = {
    version: string
    notes: string
    date: string
}

let pendingUpdate: Update | null = null

export async function checkForAppUpdate(): Promise<UpdateInfo | null> {
    try {
        const update = await check()
        if (!update) {
            pendingUpdate = null
            return null
        }
        pendingUpdate = update
        return {
            version: update.version,
            notes: update.body ?? "",
            date: update.date ?? "",
        }
    } catch (err) {
        console.error("[Updater] check failed:", err)
        return null
    }
}

export async function installPendingUpdate(
    onProgress?: (percent: number) => void
): Promise<void> {
    if (!pendingUpdate) {
        throw new Error("No pending update")
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
    let downloaded = 0
    let total = 0

    await update.downloadAndInstall((event) => {
        if (event.event === "Started") {
            total = event.data.contentLength ?? 0
            onProgress?.(0)
        } else if (event.event === "Progress") {
            downloaded += event.data.chunkLength
            if (total > 0) {
                onProgress?.(Math.min(100, Math.round((downloaded / total) * 100)))
            }
        } else if (event.event === "Finished") {
            onProgress?.(100)
        }
    })

    pendingUpdate = null
    await relaunch()
}

export async function openReleasesPage(): Promise<void> {
    const link = await invoke<string>("get_repository_link").catch(
        () => "https://github.com/120spartacus120-hash/jarvis/releases"
    )
    const { open } = await import("@tauri-apps/plugin-shell")
    await open(link + "/releases")
}
