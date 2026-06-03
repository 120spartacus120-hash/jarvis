<script lang="ts">
    import { onMount } from "svelte"
    import { goto } from "@roxi/routify"

    import HDivider from "@/components/elements/HDivider.svelte"
    import Footer from "@/components/Footer.svelte"
    import { translations, translate } from "@/stores"
    import { loadCustomCommands, saveCustomCommands, normalizePhrases } from "@/lib/customCommands"

    import { Button, Input, Alert } from "@svelteuidev/core"
    import { Plus, Trash, ArrowLeft } from "radix-icons-svelte"

    $: t = (key: string, args?: Record<string, string | number>) => translate($translations, key, args)

    let phrases: string[] = []
    let thanksPhrases: string[] = []
    let weatherPhrases: string[] = []
    let userCommands: import("@/lib/customCommands").UserCommand[] = []
    let newPhrase = ""
    let loading = true
    let saving = false
    let saved = false
    let loadError = ""

    async function loadConfig() {
        loading = true
        loadError = ""
        try {
            const config = await loadCustomCommands()
            phrases = [...(config.shutdown_phrases ?? [])]
            thanksPhrases = [...(config.thanks_phrases ?? [])]
            weatherPhrases = [...(config.weather_phrases ?? [])]
            userCommands = [...(config.user_commands ?? [])]
        } catch (err) {
            console.error("failed to load custom commands:", err)
            loadError = t("error-generic")
        } finally {
            loading = false
        }
    }

    async function saveConfig() {
        saving = true
        saved = false
        try {
            await saveCustomCommands({
                thanks_phrases: thanksPhrases,
                shutdown_phrases: normalizePhrases(phrases),
                weather_phrases: weatherPhrases,
                user_commands: userCommands,
            })
            saved = true
            setTimeout(() => {
                saved = false
            }, 3000)
        } catch (err) {
            console.error("failed to save custom commands:", err)
            loadError = t("error-generic")
        } finally {
            saving = false
        }
    }

    function addPhrase() {
        const phrase = newPhrase.trim().toLowerCase()
        if (!phrase || phrases.includes(phrase)) {
            newPhrase = ""
            return
        }
        phrases = [...phrases, phrase]
        newPhrase = ""
    }

    function removePhrase(index: number) {
        phrases = phrases.filter((_, i) => i !== index)
    }

    function handlePhraseKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            event.preventDefault()
            addPhrase()
        }
    }

    onMount(loadConfig)
</script>

<div class="form-page">
    <button type="button" class="back-btn" on:click={() => $goto("/commands")}>
        <ArrowLeft size={14} />
        {t("commands-back")}
    </button>

    <header class="form-header">
        <h1>{t("commands-card-shutdown")}</h1>
        <p>{t("commands-shutdown-desc")}</p>
    </header>

    {#if loading}
        <p class="hint">{t("stats-loading")}</p>
    {:else}
        {#if saved || loadError}
            <div class="alerts">
                {#if saved}
                    <Alert color="green" title={t("commands-saved")} />
                {/if}
                {#if loadError}
                    <Alert color="red" title={loadError} />
                {/if}
            </div>
        {/if}

        <section class="form-section">
            <div class="section-label">
                <span class="step">1</span>
                <span>{t("commands-phrases-title")}</span>
            </div>
            <p class="section-hint">{t("commands-shutdown-hint")}</p>

            <div class="phrase-add" role="group" on:keydown={handlePhraseKeydown}>
                <Input
                    placeholder={t("commands-phrase-placeholder-shutdown")}
                    bind:value={newPhrase}
                />
                <Button on:click={addPhrase} disabled={!newPhrase.trim()}>
                    <Plus size={14} />
                    {t("commands-add-phrase")}
                </Button>
            </div>

            {#if phrases.length === 0}
                <p class="empty-phrases">{t("commands-no-phrases")}</p>
            {:else}
                <ul class="phrase-list">
                    {#each phrases as phrase, index}
                        <li>
                            <span>{phrase}</span>
                            <button type="button" class="remove-btn" on:click={() => removePhrase(index)} title={t("commands-remove-phrase")}>
                                <Trash size={14} />
                            </button>
                        </li>
                    {/each}
                </ul>
            {/if}
        </section>

        <div class="actions">
            <Button class="save-btn" on:click={saveConfig} loading={saving}>
                {t("settings-save")}
            </Button>
        </div>
    {/if}
</div>

<HDivider />
<Footer />

<style lang="scss">
    @use "../../../css/builtin-command-page.scss";
</style>
