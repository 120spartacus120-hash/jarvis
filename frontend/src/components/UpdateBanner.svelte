<script lang="ts">
    import { onMount } from "svelte"
    import { translations, translate } from "@/stores"
    import {
        checkForAppUpdate,
        installPendingUpdate,
        type UpdateInfo,
    } from "@/lib/updater"

    $: t = (key: string, args?: Record<string, string>) =>
        translate($translations, key, args)

    let updateInfo: UpdateInfo | null = null
    let dismissed = false
    let installing = false
    let progress = 0
    let error = ""

    onMount(() => {
        void refresh()
    })

    async function refresh() {
        error = ""
        updateInfo = await checkForAppUpdate()
    }

    async function install() {
        if (!updateInfo || installing) return
        installing = true
        error = ""
        progress = 0
        try {
            await installPendingUpdate((p) => {
                progress = p
            })
        } catch (err) {
            console.error("[Updater] install failed:", err)
            error = String(err)
            installing = false
        }
    }

    function dismiss() {
        dismissed = true
    }
</script>

{#if updateInfo && !dismissed}
    <div class="update-banner" class:installing>
        <div class="update-text">
            <span class="update-title">
                {t("update-available", { version: updateInfo.version })}
            </span>
            {#if updateInfo.notes}
                <span class="update-notes">{updateInfo.notes}</span>
            {/if}
            {#if installing}
                <span class="update-progress">
                    {t("update-downloading", { percent: String(progress) })}
                </span>
            {/if}
            {#if error}
                <span class="update-error">{t("update-error")}: {error}</span>
            {/if}
        </div>
        <div class="update-actions">
            {#if !installing}
                <button class="btn-install" on:click={install}>
                    {t("update-install")}
                </button>
                <button class="btn-later" on:click={dismiss}>
                    {t("update-later")}
                </button>
            {/if}
        </div>
    </div>
{/if}

<style lang="scss">
    .update-banner {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 1rem;
        padding: 0.65rem 1.5rem;
        background: rgba(82, 254, 254, 0.08);
        border-bottom: 1px solid rgba(82, 254, 254, 0.2);
        flex-wrap: wrap;
    }

    .update-text {
        display: flex;
        flex-direction: column;
        gap: 0.15rem;
        min-width: 0;
    }

    .update-title {
        font-size: 0.85rem;
        font-weight: 600;
        color: #52fefe;
        letter-spacing: 0.03em;
    }

    .update-notes,
    .update-progress,
    .update-error {
        font-size: 0.75rem;
        color: rgba(255, 255, 255, 0.55);
    }

    .update-error {
        color: #fca5a5;
    }

    .update-actions {
        display: flex;
        gap: 0.5rem;
        flex-shrink: 0;
    }

    .btn-install {
        padding: 0.4rem 1rem;
        border-radius: 100px;
        border: 1px solid rgba(82, 254, 254, 0.45);
        background: rgba(82, 254, 254, 0.15);
        color: #52fefe;
        font-size: 0.8rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
            background: rgba(82, 254, 254, 0.25);
        }
    }

    .btn-later {
        padding: 0.4rem 0.85rem;
        border: none;
        background: transparent;
        color: rgba(255, 255, 255, 0.45);
        font-size: 0.8rem;
        cursor: pointer;

        &:hover {
            color: rgba(255, 255, 255, 0.8);
        }
    }
</style>
