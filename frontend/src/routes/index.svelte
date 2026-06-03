<script lang="ts">
    import { onMount, onDestroy } from "svelte"
    import { invoke } from "@tauri-apps/api/core"

    import SearchBar from "@/components/elements/SearchBar.svelte"
    import ArcReactor from "@/components/elements/ArcReactor.svelte"
    import Stats from "@/components/elements/Stats.svelte"

    import {
        isJarvisRunning,
        updateJarvisStats,
        enableIpc,
        disableIpc,
        translate,
        translations,
        jarvisState,
        lastError,
        sendTextCommand
    } from "@/stores"
    import { get } from "svelte/store"
    import { LightningBolt } from "radix-icons-svelte"

    $: t = (key: string) => translate($translations, key)

    let processRunning = false
    let launching = false
    let startError = ""
    let wasRunning = false

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
        if (value) {
            enableIpc()
            wasRunning = true
        } else if (wasRunning) {
            disableIpc()
            wasRunning = false
        }
    })

    onMount(() => {
        updateJarvisStats()
    })

    onDestroy(() => {
        disableIpc()
    })

    async function runAssistant() {
        launching = true
        startError = ""
        try {
            await invoke("run_jarvis_app")
            setTimeout(async () => {
                await updateJarvisStats()
                launching = false
                if (!get(isJarvisRunning)) {
                    startError = "Ассистент не запустился. Проверьте логи."
                }
            }, 2500)
        } catch (err) {
            console.error("Failed to run jarvis-app:", err)
            startError = String(err)
            launching = false
        }
    }
</script>

<div class="minimal-layout">
    <!-- Top Navigation -->
    <nav class="top-nav">
        <div class="top-right-controls">
            {#if processRunning}
                <div class="stats-area">
                    <Stats />
                </div>
            {:else}
                <button class="power-btn" on:click={runAssistant} disabled={launching}>
                    <LightningBolt size={16} />
                    {launching ? t("btn-starting") : t("btn-start")}
                </button>
            {/if}
        </div>
    </nav>

    <!-- Main Centerpiece -->
    <main class="hero-center">
        <div class="reactor-showcase" class:offline={!processRunning}>
            <ArcReactor />
        </div>
        
        <div class="typography-section">
            <h1 class="main-title">{processRunning ? t(statusKey) : "JARVIS"}</h1>
            <p class="sub-title">{statusHint}</p>
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
        display: flex;
        justify-content: flex-end;
        align-items: center;
        padding: 2.5rem 2.5rem 1rem;
        z-index: 10;
    }

    .top-right-controls {
        display: flex;
        align-items: center;
        height: 100%;
    }

    .power-btn {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background: rgba(82, 254, 254, 0.05);
        border: 1px solid rgba(82, 254, 254, 0.2);
        color: #52fefe;
        padding: 0.5rem 1.25rem;
        border-radius: 100px;
        font-size: 0.85rem;
        font-weight: 600;
        text-transform: uppercase;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover:not(:disabled) {
            background: rgba(82, 254, 254, 0.15);
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(82, 254, 254, 0.1);
        }

        &:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }
    }

    .hero-center {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        z-index: 10;
        margin-top: 2vh;
    }

    .reactor-showcase {
        margin-bottom: 2.5rem;
        transition: all 0.5s ease;
        position: relative;

        &::after {
            content: '';
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            width: 160%;
            height: 160%;
            background: radial-gradient(circle, rgba(82, 254, 254, 0.15) 0%, transparent 60%);
            z-index: -1;
            pointer-events: none;
            opacity: 0;
            transition: opacity 1s ease;
        }

        &:not(.offline)::after {
            opacity: 1;
        }

        &.offline {
            opacity: 0.5;
            filter: grayscale(0.9) brightness(0.7);
            transform: scale(0.95);
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