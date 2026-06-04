<script lang="ts">
    import { onMount, onDestroy } from "svelte"
    import { Container } from "@svelteuidev/core"
    import Header from "@/components/Header.svelte"
    import UpdateBanner from "@/components/UpdateBanner.svelte"
    import {
        isJarvisRunning,
        enableIpc,
        disableIpc,
        updateJarvisStats,
        startStatsPolling,
        stopStatsPolling,
    } from "@/stores"

    let wasRunning = false

    const unsubRunning = isJarvisRunning.subscribe((running) => {
        if (running) {
            enableIpc()
            wasRunning = true
        } else if (wasRunning) {
            disableIpc()
            wasRunning = false
        }
    })

    onMount(() => {
        startStatsPolling()
        void updateJarvisStats()
    })

    onDestroy(() => {
        unsubRunning()
        stopStatsPolling()
        disableIpc()
    })
</script>

<Container fluid id="wrapper">
    <Header />
    <UpdateBanner />
    <main>
        <slot></slot>
    </main>
</Container>
