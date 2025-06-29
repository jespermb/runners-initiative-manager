<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import ActionButtonGroup from "./ui/ActionButtonGroup.svelte";
    
    const dispatch = createEventDispatcher();
    const { name = "", displayName = name, physical, stun } = $props<{ 
        name: string; 
        displayName?: string; 
        physical?: number; 
        stun?: number; 
    }>();

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
        {#if physical !== undefined && stun !== undefined}
            <div class="health-indicators">
                <div class="health-row">
                    <span class="health-label">PHY:</span>
                    <div class="health-squares">
                        {#each Array(physical) as _, i}
                            <div class="health-square physical"></div>
                        {/each}
                    </div>
                </div>
                <div class="health-row">
                    <span class="health-label">STN:</span>
                    <div class="health-squares">
                        {#each Array(stun) as _, i}
                            <div class="health-square stun"></div>
                        {/each}
                    </div>
                </div>
            </div>
        {/if}
    </div>
    <div class="item-actions">
        <ActionButtonGroup
            onEdit={edit}
            onDelete={remove}
            size="sm"
        />
    </div>
</div>

<style>
    .cyberpunk-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.75rem 0 0.75rem 1rem;
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
        margin: 0 0 0.25rem 0;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .health-indicators {
        display: flex;
        flex-direction: column;
        gap: 0.125rem;
    }

    .health-row {
        display: flex;
        align-items: center;
        gap: 0.25rem;
    }

    .health-label {
        font-size: 0.65rem;
        font-weight: 600;
        color: #c0c0c0;
        min-width: 2rem;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .health-squares {
        display: flex;
        gap: 0.125rem;
        flex-wrap: wrap;
    }

    .health-square {
        width: 0.5rem;
        height: 0.5rem;
        border-radius: 0.125rem;
        border: 1px solid rgba(255, 255, 255, 0.3);
        position: relative;
    }

    .health-square.physical {
        background-color: #ef4444;
        border-color: #dc2626;
        box-shadow: 0 0 3px rgba(239, 68, 68, 0.5);
    }

    .health-square.stun {
        background-color: #3b82f6;
        border-color: #2563eb;
        box-shadow: 0 0 3px rgba(59, 130, 246, 0.5);
    }

    .item-actions {
        position: relative;
        z-index: 1;
    }
</style>
