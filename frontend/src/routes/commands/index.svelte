<script lang="ts">
    import { onMount } from "svelte"
    import { goto } from "@roxi/routify"
    import { invoke } from "@tauri-apps/api/core"

    import HDivider from "@/components/elements/HDivider.svelte"
    import Footer from "@/components/Footer.svelte"
    import { translations, translate } from "@/stores"
    import {
        loadCustomCommands,
        type CustomCommandsConfig,
    } from "@/lib/customCommands"

    import { Button, Input } from "@svelteuidev/core"
    import { Plus, MagnifyingGlass } from "radix-icons-svelte"

    $: t = (key: string, args?: Record<string, string | number>) => translate($translations, key, args)

    interface CommandCard {
        id: string
        title: string
        descKey: string
        descArgs?: Record<string, string>
        route: string
        routeParams?: Record<string, string>
        phraseCount: number
        configured: boolean
        builtin: boolean
        typeLabelKey?: string
    }

    let search = ""
    let loading = true
    let cards: CommandCard[] = []

    $: filtered = cards.filter((card) => {
        const q = search.trim().toLowerCase()
        if (!q) return true
        const haystack = [
            card.title,
            card.descKey ? t(card.descKey) : "",
            card.descArgs?.city ?? "",
            card.descArgs?.program ?? "",
            card.descArgs?.site ?? "",
            card.typeLabelKey ? t(card.typeLabelKey) : "",
        ]
            .join(" ")
            .toLowerCase()
        return haystack.includes(q)
    })

    async function loadCards() {
        loading = true
        try {
            const [config, weatherCity] = await Promise.all([
                loadCustomCommands(),
                invoke<string>("db_read", { key: "weather_city" }).catch(() => ""),
            ])
            cards = buildCards(config, weatherCity ?? "")
        } catch (err) {
            console.error("failed to load commands:", err)
            cards = []
        } finally {
            loading = false
        }
    }

    function buildCards(config: CustomCommandsConfig, weatherCity: string): CommandCard[] {
        const city = weatherCity.trim()
        const weatherPhrases = config.weather_phrases?.length ?? 0

        const builtin: CommandCard[] = [
            {
                id: "thanks",
                title: t("commands-card-thanks"),
                descKey: "commands-card-thanks-desc",
                route: "/commands/thanks",
                phraseCount: config.thanks_phrases?.length ?? 0,
                configured: (config.thanks_phrases?.length ?? 0) > 0,
                builtin: true,
            },
            {
                id: "shutdown",
                title: t("commands-card-shutdown"),
                descKey: "commands-card-shutdown-desc",
                route: "/commands/shutdown",
                phraseCount: config.shutdown_phrases?.length ?? 0,
                configured: (config.shutdown_phrases?.length ?? 0) > 0,
                builtin: true,
            },
            {
                id: "weather",
                title: t("commands-card-weather"),
                descKey: city ? "" : "commands-card-weather-desc",
                descArgs: city ? { city } : undefined,
                route: "/commands/weather",
                phraseCount: weatherPhrases,
                configured: Boolean(city) && weatherPhrases > 0,
                builtin: true,
            },
        ]

        const userCards: CommandCard[] = (config.user_commands ?? [])
            .filter((cmd) => cmd.type === "open_program" || cmd.type === "open_website")
            .map((cmd): CommandCard => {
                if (cmd.type === "open_website") {
                    const site =
                        cmd.website_url?.trim() ||
                        cmd.program_path?.trim() ||
                        ""
                    let host = site
                    try {
                        if (site) host = new URL(site).hostname
                    } catch {
                        host = site
                    }
                    return {
                        id: cmd.id,
                        title: cmd.name,
                        descKey: site ? "" : "commands-type-open-website",
                        descArgs: site ? { site: host } : undefined,
                        route: "/commands/edit/[id]",
                        routeParams: { id: cmd.id },
                        phraseCount: cmd.phrases?.length ?? 0,
                        configured:
                            Boolean(site) && (cmd.phrases?.length ?? 0) > 0,
                        builtin: false,
                        typeLabelKey: "commands-type-open-website",
                    }
                }

                const program = cmd.program_path?.trim() ?? ""
                const fileName = program.split(/[/\\]/).pop() ?? program
                return {
                    id: cmd.id,
                    title: cmd.name,
                    descKey: program ? "" : "commands-type-open-program",
                    descArgs: program ? { program: fileName } : undefined,
                    route: "/commands/edit/[id]",
                    routeParams: { id: cmd.id },
                    phraseCount: cmd.phrases?.length ?? 0,
                    configured:
                        Boolean(program) && (cmd.phrases?.length ?? 0) > 0,
                    builtin: false,
                    typeLabelKey: "commands-type-open-program",
                }
            })

        return [...builtin, ...userCards]
    }

    function openCard(card: CommandCard) {
        if (card.routeParams) {
            $goto(card.route, card.routeParams)
        } else {
            $goto(card.route)
        }
    }
    onMount(loadCards)
