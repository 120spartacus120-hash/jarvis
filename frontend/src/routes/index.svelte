<script lang="ts">
    import { onMount, onDestroy } from "svelte"
    import { invoke } from "@tauri-apps/api/core"

    import SearchBar from "@/components/elements/SearchBar.svelte"
    import ArcReactor from "@/components/elements/ArcReactor.svelte"
    import Stats from "@/components/elements/Stats.svelte"

    import {
        isJarvisRunning,
        updateJarvisStats,
        translate,
        translations,
        jarvisState,
        lastError,
        sendTextCommand
    } from "@/stores"
    import { get } from "svelte/store"

    $: t = (key: string) => translate($translations, key)

    const ACTIVATION_MS = 3000

    let processRunning = false
    let activating = false
    let launching = false
    let startError = ""
    $: statusKey =
        $jarvisState === "listening"
            ? "status-listening"
            : $jarvisState === "processing"
              ? "status-processing"
              : $jarvisState === "idle"
                ? "status-standby"
                : "status-disconnected"

    $: statusHint = processRunning
        ? t("assist-running-hint")
        : t("assistant-offline-hint")

    isJarvisRunning.subscribe((value) => {
        processRunning = value
    })

    onMount(() => {
        updateJarvisStats()
    })

    async function runAssistant() {
        if (activating || processRunning) return

        activating = true
        launching = true
        startError = ""

        const startedAt = Date.now()

        try {
            await invoke("run_jarvis_app")

            const elapsed = Date.now() - startedAt
            const remaining = Math.max(0, ACTIVATION_MS - elapsed)
            if (remaining > 0) {
                await new Promise((resolve) => setTimeout(resolve, remaining))
            }

            await updateJarvisStats()

            if (!get(isJarvisRunning)) {
                activating = false
                startError = "Ассистент не запустился. Проверьте логи."
            } else {
                await new Promise((resolve) => setTimeout(resolve, 300))
            }
        } catch (err) {
            console.error("Failed to run jarvis-app:", err)
            startError = String(err)
            activating = false
        } finally {
            launching = false
            if (get(isJarvisRunning)) {
                activating = false
            }
        }
    }
</script>

<div class="minimal-layout">
    <nav class="top-nav" class:visible={processRunning && !activating}>
        {#if processRunning && !activating}
            <div class="stats-area">
                <Stats />
            </div>
        {/if}
    </nav>

    <main class="hero-center">
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div 
            class="reactor-showcase" 
            class:clickable={!processRunning && !activating}
            on:click={() => {
                if (!processRunning && !activating) runAssistant();
            }}
        >
            <ArcReactor {activating} online={processRunning} />
        </div>

        <div class="typography-section">
            <h1 class="main-title">{processRunning && !activating ? t(statusKey) : "JARVIS"}</h1>
            <p class="sub-title">{activating ? t("btn-starting") : statusHint}</p>

            {#if startError || $lastError}
                <p class="error-text">{startError || $lastError}</p>
            {/if}
        </div>
    </main>

    <!-- Bottom Controls -->
    <footer class="bottom-dock">
        <div class="search-area">
            <SearchBar />
        </div>
    </footer>
</div>

<style lang="scss">
    .minimal-layout {
        height: calc(100vh - 80px);
        width: 100%;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        background: #05070a;
        color: #ffffff;
        font-family: 'Inter', system-ui, -apple-system, sans-serif;
        overflow: hidden;
        position: relative;
    }

    /* Subtle background glow */
    .minimal-layout::before {
        content: '';
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 60vw;
        height: 60vw;
        background: radial-gradient(circle, rgba(82, 254, 254, 0.03) 0%, transparent 70%);
        pointer-events: none;
        z-index: 0;
    }

    .top-nav {
        position: absolute;
        top: 0;
        right: 0;
        left: 0;
        display: flex;
        justify-content: flex-end;
        align-items: center;
        padding: 1.5rem 2.5rem 0;
        z-index: 20;
        opacity: 0;
        pointer-events: none;
        transition: opacity 0.8s ease;

        &.visible {
            opacity: 1;
            pointer-events: auto;
        }
    }

    .hero-center {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        z-index: 10;
        min-height: 0;
    }

    .reactor-showcase {
        margin-bottom: 2.5rem;
        position: relative;
        border-radius: 50%;
        width: 300px;
        height: 300px;
        display: flex;
        align-items: center;
        justify-content: center;

        &.clickable {
            cursor: pointer;
        }
    }

    .typography-section {
        text-align: center;

        .main-title {
            font-size: 2.5rem;
            font-weight: 200;
            letter-spacing: 0.2em;
            margin: 0;
            text-transform: uppercase;
            background: linear-gradient(180deg, #ffffff 0%, rgba(255, 255, 255, 0.4) 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            line-height: 1.3;
            padding: 0.1em 0;
        }

        .sub-title {
            margin: 0.75rem 0 0;
            font-size: 0.9rem;
            color: rgba(255, 255, 255, 0.3);
            font-weight: 400;
            letter-spacing: 0.5px;
        }

        .error-text {
            margin-top: 1rem;
            color: #ef4444;
            font-size: 0.85rem;
            background: rgba(239, 68, 68, 0.1);
            padding: 0.5rem 1rem;
            border-radius: 8px;
            display: inline-block;
        }
    }

    .bottom-dock {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 2rem 2rem 3rem;
        z-index: 10;
    }

    .search-area {
        width: 100%;
        max-width: 600px;
    }

    .stats-area {
        opacity: 0.5;
        transition: all 0.3s ease;

        &:hover {
            opacity: 1;
        }
    }
</style>