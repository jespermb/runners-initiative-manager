<script lang="ts">
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();
    const { name = "", displayName = name } = $props<{ name: string; displayName?: string }>();

    function edit() {
        dispatch("edit");
    }

    function remove() {
        console.log("ListItem remove clicked for:", name);
        // Temporarily skip confirmation dialog for debugging
        console.log("Dispatching remove event");
        dispatch("remove");
    }
</script>

<div class="cyberpunk-item">
    <div class="item-content">
        <h2 class="item-name">{name}</h2>
    </div>
    <div class="item-actions">
        <button 
            class="cyberpunk-btn edit"
            onclick={edit}
            aria-label="Edit"
        >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
            </svg>
        </button>
        <button 
            class="cyberpunk-btn delete"
            onclick={remove}
            aria-label="Remove"
        >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
        </button>
    </div>
</div>

<style>
    .cyberpunk-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.75rem 1rem;
        background-color: #0a0a0a;
        color: white;
        border-radius: 0.25rem;
        margin-bottom: 0.75rem;
        border-left: 3px solid #00f3ff;
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.2);
        position: relative;
        overflow: hidden;
    }

    .cyberpunk-item::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: linear-gradient(90deg, 
            rgba(0, 243, 255, 0.1) 0%, 
            rgba(0, 0, 0, 0) 20%);
        z-index: 0;
    }

    .item-content {
        flex: 1;
        overflow: hidden;
        position: relative;
        z-index: 1;
    }

    .item-name {
        font-size: 1rem;
        font-weight: 500;
        margin: 0;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .item-actions {
        display: flex;
        gap: 0.5rem;
        position: relative;
        z-index: 1;
    }

    .cyberpunk-btn {
        width: 2rem;
        height: 2rem;
        border-radius: 0.25rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border: 1px solid;
        background-color: transparent;
        transition: all 0.2s ease;
    }

    .cyberpunk-btn.edit {
        border-color: #00f3ff;
        color: #00f3ff;
    }

    .cyberpunk-btn.delete {
        border-color: #ff00ff;
        color: #ff00ff;
    }

    .cyberpunk-btn.edit:hover {
        background-color: rgba(0, 243, 255, 0.1);
        box-shadow: 0 0 8px rgba(0, 243, 255, 0.5);
    }

    .cyberpunk-btn.delete:hover {
        background-color: rgba(255, 0, 255, 0.1);
        box-shadow: 0 0 8px rgba(255, 0, 255, 0.5);
    }
</style>