</script>

<div class="commands-layout">
    <div class="sidebar">
        <div class="sidebar-header">
            <h2>Команды</h2>
            <p class="subtitle">{t("commands-list-desc")}</p>
        </div>

        <div class="actions">
            <Input
                class="search"
                type="search"
                placeholder={t("commands-search")}
                bind:value={search}
                icon={MagnifyingGlass}
            />

            <Button class="add-btn" color="lime" radius="md" size="sm" uppercase fullSize on:click={() => $goto("/commands/new")}>
                <Plus size={16} style="margin-right: 8px;" />
                {t("commands-add-new")}
            </Button>
        </div>
    </div>

    <div class="content-area">
        {#if loading}
            <div class="empty-state">
                <p class="hint">{t("stats-loading")}</p>
            </div>
        {:else if filtered.length === 0}
            <div class="empty-state">
                <p class="hint">{t("error-not-found")}</p>
            </div>
        {:else}
            <div class="command-grid">
                {#each filtered as card}
                    <button type="button" class="command-card" on:click={() => openCard(card)}>
                        <div class="card-head">
                            <span class="card-title">{card.title}</span>
                            {#if card.configured}
                                <span class="card-badge ok">{t("commands-configured")}</span>
                            {:else}
                                <span class="card-badge">{t("commands-not-configured")}</span>
                            {/if}
                        </div>
                        <p class="card-desc">
                            {#if card.builtin && card.descArgs?.city}
                                {t("commands-card-weather-city", { city: card.descArgs.city })}
                            {:else if card.builtin && card.descKey}
                                {t(card.descKey)}
                            {:else if card.descArgs?.program}
                                {t("commands-type-open-program")} · {card.descArgs.program}
                            {:else if card.descArgs?.site}
                                {t("commands-type-open-website")} · {card.descArgs.site}
                            {:else if card.descKey}
                                {t(card.descKey, card.descArgs)}
                            {:else if card.typeLabelKey}
                                {t(card.typeLabelKey)}
                            {/if}
                        </p>
                        <p class="card-meta">
                            {t("commands-phrases-count", { count: card.phraseCount })}
                            {#if !card.builtin}
                                · {t("commands-tap-to-edit")}
                            {/if}
                        </p>
                    </button>
                {/each}
            </div>
        {/if}
    </div>
</div>

<style lang="scss">
    :global(body) {
        background-color: #0b0e14;
        color: #e0e6ed;
        overflow: hidden !important;
        margin: 0;
        padding: 0;
    }

    .commands-layout {
        display: flex;
        height: calc(100vh - 60px);
        width: 100vw;
        overflow: hidden;
        position: fixed;
        top: 60px;
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
        overflow-y: auto;
    }

    .sidebar-header {
        margin-bottom: 20px;
        h2 {
            margin: 0;
            font-size: 1.5rem;
            font-weight: 600;
            color: #fff;
            letter-spacing: 0.5px;
        }
    }

    .subtitle {
        margin: 0.5rem 0 0;
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.45);
        line-height: 1.4;
    }

    .actions {
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    :global(.search) {
        width: 100%;
    }

    .content-area {
        flex: 1;
        height: 100%;
        padding: 30px;
        padding-bottom: 80px;
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

    .command-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 16px;
    }

    .command-card {
        text-align: left;
        width: 100%;
        padding: 20px;
        border-radius: 12px;
        border: 1px solid rgba(255, 255, 255, 0.05);
        background: rgba(20, 26, 33, 0.6);
        backdrop-filter: blur(10px);
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
            border-color: rgba(138, 200, 50, 0.4);
            transform: translateY(-2px);
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
            background: rgba(30, 38, 48, 0.8);
        }
    }

    .card-head {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: 0.5rem;
        margin-bottom: 12px;
    }

    .card-title {
        font-size: 1.1rem;
        font-weight: 600;
        color: #fff;
    }

    .card-badge {
        flex-shrink: 0;
        font-size: 0.65rem;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        padding: 4px 8px;
        border-radius: 999px;
        color: rgba(255, 255, 255, 0.5);
        background: rgba(255, 255, 255, 0.08);

        &.ok {
            color: #8AC832;
            background: rgba(138, 200, 50, 0.15);
        }
    }

    .card-desc {
        margin: 0 0 12px;
        font-size: 0.85rem;
        color: rgba(255, 255, 255, 0.6);
        line-height: 1.5;
    }

    .card-meta {
        margin: 0;
        font-size: 0.75rem;
        color: rgba(255, 255, 255, 0.4);
    }

    .empty-state {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
    }

    .hint {
        color: rgba(255, 255, 255, 0.5);
        font-size: 0.95rem;
    }
</style>
