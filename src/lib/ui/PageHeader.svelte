<script lang="ts">
    import CyberpunkButton from "./CyberpunkButton.svelte";

    interface Props {
        title: string;
        backUrl?: string;
        onEdit?: () => void;
        editLabel?: string;
    }

    const { title, backUrl, onEdit, editLabel = "Edit" }: Props = $props();
</script>

<div class="cyberpunk-header">
    <div class="header-left">
        {#if backUrl}
            <a href={backUrl} class="return-button" aria-label="Return">
                <CyberpunkButton
                    variant="return"
                    size="lg"
                    onClick={() => {}}
                />
            </a>
        {/if}
        <h2 class="header-title">{title}</h2>
    </div>
    
    {#if onEdit}
        <CyberpunkButton
            variant="edit"
            size="lg"
            onClick={onEdit}
            ariaLabel={editLabel}
        />
    {/if}
</div>

<style>
    .cyberpunk-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        width: 100%;
        position: relative;
    }

    .header-left {
        display: flex;
        align-items: center;
        gap: 0.75rem;
    }

    .return-button {
        text-decoration: none;
    }

    .header-title {
        color: white;
        font-size: 1.5rem;
        font-weight: 700;
        margin: 0;
        text-transform: uppercase;
        letter-spacing: 1px;
        position: relative;
        z-index: 1;
        text-shadow: 0 0 5px rgba(0, 243, 255, 0.7);
    }
</style>