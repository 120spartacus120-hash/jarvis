<script lang="ts">
    import { onMount } from "svelte"
    import { invoke } from "@tauri-apps/api/core"
    import { goto } from "@roxi/routify"
    import { setTimeout } from "worker-timers"

    import { assistantVoice, translations, translate } from "@/stores"

    import {
        Notification,
        Button,
        Text,
        Space,
        Alert,
        Input,
        InputWrapper,
        NativeSelect,
        Switch
    } from "@svelteuidev/core"

    import {
        Check,
        Mix,
        Cube,
        Code,
        Gear,
        CrossCircled,
        Globe
    } from "radix-icons-svelte"

    $: t = (key: string) => translate($translations, key)

    interface VoiceMeta {
        id: string
        name: string
        author: string
        languages: string[]
    }

    interface VoiceConfig {
        voice: VoiceMeta
    }
    
    let availableVoices: VoiceMeta[] = []

    async function selectVoice(voiceId: string) {
        voiceVal = voiceId
        
        // play preview sound
        try {
            await invoke("preview_voice", { voiceId })
        } catch (err) {
            console.error("Failed to preview voice:", err)
        }
    }

    // ### STATE
    interface MicrophoneOption {
        label: string
        value: string
    }

    let availableMicrophones: MicrophoneOption[] = []
    let availableVoskModels: { label: string; value: string }[] = []
    let availableGlinerModels: { label: string; value: string }[] = []
    let settingsSaved = false
    let saveButtonDisabled = false

    // form values (state vars)
    let voiceVal = ""
    let selectedMicrophone = ""
    let selectedWakeWordEngine = ""
    let selectedIntentRecognitionEngine = ""
    let selectedSlotExtractionEngine = ""
    let selectedGlinerModel = ""
    let selectedVoskModel = ""
    let selectedNoiseSuppression = ""
    let selectedVad = ""
    let gainNormalizerEnabled = false
    let windowsAutostart = false
    let apiKeyPicovoice = ""
    let apiKeyOpenai = ""
    let weatherCity = ""

    // tabs state
    let activeTab = 'general'

    // subscribe to stores
    assistantVoice.subscribe(value => {
        voiceVal = value
    })

    // ### FUNCTIONS
    async function saveSettings() {
        saveButtonDisabled = true
        settingsSaved = false

        try {
            await Promise.all([
                invoke("db_write", { key: "assistant_voice", val: voiceVal }),
                invoke("db_write", { key: "selected_microphone", val: selectedMicrophone }),
                invoke("db_write", { key: "selected_wake_word_engine", val: selectedWakeWordEngine }),
                invoke("db_write", { key: "selected_intent_recognition_engine", val: selectedIntentRecognitionEngine }),
                invoke("db_write", { key: "selected_slot_extraction_engine", val: selectedSlotExtractionEngine }),
                invoke("db_write", { key: "selected_gliner_model", val: selectedGlinerModel }),
                invoke("db_write", { key: "selected_vosk_model", val: selectedVoskModel }),

                invoke("db_write", { key: "noise_suppression", val: selectedNoiseSuppression }),
                invoke("db_write", { key: "vad", val: selectedVad }),
                invoke("db_write", { key: "gain_normalizer", val: gainNormalizerEnabled.toString() }),

                invoke("set_windows_autostart", { enabled: windowsAutostart }),

                invoke("db_write", { key: "api_key__picovoice", val: apiKeyPicovoice }),
                invoke("db_write", { key: "api_key__openai", val: apiKeyOpenai }),
                invoke("db_write", { key: "weather_city", val: weatherCity })
            ])

            // update shared store
            assistantVoice.set(voiceVal)
            settingsSaved = true

            // hide alert after 5 seconds
            setTimeout(() => {
                settingsSaved = false
            }, 5000)

        } catch (err) {
            console.error("failed to save settings:", err)
        }

        setTimeout(() => {
            saveButtonDisabled = false
        }, 1000)
    }

    // ### INIT
    onMount(async () => {
        // load voices
        try {
            const voices = await invoke<VoiceConfig[]>("list_voices")
            availableVoices = voices.map(v => v.voice)
        } catch (err) {
            console.error("Failed to load voices:", err)
            availableVoices = []
        }

        try {
            // load microphones
            const mics = await invoke<string[]>("pv_get_audio_devices")
            availableMicrophones = [
                { label: t('settings-mic-default'), value: "-1" },  // system default
                ...mics.map((name, idx) => ({
                    label: name,
                    value: String(idx)
                }))
            ]

            // load vosk models
            const languageNames: Record<string, string> = {
                us: 'English',
                ru: 'Русский',
                uk: 'Українська',
                de: 'German',
                fr: 'French',
                es: 'Spanish',
            };
            const voskModels = await invoke<{ name: string; language: string; size: string }[]>("list_vosk_models")
            availableVoskModels = voskModels.map(m => ({
                label: `${m.name} (${languageNames[m.language] ?? m.language}, ${m.size})`,
                value: m.name
            }))

            // load gliner models
            const glinerModels = await invoke<{ display_name: string; value: string }[]>("list_gliner_models")
            availableGlinerModels = glinerModels.map(m => ({
                label: m.display_name,
                value: m.value,
            }))

            // load settings from db
            const [mic, wakeWord, intentReco, slotEngine, glinerModel, voskModel,
                   noiseSuppression, vad, gainNormalizer,
                   pico, openai, weather_city_val] = await Promise.all([
                invoke<string>("db_read", { key: "selected_microphone" }),
                invoke<string>("db_read", { key: "selected_wake_word_engine" }),
                invoke<string>("db_read", { key: "selected_intent_recognition_engine" }),
                invoke<string>("db_read", { key: "selected_slot_extraction_engine" }),
                invoke<string>("db_read", { key: "selected_gliner_model" }),
                invoke<string>("db_read", { key: "selected_vosk_model" }),

                invoke<string>("db_read", { key: "noise_suppression" }),
                invoke<string>("db_read", { key: "vad" }),
                invoke<string>("db_read", { key: "gain_normalizer" }),

                invoke<string>("db_read", { key: "api_key__picovoice" }),
                invoke<string>("db_read", { key: "api_key__openai" }),
                invoke<string>("db_read", { key: "weather_city" })
            ])

            selectedMicrophone = mic
            selectedWakeWordEngine = wakeWord
            selectedIntentRecognitionEngine = intentReco
            selectedSlotExtractionEngine = slotEngine
            selectedVoskModel = voskModel
            selectedGlinerModel = glinerModel
            selectedNoiseSuppression = noiseSuppression
            selectedVad = vad
            gainNormalizerEnabled = gainNormalizer === "true"
            windowsAutostart = await invoke<boolean>("is_windows_autostart")
            apiKeyPicovoice = pico
            apiKeyOpenai = openai
            weatherCity = weather_city_val
        } catch (err) {
            console.error("failed to load settings:", err)
        }
    })
