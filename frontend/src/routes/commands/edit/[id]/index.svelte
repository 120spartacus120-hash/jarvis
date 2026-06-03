<script lang="ts">
    import { params } from "@roxi/routify"
    import CommandForm from "@/components/commands/CommandForm.svelte"
    import { translations, translate } from "@/stores"

    $: t = (key: string) => translate($translations, key)
    $: commandId = ($params.id as string) ?? ""
</script>

{#if commandId}
    {#key commandId}
        <CommandForm mode="edit" {commandId} />
    {/key}
{:else}
    <p class="hint">{t("stats-loading")}</p>
{/if}

<style lang="scss">
    .hint {
        text-align: center;
        color: rgba(255, 255, 255, 0.5);
        font-size: 0.85rem;
        margin-top: 2rem;
    }
</style>
