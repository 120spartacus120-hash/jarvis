<script lang="ts">
    import { onMount } from "svelte"
    import { goto } from "@roxi/routify"
    import { open } from "@tauri-apps/plugin-dialog"

    import HDivider from "@/components/elements/HDivider.svelte"
    import Footer from "@/components/Footer.svelte"
    import CommandTypePicker from "@/components/commands/CommandTypePicker.svelte"
    import { translations, translate } from "@/stores"
    import {
        loadCustomCommands,
        saveCustomCommands,
        normalizePhrases,
        normalizeWebsiteUrl,
        isValidWebsiteUrl,
        newCommandId,
        normalizeVolumePercent,
        resolveVolumePercent,
        DEFAULT_VOLUME_UP_PHRASES,
        DEFAULT_VOLUME_DOWN_PHRASES,
        type CustomCommandsConfig,
        type CommandType,
        type UserCommand,
    } from "@/lib/customCommands"

    import { Button, Input, Alert } from "@svelteuidev/core"
    import { Plus, Trash, ArrowLeft } from "radix-icons-svelte"

    export let mode: "create" | "edit" = "create"
    export let commandId = ""

    $: t = (key: string, args?: Record<string, string | number>) => translate($translations, key, args)
    $: isEdit = mode === "edit"
    $: pageTitle = isEdit ? t("commands-edit-title") : t("commands-new-title")
    $: typeSelected = commandType !== ""
    $: formDesc = isEdit || typeSelected ? t("commands-new-desc") : t("commands-new-desc-pick")

    let config: CustomCommandsConfig | null = null
    let commandType: CommandType | "" = ""
    let name = ""
    let programPath = ""
    let websiteUrl = ""
    let phrases: string[] = []
    let upPhrases: string[] = []
    let downPhrases: string[] = []
    let volumePercent = 2
    let newPhrase = ""
    let newUpPhrase = ""
    let newDownPhrase = ""
    let loading = true
    let saving = false
    let saved = false
    let loadError = ""
    let saveError = ""
    let loadedKey = ""
    let previousType = ""

    $: if (!isEdit && commandType && commandType !== previousType) {
        if (previousType) {
            name = ""
            programPath = ""
            websiteUrl = ""
            phrases = []
            newPhrase = ""
            newUpPhrase = ""
            newDownPhrase = ""
        }
        upPhrases = []
        downPhrases = []
        volumePercent = 2
        if (commandType === "volume_control") {
            upPhrases = [...DEFAULT_VOLUME_UP_PHRASES]
            downPhrases = [...DEFAULT_VOLUME_DOWN_PHRASES]
        }
        previousType = commandType
    }

    $: volumePercentPreview = normalizeVolumePercent(volumePercent)

    async function loadForm() {
        const key = `${mode}:${commandId}`
        if (key === loadedKey && config) return
        if (isEdit && !commandId) return

        loading = true
        loadError = ""
        try {
            config = await loadCustomCommands()

            if (isEdit) {
                const existing = config.user_commands.find((cmd) => cmd.id === commandId)
                if (
                    !existing ||
                    (existing.type !== "open_program" &&
                        existing.type !== "open_website" &&
                        existing.type !== "volume_control")
                ) {
                    loadError = t("error-not-found")
                    return
                }
                commandType = existing.type
                name = existing.name
                programPath = existing.program_path ?? ""
                websiteUrl =
                    existing.website_url?.trim() ||
                    (existing.type === "open_website" ? existing.program_path ?? "" : "")
                phrases = [...(existing.phrases ?? [])]
                upPhrases = [...(existing.volume_up_phrases ?? [])]
                downPhrases = [...(existing.volume_down_phrases ?? [])]
                volumePercent = resolveVolumePercent(existing)
            } else {
                commandType = ""
                name = ""
                programPath = ""
                websiteUrl = ""
                phrases = []
                upPhrases = []
                downPhrases = []
                volumePercent = 2
            }

            loadedKey = key
        } catch (err) {
            console.error("failed to load command form:", err)
            loadError = t("error-generic")
        } finally {
            loading = false
        }
    }

    $: if (isEdit ? commandId : true) {
        loadForm()
    }

    async function saveForm() {
        if (!config) return

        saveError = ""
        if (!commandType) {
            saveError = t("commands-type-required")
            return
        }

        const trimmedName = name.trim()
        if (!trimmedName) {
            saveError = t("commands-name-required")
            return
        }

        if (commandType === "open_program" && !programPath.trim()) {
            saveError = t("commands-program-required")
            return
        }

        if (commandType === "open_website") {
            if (!websiteUrl.trim()) {
                saveError = t("commands-website-required")
                return
            }
            if (!isValidWebsiteUrl(websiteUrl)) {
                saveError = t("commands-website-invalid")
                return
            }
        }

        if (commandType === "volume_control") {
            const normalizedUp = normalizePhrases(upPhrases)
            const normalizedDown = normalizePhrases(downPhrases)
            if (normalizedUp.length === 0 && normalizedDown.length === 0) {
                saveError = t("commands-volume-phrases-required")
                return
            }
        }

        const normalizedPhrases = normalizePhrases(phrases)

        const userCommand: UserCommand = {
            id: isEdit ? commandId : newCommandId(),
            name: trimmedName,
            type: commandType,
            program_path: commandType === "open_program" ? programPath.trim() : "",
            website_url:
                commandType === "open_website" ? normalizeWebsiteUrl(websiteUrl) : "",
            phrases: commandType === "volume_control" ? [] : normalizedPhrases,
            user_line: "",
            jarvis_line: "",
            volume_percent:
                commandType === "volume_control" ? normalizeVolumePercent(volumePercent) : undefined,
            volume_up_phrases:
                commandType === "volume_control" ? normalizePhrases(upPhrases) : undefined,
            volume_down_phrases:
                commandType === "volume_control" ? normalizePhrases(downPhrases) : undefined,
        }

        saving = true
        saved = false
        try {
            let userCommands = config.user_commands.filter((cmd) =>
                isEdit ? cmd.id !== commandId : true
            )
            userCommands = [...userCommands, userCommand]

            await saveCustomCommands({
                thanks_phrases: config.thanks_phrases,
                shutdown_phrases: config.shutdown_phrases,
                weather_phrases: config.weather_phrases ?? [],
                greeting_phrases: config.greeting_phrases ?? [],
                user_commands: userCommands,
            })

            saved = true
            setTimeout(() => {
                $goto("/commands")
            }, 600)
        } catch (err) {
            console.error("failed to save command:", err)
            saveError = t("error-generic")
        } finally {
            saving = false
        }
    }

    async function deleteCommand() {
        if (!config || !isEdit) return
        if (!confirm(t("commands-delete-confirm"))) return

        saving = true
        try {
            await saveCustomCommands({
                thanks_phrases: config.thanks_phrases,
                shutdown_phrases: config.shutdown_phrases,
                weather_phrases: config.weather_phrases ?? [],
                greeting_phrases: config.greeting_phrases ?? [],
                user_commands: config.user_commands.filter((cmd) => cmd.id !== commandId),
            })
            $goto("/commands")
        } catch (err) {
            console.error("failed to delete command:", err)
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

    function addVolumePhrase(list: "up" | "down") {
        if (list === "up") {
            const phrase = newUpPhrase.trim().toLowerCase()
            if (!phrase || upPhrases.includes(phrase)) {
                newUpPhrase = ""
                return
            }
            upPhrases = [...upPhrases, phrase]
            newUpPhrase = ""
            return
        }
        const phrase = newDownPhrase.trim().toLowerCase()
        if (!phrase || downPhrases.includes(phrase)) {
            newDownPhrase = ""
            return
        }
        downPhrases = [...downPhrases, phrase]
        newDownPhrase = ""
    }

    function removeVolumePhrase(list: "up" | "down", index: number) {
        if (list === "up") {
            upPhrases = upPhrases.filter((_, i) => i !== index)
        } else {
            downPhrases = downPhrases.filter((_, i) => i !== index)
        }
    }

    function handleUpPhraseKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            event.preventDefault()
            addVolumePhrase("up")
        }
    }

    function handleDownPhraseKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            event.preventDefault()
            addVolumePhrase("down")
        }
    }

    async function pickProgramFile() {
        const selected = await open({
            multiple: false,
            filters: [
                {
                    name: "Program",
                    extensions: ["lnk", "exe", "bat", "cmd"],
                },
            ],
        })

        if (typeof selected === "string") {
            programPath = selected
        }
    }

    onMount(loadForm)
