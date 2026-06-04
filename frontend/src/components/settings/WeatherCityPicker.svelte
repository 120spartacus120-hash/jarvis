<script lang="ts">
    import { ChevronDown } from "radix-icons-svelte"
    import { WEATHER_CITIES } from "@/lib/weatherCities"

    export let value = ""
    export let label = ""
    export let description = ""
    export let disabled = false

    let open = false

    $: options = WEATHER_CITIES.map((city) => ({
        value: city.value,
        label: city.label,
    }))

    $: selectedLabel =
        options.find((item) => item.value === value)?.label ?? options[0]?.label ?? ""

    function select(next: string) {
        value = next
        open = false
    }

    function toggle() {
        if (disabled) return
        open = !open
    }

    function handleWindowClick(event: MouseEvent) {
        const target = event.target as HTMLElement
        if (!target.closest(".city-picker")) {
            open = false
        }
    }
</script>

<svelte:window on:click={handleWindowClick} />

<div class="city-picker" class:open class:disabled>
    {#if label}
        <span class="picker-label">{label}</span>
    {/if}
    {#if description}
        <p class="picker-desc">{description}</p>
    {/if}

    <button
        type="button"
        class="picker-trigger"
        on:click|stopPropagation={toggle}
        {disabled}
    >
        <span class="picker-value">{selectedLabel}</span>
        <span class="chevron-wrap"><ChevronDown size={16} /></span>
    </button>

    {#if open && !disabled}
        <ul class="picker-panel" role="listbox">
            {#each options as option}
                <li>
                    <button
                        type="button"
                        role="option"
                        class:selected={option.value === value}
                        aria-selected={option.value === value}
                        on:click|stopPropagation={() => select(option.value)}
                    >
                        {option.label}
                    </button>
                </li>
            {/each}
        </ul>
    {/if}
</div>

<style lang="scss">
    .city-picker {
        position: relative;

        &.disabled .picker-trigger {
            opacity: 0.65;
            cursor: default;
        }
    }

    .picker-label {
        display: block;
        margin-bottom: 0.35rem;
        font-size: 0.72rem;
        font-weight: 600;
        color: rgba(255, 255, 255, 0.55);
        text-transform: uppercase;
        letter-spacing: 0.04em;
    }

    .picker-desc {
        margin: 0 0 0.55rem;
        font-size: 0.72rem;
        color: rgba(255, 255, 255, 0.42);
        line-height: 1.4;
    }

    .picker-trigger {
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.5rem;
        padding: 0.7rem 0.85rem;
        border-radius: 10px;
        border: 1px solid rgba(82, 254, 254, 0.35);
        background: rgba(8, 16, 20, 0.9);
        color: #fff;
        font-size: 0.88rem;
        font-weight: 600;
        cursor: pointer;
        transition: border-color 0.15s ease, box-shadow 0.15s ease;

        &:hover:not(:disabled) {
            border-color: rgba(82, 254, 254, 0.7);
            box-shadow: 0 0 16px rgba(82, 254, 254, 0.12);
        }
    }

    .city-picker.open .picker-trigger {
        border-color: rgba(82, 254, 254, 0.85);
        box-shadow: 0 0 18px rgba(82, 254, 254, 0.18);
    }

    .chevron-wrap {
        display: flex;
        color: #52fefe;
        flex-shrink: 0;
        transition: transform 0.2s ease;
    }

    .city-picker.open .chevron-wrap {
        transform: rotate(180deg);
    }

    .picker-panel {
        list-style: none;
        margin: 0.4rem 0 0;
        padding: 0.35rem;
        border-radius: 10px;
        border: 1px solid rgba(82, 254, 254, 0.3);
        background: rgba(12, 22, 28, 0.98);
        box-shadow: 0 12px 32px rgba(0, 0, 0, 0.45);
        z-index: 20;
        position: relative;
    }

    .picker-panel li button {
        width: 100%;
        text-align: left;
        padding: 0.65rem 0.75rem;
        border: none;
        border-radius: 8px;
        background: transparent;
        color: rgba(255, 255, 255, 0.88);
        font-size: 0.85rem;
        cursor: pointer;

        &:hover,
        &.selected {
            background: rgba(82, 254, 254, 0.12);
            color: #52fefe;
        }
    }
</style>