</script>

<div class="settings-layout">
    <div class="sidebar">
        <div class="sidebar-header">
            <h2>Настройки</h2>
        </div>
        <nav class="nav-menu">
            <button class="nav-item" class:active={activeTab === 'general'} on:click={() => activeTab = 'general'}>
                <Gear class="nav-icon" />
                <span>{t('settings-general')}</span>
            </button>
            <button class="nav-item" class:active={activeTab === 'devices'} on:click={() => activeTab = 'devices'}>
                <Mix class="nav-icon" />
                <span>{t('settings-devices')}</span>
            </button>
            <button class="nav-item" class:active={activeTab === 'neural'} on:click={() => activeTab = 'neural'}>
                <Cube class="nav-icon" />
                <span>{t('settings-neural-networks')}</span>
            </button>
            <button class="nav-item" class:active={activeTab === 'weather'} on:click={() => activeTab = 'weather'}>
                <Globe class="nav-icon" />
                <span>Погода</span>
            </button>
        </nav>

        <div class="sidebar-footer">
            <Button
                color="lime"
                radius="md"
                size="sm"
                uppercase
                ripple
                fullSize
                on:click={saveSettings}
                disabled={saveButtonDisabled}
            >
                {t('settings-save')}
            </Button>
            <Space h="sm" />
            <Button
                color="gray"
                radius="md"
                size="sm"
                uppercase
                fullSize
                on:click={() => $goto("/")}
            >
                {t('settings-back')}
            </Button>
        </div>
    </div>

    <div class="content-area">
        {#if settingsSaved}
            <Notification
                title={t('notification-saved')}
                icon={Check}
                color="teal"
                on:close={() => { settingsSaved = false }}
            />
            <Space h="xl" />
        {/if}

        <div class="tab-content" class:active={activeTab === 'general'}>
            <div class="card">
                <h3>Основные настройки</h3>
                <InputWrapper label={t('settings-autostart-label')}>
                    <Text size="sm" color="gray">
                        {t('settings-autostart-desc')}
                    </Text>
                    <Space h="xs" />
                    <Switch
                        label={windowsAutostart ? t('settings-enabled') : t('settings-disabled')}
                        bind:checked={windowsAutostart}
                    />
                </InputWrapper>
            </div>

            <Space h="md" />

            <div class="card">
                <h3>Голос ассистента</h3>
                <div class="voice-select">
                    <p class="description">{t('settings-voice-desc')}</p>
                    
                    <div class="voice-options">
                        {#each availableVoices as voice}
                            <button 
                                type="button"
                                class="voice-option"
                                class:selected={voiceVal === voice.id}
                                on:click={() => selectVoice(voice.id)}
                            >
                                <div class="voice-info">
                                    <span class="voice-name">{voice.name}</span>
                                    {#if voice.author}
                                        <span class="voice-author">by {voice.author}</span>
                                    {/if}
                                </div>
                                <div class="voice-languages">
                                    {#each voice.languages as lang}
                                        <img 
                                            src="/media/flags/{lang.toUpperCase()}.png" 
                                            alt={lang} 
                                            width="20" 
                                            title={lang}
                                        />
                                    {/each}
                                </div>
                            </button>
                        {/each}
                        
                        {#if availableVoices.length === 0}
                            <p class="no-voices">{t('settings-no-voices')}</p>
                        {/if}
                    </div>
                </div>
            </div>
        </div>

        <div class="tab-content" class:active={activeTab === 'devices'}>
            <div class="card">
                <h3>Устройства</h3>
                <NativeSelect
                    data={availableMicrophones}
                    label={t('settings-microphone')}
                    description={t('settings-microphone-desc')}
                    variant="filled"
                    bind:value={selectedMicrophone}
                />
            </div>
        </div>

        <div class="tab-content" class:active={activeTab === 'neural'}>
            <div class="card">
                <h3>Распознавание речи и намерений</h3>
                <NativeSelect
                    data={[
                        { label: "Rustpotter", value: "Rustpotter" },
                        { label: "Vosk", value: "Vosk" },
                        { label: "Picovoice Porcupine", value: "Picovoice" }
                    ]}
                    label={t('settings-wake-word-engine')}
                    description={t('settings-wake-word-desc')}
                    variant="filled"
                    bind:value={selectedWakeWordEngine}
                />

                {#if selectedWakeWordEngine === "picovoice"}
                    <Space h="sm" />
                    <Alert title={t('settings-attention')} color="#868E96" variant="outline">
                        <Notification
                            title={t('settings-picovoice-warning')}
                            icon={CrossCircled}
                            color="orange"
                            withCloseButton={false}
                        >
                            {t('settings-picovoice-waiting')}
                        </Notification>
                        <Space h="sm" />
                        <Text size="sm" color="gray">
                            {t('settings-picovoice-key-desc')}
                            <a href="https://console.picovoice.ai/" target="_blank">Picovoice Console</a>.
                        </Text>
                        <Space h="sm" />
                        <Input
                            icon={Code}
                            placeholder={t('settings-picovoice-key')}
                            variant="filled"
                            autocomplete="off"
                            bind:value={apiKeyPicovoice}
                        />
                    </Alert>
                {/if}

                <Space h="xl" />
                {#key availableVoskModels}
                <NativeSelect
                    data={[
                        { label: t('settings-auto-detect'), value: "" },
                        ...availableVoskModels
                    ]}
                    label={t('settings-vosk-model')}
                    description={t('settings-vosk-model-desc')}
                    variant="filled"
                    bind:value={selectedVoskModel}
                />
                {/key}

                {#if availableVoskModels.length === 0}
                    <Space h="sm" />
                    <Alert title={t('settings-models-not-found')} color="orange" variant="outline">
                        <Text size="sm" color="gray">
                            {t('settings-models-hint')}
                        </Text>
                    </Alert>
                {/if}

                <Space h="xl" />
                <NativeSelect
                    data={[
                        { label: "Intent Classifier", value: "IntentClassifier" },
                        { label: "Embedding Classifier", value: "EmbeddingClassifier" }
                    ]}
                    label={t('settings-intent-engine')}
                    description={t('settings-intent-engine-desc')}
                    variant="filled"
                    bind:value={selectedIntentRecognitionEngine}
                />

                <Space h="xl" />
                <NativeSelect
                    data={[
                        { label: t('settings-disabled'), value: "None" },
                        { label: "GLiNER (NER)", value: "GLiNER" }
                    ]}
                    label={t('settings-slot-engine')}
                    description={t('settings-slot-engine-desc')}
                    variant="filled"
                    bind:value={selectedSlotExtractionEngine}
                />

                {#if selectedSlotExtractionEngine === "GLiNER"}
                    <Space h="sm" />
                    {#key availableGlinerModels}
                    <NativeSelect
                        data={[
                            { label: t('settings-auto-detect'), value: "" },
                            ...availableGlinerModels
                        ]}
                        label={t('settings-gliner-model')}
                        description={t('settings-gliner-model-desc')}
                        variant="filled"
                        bind:value={selectedGlinerModel}
                    />
                    {/key}

                    {#if availableGlinerModels.length === 0}
                        <Space h="sm" />
                        <Alert title={t('settings-models-not-found')} color="orange" variant="outline">
                            <Text size="sm" color="gray">
                                {t('settings-gliner-models-hint')}
                            </Text>
                        </Alert>
                    {/if}
                {/if}
            </div>

            <Space h="md" />

            <div class="card">
                <h3>Обработка аудио</h3>
                <NativeSelect
                    data={[
                        { label: t('settings-disabled'), value: "None" },
                        { label: "Nnnoiseless", value: "Nnnoiseless" }
                    ]}
                    label={t('settings-noise-suppression')}
                    description={t('settings-noise-suppression-desc')}
                    variant="filled"
                    bind:value={selectedNoiseSuppression}
                />

                <Space h="md" />

                <NativeSelect
                    data={[
                        { label: t('settings-disabled'), value: "None" },
                        { label: "Energy", value: "Energy" },
                        { label: "Nnnoiseless", value: "Nnnoiseless" }
                    ]}
                    label={t('settings-vad')}
                    description={t('settings-vad-desc')}
                    variant="filled"
                    bind:value={selectedVad}
                />

                <Space h="md" />

                <InputWrapper label={t('settings-gain-normalizer')}>
                    <Text size="sm" color="gray">
                        {t('settings-gain-normalizer-desc')}
                    </Text>
                    <Space h="xs" />
                    <Switch
                        label={gainNormalizerEnabled ? t('settings-enabled') : t('settings-disabled')}
                        bind:checked={gainNormalizerEnabled}
                    />
                </InputWrapper>
            </div>
        </div>

        <div class="tab-content" class:active={activeTab === 'weather'}>
            <div class="card">
                <h3>Настройки погоды</h3>
                <InputWrapper label="Город для погоды">
                    <Text size="sm" color="gray">
                        Введите название вашего города на русском языке (например: Москва, Санкт-Петербург).
                    </Text>
                    <Space h="xs" />
                    <Input
                        placeholder="Ваш город"
                        variant="filled"
                        bind:value={weatherCity}
                    />
                </InputWrapper>
            </div>
        </div>
    </div>
</div>

<style lang="scss">
:global(body) {
    background-color: #0b0e14;
    color: #e0e6ed;
    overflow: hidden !important; /* Force prevent body scroll */
    margin: 0;
    padding: 0;
}

.settings-layout {
    display: flex;
    height: calc(100vh - 60px); /* Adjust for header */
    width: 100vw;
    overflow: hidden;
    position: fixed;
    top: 60px; /* Below header */
    left: 0;
    background: linear-gradient(135deg, #0b0e14 0%, #121820 100%);
}

.sidebar {
    width: 260px;
    height: 100%;
    flex-shrink: 0;
    background: rgba(15, 20, 25, 0.95);
    border-right: 1px solid rgba(255, 255, 255, 0.05);
    display: flex;
    flex-direction: column;
    padding: 40px 20px 20px 20px; /* Added extra top padding */
    box-shadow: 2px 0 10px rgba(0, 0, 0, 0.2);
    z-index: 10;
    overflow-y: auto; /* Allow sidebar itself to scroll if needed, but not hide */
}

.sidebar-header {
    display: none; /* Hide header since app already has one */
    margin-bottom: 30px;
    h2 {
        margin: 0;
        font-size: 1.5rem;
        font-weight: 600;
        color: #fff;
        letter-spacing: 0.5px;
    }
}

.nav-menu {
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex: 1;
}

.nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: transparent;
    border: none;
    border-radius: 8px;
    color: #a0aab5;
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    text-align: left;

    &:hover {
        background: rgba(255, 255, 255, 0.05);
        color: #fff;
    }

    &.active {
        background: rgba(138, 200, 50, 0.15);
        color: #8AC832;
        
        :global(.nav-icon) {
            color: #8AC832;
        }
    }

    :global(.nav-icon) {
        width: 18px;
        height: 18px;
        opacity: 0.8;
    }
}

.sidebar-footer {
    margin-top: auto;
    padding-top: 20px;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
}

.content-area {
    flex: 1;
    height: 100%;
    padding: 30px;
    padding-bottom: 80px; /* Extra padding at bottom to ensure content isn't cut off */
    overflow-y: auto;
    background: transparent;
    
    &::-webkit-scrollbar {
        width: 8px;
    }
    
    &::-webkit-scrollbar-track {
        background: rgba(0, 0, 0, 0.1);
    }
    
    &::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.1);
        border-radius: 4px;
        
        &:hover {
            background: rgba(255, 255, 255, 0.2);
        }
    }
}

.tab-content {
    display: none;
    animation: fadeIn 0.3s ease;
    
    &.active {
        display: block;
    }
}

@keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
}

.card {
    background: rgba(20, 26, 33, 0.6);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 12px;
    padding: 24px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    backdrop-filter: blur(10px);

    h3 {
        margin: 0 0 20px 0;
        font-size: 1.1rem;
        font-weight: 500;
        color: #fff;
        border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        padding-bottom: 10px;
    }
}

// Voice select styles
.voice-select {
    .description {
        font-size: 0.85rem;
        color: #a0aab5;
        margin: 0 0 1rem;
    }
}

.voice-options {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
    gap: 12px;
}

.voice-option {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: rgba(30, 40, 45, 0.4);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    text-align: left;
    
    &:hover {
        background: rgba(40, 55, 60, 0.6);
        border-color: rgba(255, 255, 255, 0.1);
        transform: translateY(-1px);
    }
    
    &.selected {
        background: rgba(138, 200, 50, 0.1);
        border-color: rgba(138, 200, 50, 0.4);
        box-shadow: 0 0 10px rgba(138, 200, 50, 0.1);
    }
}

.voice-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.voice-name {
    font-size: 0.9rem;
    color: #fff;
    font-weight: 500;
}

.voice-author {
    font-size: 0.75rem;
    color: #a0aab5;
}

.voice-languages {
    display: flex;
    gap: 6px;
    
    img {
        opacity: 0.9;
        border-radius: 2px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.2);
    }
}

.no-voices {
    font-size: 0.85rem;
    color: #a0aab5;
    font-style: italic;
    grid-column: 1 / -1;
}
</style>