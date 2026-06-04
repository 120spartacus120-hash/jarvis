<script lang="ts">
    import { onMount } from "svelte"
    import { goto } from "@roxi/routify"
    import { invoke } from "@tauri-apps/api/core"

    import HDivider from "@/components/elements/HDivider.svelte"
    import Footer from "@/components/Footer.svelte"
    import { translations, translate } from "@/stores"
    import { reloadSettings } from "@/lib/ipc"
    import { loadCustomCommands, saveCustomCommands, normalizePhrases } from "@/lib/customCommands"
    import { resolveWeatherCity, WEATHER_CITIES } from "@/lib/weatherCities"

    import { Button, Alert, NativeSelect } from "@svelteuidev/core"
    import { Plus, Trash, ArrowLeft } from "radix-icons-svelte"

    $: t = (key: string, args?: Record<string, string | number>) => translate($translations, key, args)

    let phrases: string[] = []
    let thanksPhrases: string[] = []
    let shutdownPhrases: string[] = []
    let userCommands: import("@/lib/customCommands").UserCommand[] = []
    let weatherCity = ""
    let newPhrase = ""
    let loading = true
    let saving = false
    let saved = false
    let loadError = ""
    let saveError = ""

    async function loadConfig() {
        loading = true
        loadError = ""
        try {
            const [config, city] = await Promise.all([
                loadCustomCommands(),
                invoke<string>("db_read", { key: "weather_city" }).catch(() => ""),
            ])
            phrases = [...(config.weather_phrases ?? [])]
            thanksPhrases = [...(config.thanks_phrases ?? [])]
            shutdownPhrases = [...(config.shutdown_phrases ?? [])]
            userCommands = [...(config.user_commands ?? [])]
            weatherCity = resolveWeatherCity(city)
        } catch (err) {
            console.error("failed to load weather command:", err)
            loadError = t("error-generic")
        } finally {
            loading = false
        }
    }

    async function saveConfig() {
        const city = resolveWeatherCity(weatherCity)
        weatherCity = city

        saving = true
        saved = false
        saveError = ""
        try {
            const ok = await invoke<boolean>("db_write", { key: "weather_city", val: city })
            if (!ok) {
                saveError = t("error-generic")
                return
            }

            await reloadSettings()

            await saveCustomCommands({
                thanks_phrases: thanksPhrases,
                shutdown_phrases: shutdownPhrases,
                weather_phrases: normalizePhrases(phrases),
                user_commands: userCommands,
            })

            saved = true
            setTimeout(() => {
                saved = false
            }, 3000)
        } catch (err) {
            console.error("failed to save weather command:", err)
            saveError = t("error-generic")
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
        <h1>{t("commands-card-weather")}</h1>
        <p>{t("commands-weather-desc")}</p>
    </header>

    {#if loading}
        <p class="hint">{t("stats-loading")}</p>
    {:else}
        {#if saved || loadError || saveError}
            <div class="alerts">
                {#if saved}
                    <Alert color="green" title={t("commands-saved")} />
                {/if}
                {#if loadError}
                    <Alert color="red" title={loadError} />
                {/if}
                {#if saveError}
                    <Alert color="red" title={saveError} />
                {/if}
            </div>
        {/if}

        <section class="form-section">
            <div class="section-label">
                <span class="step">1</span>
                <span>{t("commands-weather-city-label")}</span>
            </div>
            <p class="section-hint">{t("commands-weather-city-hint")}</p>
            <NativeSelect
                data={WEATHER_CITIES.map((c) => ({ label: c.label, value: c.value }))}
                label={t("commands-weather-city-label")}
                variant="filled"
                bind:value={weatherCity}
            />
        </section>

        <section class="form-section">
            <div class="section-label">
                <span class="step">2</span>
                <span>{t("commands-phrases-title")}</span>
            </div>
            <p class="section-hint">{t("commands-hint")}</p>

            <div class="phrase-add" role="group" on:keydown={handlePhraseKeydown}>
                <input
                    class="phrase-input"
                    placeholder={t("commands-phrase-placeholder-weather")}
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

    .phrase-input {
        flex: 1;
        padding: 0.65rem 0.75rem;
        border-radius: 8px;
        border: 1px solid rgba(255, 255, 255, 0.12);
        background: rgba(8, 16, 20, 0.85);
        color: #fff;
        font-size: 0.85rem;
    }
</style>
