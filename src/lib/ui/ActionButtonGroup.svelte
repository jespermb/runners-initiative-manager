<script lang="ts">
    import CyberpunkButton from "./CyberpunkButton.svelte";

    interface Props {
        onEdit?: () => void;
        onDelete?: () => void;
        editLabel?: string;
        deleteLabel?: string;
        size?: 'sm' | 'md';
        editDisabled?: boolean;
        deleteDisabled?: boolean;
    }

    const { 
        onEdit,
        onDelete,
        editLabel = "Edit",
        deleteLabel = "Remove",
        size = 'md',
        editDisabled = false,
        deleteDisabled = false
    }: Props = $props();
</script>

<div class="action-button-group">
    {#if onEdit}
        <CyberpunkButton
            variant="edit"
            {size}
            disabled={editDisabled}
            onClick={onEdit}
            ariaLabel={editLabel}
        />
    {/if}
    
    {#if onDelete}
        <CyberpunkButton
            variant="delete"
            {size}
            disabled={deleteDisabled}
            onClick={onDelete}
            ariaLabel={deleteLabel}
        />
    {/if}
</div>

<style>
    .action-button-group {
        display: flex;
        gap: 0.5rem;
        align-items: center;
    }
</style>