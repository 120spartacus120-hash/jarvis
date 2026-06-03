<script lang="ts">
    import { translations, translate, isJarvisRunning, ipcConnected, sendTextCommand } from "@/stores"
    
    $: t = (key: string) => translate($translations, key)
    
    let searchQuery = ""
    let isProcessing = false
    let statusMessage = ""

    async function handleSubmit(e: Event) {
        e.preventDefault()
        
        const command = searchQuery.trim()
        if (!command || isProcessing) return
        
        if (!$isJarvisRunning || !$ipcConnected) {
            statusMessage = t('search-error-not-running')
            setTimeout(() => statusMessage = "", 3000)
            return
        }

        isProcessing = true
        statusMessage = ""

        try {
            await sendTextCommand(command)
            searchQuery = ""
        } catch (err) {
            console.error("Failed to send command:", err)
            statusMessage = t('search-error-failed')
            setTimeout(() => statusMessage = "", 3000)
        } finally {
            isProcessing = false
        }
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Escape") {
            searchQuery = ""
        }
    }
</script>

<div id="search-form" class="modern-search" class:active={searchQuery !== ""} class:processing={isProcessing}>
    <form on:submit={handleSubmit}>
        <div class="input-wrapper">
            <input
                bind:value={searchQuery}
                on:keydown={handleKeydown}
                type="text"
                name="q"
                placeholder={t('search-placeholder')}
                autocomplete="off"
                minlength="1"
                maxlength="200"
                disabled={isProcessing}
            />
            <button type="submit" class="submit-btn" disabled={isProcessing || !searchQuery.trim()}>
                {isProcessing ? '...' : 'Enter'}
            </button>
        </div>
    </form>
    {#if statusMessage}
        <div class="search-status">{statusMessage}</div>
    {/if}
</div>

<style lang="scss">
    .modern-search {
        position: relative;
        width: 100%;
        max-width: 600px;
        margin: 0 auto;

        form {
            width: 100%;
        }
    }

    .input-wrapper {
        position: relative;
        display: flex;
        align-items: center;
        width: 100%;
        background: transparent;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
        padding: 0.5rem 0;
        transition: all 0.3s ease;

        &:focus-within {
            border-bottom-color: rgba(82, 254, 254, 0.4);
        }
    }

    input {
        flex: 1;
        background: transparent;
        border: none;
        outline: none;
        color: #fff;
        font-size: 1rem;
        font-family: inherit;
        padding: 0.5rem 0;

        &::placeholder {
            color: rgba(255, 255, 255, 0.3);
            font-style: italic;
        }

        &:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }
    }

    .submit-btn {
        background: transparent;
        border: none;
        color: rgba(255, 255, 255, 0.3);
        padding: 0.5rem;
        font-size: 0.85rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 1px;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover:not(:disabled) {
            color: #52fefe;
        }

        &:disabled {
            opacity: 0.3;
            cursor: not-allowed;
        }
    }

    .modern-search.processing .input-wrapper {
        opacity: 0.7;
        pointer-events: none;
    }

    .search-status {
        position: absolute;
        bottom: -28px;
        left: 50%;
        transform: translateX(-50%);
        font-size: 0.8rem;
        color: #52fefe;
        white-space: nowrap;
        animation: fadeIn 0.2s ease;
        background: rgba(82, 254, 254, 0.1);
        padding: 0.25rem 0.75rem;
        border-radius: 100px;
        backdrop-filter: blur(4px);
    }

    @keyframes fadeIn {
        from { opacity: 0; transform: translateX(-50%) translateY(-5px); }
        to { opacity: 1; transform: translateX(-50%) translateY(0); }
    }
</style>