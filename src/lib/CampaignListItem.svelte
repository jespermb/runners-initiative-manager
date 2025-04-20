<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { Campaign } from "../types/Campaign";
    
    const dispatch = createEventDispatcher<{
        campaignRemoved: { id: number }
    }>();
    
    export let campaign: Campaign;
    
    function remove() {
        // Show confirmation dialog
        if (confirm(`Are you sure you want to remove "${campaign.name}"?`)) {
            dispatch("campaignRemoved", { id: campaign.id });
        }
    }
</script>

<div class="cyberpunk-item">
    <div class="item-content">
        <h2 class="item-name">{campaign.name}</h2>
        <p class="item-version">{campaign.version}th edition</p>
    </div>
    <div class="item-actions">
        <button 
            class="cyberpunk-btn delete"
            on:click={remove}
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
        color: white;
        border-radius: 0.25rem;
        margin-bottom: 0.75rem;
        border-left: 3px solid #00f3ff;
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.2);
        position: relative;
        overflow: hidden;
        transition: transform 0.2s ease, box-shadow 0.2s ease;
    }

    .cyberpunk-item:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 15px rgba(0, 243, 255, 0.3);
    }

    .cyberpunk-item::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
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
        color: #ffffff;
        text-shadow: 0 0 5px rgba(0, 243, 255, 0.5);
    }

    .item-version {
        font-size: 0.75rem;
        margin: 0.25rem 0 0;
        color: #a0a0a0;
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

    .cyberpunk-btn.delete {
        border-color: #ff00ff;
        color: #ff00ff;
    }

    .cyberpunk-btn.delete:hover {
        background-color: rgba(255, 0, 255, 0.1);
        box-shadow: 0 0 8px rgba(255, 0, 255, 0.5);
    }
</style>
