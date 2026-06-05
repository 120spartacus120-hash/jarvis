<script lang="ts">
    import { onMount } from "svelte"
    import { invoke } from "@tauri-apps/api/core"
    import { relaunch } from "@tauri-apps/plugin-process"
    import { Text, Space } from "@svelteuidev/core"
    import { translations, translate, currentLanguage } from "@/stores"

    $: t = (key: string) => translate($translations, key)

    interface LanguageOption {
        code: string
        name: string
    }

    let options: LanguageOption[] = []
    let selected = ""
    let applying = false

    onMount(async () => {
        try {
            options = await invoke<LanguageOption[]>("get_language_options")
            selected = await invoke<string>("get_current_language")
        } catch (err) {
            console.error("Failed to load languages:", err)
        }
    })

    async function onSelect(code: string) {
        if (applying || code === selected) return

        applying = true
        selected = code

        try {
            await invoke("set_language", { lang: code })
            await relaunch()
        } catch (err) {
            console.error("Failed to change language:", err)
            applying = false
            selected = $currentLanguage
        }
    }
</script>

<div class="card">
    <h3>{t("settings-language-interface")}</h3>
    <Text size="sm" color="gray">{t("settings-language-desc")}</Text>
    <Space h="sm" />
    <Text size="xs" color="gray" class="restart-note">{t("settings-language-restart-note")}</Text>
    <Space h="md" />

    <div class="language-list" role="listbox" aria-label={t("settings-language")}>
        {#each options as opt (opt.code)}
            <button
                type="button"
                class="language-option"
                class:selected={selected === opt.code}
                class:applying={applying && selected === opt.code}
                disabled={applying}
                on:click={() => onSelect(opt.code)}
            >
                <span class="language-name">{opt.name}</span>
                <span class="language-code">{opt.code.toUpperCase()}</span>
            </button>
        {/each}
    </div>
</div>

<style lang="scss">
    .restart-note {
        opacity: 0.75;
    }

    .language-list {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
        gap: 10px;
    }

    .language-option {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 12px;
        padding: 12px 16px;
        background: rgba(30, 40, 45, 0.4);
        border: 1px solid rgba(255, 255, 255, 0.06);
        border-radius: 8px;
        color: #e0e6ed;
        cursor: pointer;
        transition: all 0.2s ease;
        text-align: left;

        &:hover:not(:disabled) {
            background: rgba(40, 55, 60, 0.6);
            border-color: rgba(255, 255, 255, 0.12);
        }

        &.selected {
            background: rgba(138, 200, 50, 0.12);
            border-color: rgba(138, 200, 50, 0.45);
        }

        &:disabled {
            opacity: 0.7;
            cursor: wait;
        }
    }

    .language-name {
        font-size: 0.95rem;
        font-weight: 500;
    }

    .language-code {
        font-size: 0.7rem;
        letter-spacing: 0.08em;
        color: rgba(255, 255, 255, 0.45);
    }
</style>