</script>

<div class="form-page">
    <button type="button" class="back-btn" on:click={() => $goto("/commands")}>
        <ArrowLeft size={14} />
        {t("commands-back")}
    </button>

    <header class="form-header">
        <h1>{pageTitle}</h1>
        <p>{formDesc}</p>
    </header>

    {#if loading}
        <p class="hint">{t("stats-loading")}</p>
    {:else if loadError}
        <Alert color="red" title={loadError} />
    {:else}
        {#if saved}
            <Alert color="green" title={t("commands-saved")} />
        {/if}
        {#if saveError}
            <Alert color="red" title={saveError} />
        {/if}

        <section class="form-section">
            <div class="section-label">
                <span class="step">1</span>
                <span>{t("commands-type-label")}</span>
            </div>
            <CommandTypePicker
                bind:value={commandType}
                placeholder={t("commands-type-placeholder")}
                label={t("commands-type-label")}
                description={t("commands-type-desc")}
                disabled={isEdit}
                {t}
            />
        </section>

        {#if isEdit || typeSelected}
            <div class="command-fields">
                <section class="form-section">
                    <div class="section-label">
                        <span class="step">2</span>
                        <span>{t("commands-name-label")}</span>
                    </div>
                    <Input
                        id="command-name"
                        placeholder={t("commands-name-placeholder")}
                        bind:value={name}
                    />
                </section>

                {#if commandType === "open_program"}
                    <section class="form-section">
                        <div class="section-label">
                            <span class="step">3</span>
                            <span>{t("commands-program-path")}</span>
                        </div>
                        <p class="section-hint">{t("commands-program-path-hint")}</p>
                        <div class="path-row">
                            <Input
                                id="program-path"
                                placeholder="C:\Program Files\...\program.exe"
                                bind:value={programPath}
                            />
                            <Button class="pick-btn" on:click={pickProgramFile}>{t("commands-program-pick")}</Button>
                        </div>
                    </section>
                {:else if commandType === "open_website"}
                    <section class="form-section">
                        <div class="section-label">
                            <span class="step">3</span>
                            <span>{t("commands-website-url")}</span>
                        </div>
                        <p class="section-hint">{t("commands-website-url-hint")}</p>
                        <Input
                            id="website-url"
                            type="url"
                            placeholder="https://yandex.ru"
                            bind:value={websiteUrl}
                        />
                    </section>
                {:else if commandType === "volume_control"}
                    <section class="form-section">
                        <div class="section-label">
                            <span class="step">3</span>
                            <span>{t("commands-volume-steps-title")}</span>
                        </div>
                        <p class="section-hint">{t("commands-volume-steps-hint")}</p>
                        <div class="volume-percent-row">
                            <label class="percent-field">
                                <span>{t("commands-volume-percent-label")}</span>
                                <Input
                                    type="number"
                                    min={2}
                                    max={100}
                                    step={2}
                                    bind:value={volumePercent}
                                />
                            </label>
                            <div class="volume-preview" aria-live="polite">
                                <span class="preview-title">{t("commands-volume-preview-title")}</span>
                                <div class="preview-lines">
                                    <span class="preview-up">
                                        {t("commands-volume-preview-up", { percent: volumePercentPreview })}
                                    </span>
                                    <span class="preview-down">
                                        {t("commands-volume-preview-down", { percent: volumePercentPreview })}
                                    </span>
                                </div>
                            </div>
                        </div>
                    </section>
                {/if}

                {#if commandType === "volume_control"}
                    <section class="form-section">
                        <div class="section-label">
                            <span class="step">4</span>
                            <span>{t("commands-volume-up-phrases-title")}</span>
                        </div>
                        <p class="section-hint">{t("commands-volume-up-hint")}</p>
                        <div class="phrase-add" role="group" on:keydown={handleUpPhraseKeydown}>
                            <Input
                                placeholder={t("commands-phrase-placeholder-volume-up")}
                                bind:value={newUpPhrase}
                            />
                            <Button on:click={() => addVolumePhrase("up")} disabled={!newUpPhrase.trim()}>
                                <Plus size={14} />
                                {t("commands-add-phrase")}
                            </Button>
                        </div>
                        {#if upPhrases.length === 0}
                            <p class="empty-phrases">{t("commands-no-phrases")}</p>
                        {:else}
                            <ul class="phrase-list">
                                {#each upPhrases as phrase, index}
                                    <li>
                                        <span>{phrase}</span>
                                        <button
                                            type="button"
                                            class="remove-btn"
                                            on:click={() => removeVolumePhrase("up", index)}
                                            title={t("commands-remove-phrase")}
                                        >
                                            <Trash size={14} />
                                        </button>
                                    </li>
                                {/each}
                            </ul>
                        {/if}
                    </section>

                    <section class="form-section">
                        <div class="section-label">
                            <span class="step">5</span>
                            <span>{t("commands-volume-down-phrases-title")}</span>
                        </div>
                        <p class="section-hint">{t("commands-volume-down-hint")}</p>
                        <div class="phrase-add" role="group" on:keydown={handleDownPhraseKeydown}>
                            <Input
                                placeholder={t("commands-phrase-placeholder-volume-down")}
                                bind:value={newDownPhrase}
                            />
                            <Button on:click={() => addVolumePhrase("down")} disabled={!newDownPhrase.trim()}>
                                <Plus size={14} />
                                {t("commands-add-phrase")}
                            </Button>
                        </div>
                        {#if downPhrases.length === 0}
                            <p class="empty-phrases">{t("commands-no-phrases")}</p>
                        {:else}
                            <ul class="phrase-list">
                                {#each downPhrases as phrase, index}
                                    <li>
                                        <span>{phrase}</span>
                                        <button
                                            type="button"
                                            class="remove-btn"
                                            on:click={() => removeVolumePhrase("down", index)}
                                            title={t("commands-remove-phrase")}
                                        >
                                            <Trash size={14} />
                                        </button>
                                    </li>
                                {/each}
                            </ul>
                        {/if}
                    </section>
                {/if}

                {#if commandType === "open_program" || commandType === "open_website"}
                    <section class="form-section">
                        <div class="section-label">
                            <span class="step">4</span>
                            <span>{t("commands-phrases-title")}</span>
                        </div>
                        <p class="section-hint">{t("commands-hint")}</p>

                        <div class="phrase-add" role="group" on:keydown={handlePhraseKeydown}>
                            <Input
                                placeholder={commandType === "open_website"
                                    ? t("commands-phrase-placeholder-website")
                                    : t("commands-phrase-placeholder-custom")}
                                bind:value={newPhrase}
                            />
                            <Button on:click={addPhrase} disabled={!newPhrase.trim()}>
                                <Plus size={14} />
                                {t("commands-add-phrase")}
                            </Button>
                        </div>

                        {#if phrases.length === 0}
                            <p class="empty-phrases">{t("commands-phrases-optional")}</p>
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
                {/if}

                <div class="actions">
                    <Button class="save-btn" on:click={saveForm} loading={saving}>
                        {t("settings-save")}
                    </Button>
                    {#if isEdit}
                        <Button class="delete-btn" color="red" variant="outline" on:click={deleteCommand} disabled={saving}>
                            <Trash size={14} />
                            {t("commands-delete")}
                        </Button>
                    {/if}
                </div>
            </div>
        {/if}
    {/if}
</div>

<HDivider />
<Footer />

<style lang="scss">
    .form-page {
        padding: 8px 2px 12px;
        max-width: 520px;
        margin: 0 auto;
    }

    .back-btn {
        display: inline-flex;
        align-items: center;
        gap: 0.35rem;
        margin-bottom: 0.85rem;
        padding: 0.35rem 0.55rem;
        border: none;
        background: transparent;
        color: rgba(255, 255, 255, 0.65);
        font-size: 0.78rem;
        cursor: pointer;

        &:hover {
            color: #52fefe;
        }
    }

    .form-header {
        text-align: center;
        margin-bottom: 1.25rem;

        h1 {
            margin: 0 0 0.45rem;
            font-size: 1.45rem;
            font-weight: 700;
            color: #fff;
            letter-spacing: 0.01em;
        }

        p {
            margin: 0;
            font-size: 0.78rem;
            color: rgba(255, 255, 255, 0.45);
            line-height: 1.45;
        }
    }

    .form-section {
        margin-bottom: 1rem;
        padding: 0.85rem 0.9rem;
        border-radius: 12px;
        border: 1px solid rgba(255, 255, 255, 0.08);
        background: rgba(18, 28, 34, 0.72);
    }

    .section-label {
        display: flex;
        align-items: center;
        gap: 0.55rem;
        margin-bottom: 0.65rem;
        font-size: 0.82rem;
        font-weight: 600;
        color: rgba(255, 255, 255, 0.9);
    }

    .step {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 1.35rem;
        height: 1.35rem;
        border-radius: 999px;
        background: rgba(82, 254, 254, 0.15);
        color: #52fefe;
        font-size: 0.72rem;
        font-weight: 700;
        flex-shrink: 0;
    }

    .section-hint {
        margin: -0.25rem 0 0.65rem;
        font-size: 0.72rem;
        color: rgba(255, 255, 255, 0.42);
        line-height: 1.4;
    }

    .path-row {
        display: flex;
        flex-direction: column;
        gap: 0.55rem;
    }

    .volume-percent-row {
        display: flex;
        flex-wrap: wrap;
        align-items: stretch;
        gap: 1rem;
        margin-top: 0.25rem;
    }

    .percent-field {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        min-width: 120px;
        flex: 0 0 140px;
        font-size: 0.85rem;
        color: rgba(255, 255, 255, 0.75);
    }

    .volume-preview {
        flex: 1;
        min-width: 180px;
        padding: 0.75rem 0.9rem;
        border-radius: 10px;
        border: 1px solid rgba(82, 254, 254, 0.2);
        background: rgba(8, 16, 20, 0.65);
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .preview-title {
        font-size: 0.72rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.04em;
        color: rgba(255, 255, 255, 0.45);
    }

    .preview-lines {
        display: flex;
        flex-direction: column;
        gap: 0.35rem;
        font-size: 0.95rem;
        font-weight: 600;
    }

    .preview-up {
        color: #8ac832;
    }

    .preview-down {
        color: #52b4fe;
    }

    :global(.pick-btn) {
        align-self: flex-start;
    }

    .phrase-add {
        display: flex;
        flex-direction: column;
        gap: 0.55rem;
    }

    .phrase-list {
        list-style: none;
        margin: 0.75rem 0 0;
        padding: 0;
        display: flex;
        flex-direction: column;
        gap: 0.45rem;
        max-height: 220px;
        overflow-y: auto;

        li {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 0.5rem;
            padding: 0.55rem 0.75rem;
            border-radius: 8px;
            background: rgba(10, 18, 22, 0.85);
            border: 1px solid rgba(255, 255, 255, 0.06);
            color: rgba(255, 255, 255, 0.88);
            font-size: 0.8rem;
        }
    }

    .remove-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        border: none;
        background: transparent;
        color: rgba(255, 120, 120, 0.85);
        cursor: pointer;
        padding: 0.15rem;
    }

    .actions {
        display: flex;
        flex-direction: column;
        gap: 0.55rem;
        margin-top: 1.25rem;
        padding-top: 0.25rem;
    }

    :global(.save-btn),
    :global(.delete-btn) {
        width: 100%;
    }

    .empty-phrases,
    .hint {
        text-align: center;
        color: rgba(255, 255, 255, 0.45);
        font-size: 0.78rem;
        margin: 0.65rem 0 0;
    }

    .command-fields {
        animation: fields-in 0.25s ease;
    }

    @keyframes fields-in {
        from {
            opacity: 0;
            transform: translateY(6px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
</style>
