<script lang="ts">
    interface Props {
        items: any[];
        emptyMessage?: string;
    }

    const { items, emptyMessage = "No items found" }: Props = $props();
</script>

<div class="tab-content-container">
    {#if items.length === 0}
        <div class="empty-message">
            <p>{emptyMessage}</p>
        </div>
    {:else}
        <div class="items-list">
            {#each items as item (item.id)}
                <slot {item}></slot>
            {/each}
        </div>
    {/if}
</div>

<style>
    .tab-content-container {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        padding: 0.5rem;
    }

    .empty-message {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 2rem;
        color: #c0c0c0;
        font-style: italic;
    }

    .empty-message p {
        margin: 0;
    }

    .items-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
</style>