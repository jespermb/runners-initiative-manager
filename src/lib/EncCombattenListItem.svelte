<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher } from "svelte";
    import ListItem from "./ListItem.svelte";
    const dispatch = createEventDispatcher();

    const { name = "", id = 0, initiative = null, combatten_type = "npc", encounter_id = 0 } = $props();
    let resultMsg = $state("");
    let showEditForm = $state(false);
    let editName = $state(name);

    async function remove() {
        console.log("Remove function called with:", { encounterId: encounter_id, combattenId: id });
        try {
            resultMsg = await invoke("remove_combatten_from_encounter", { 
                encounterId: encounter_id, 
                combattenId: id 
            });
            console.log("Remove result:", resultMsg);
            dispatch("combattenRemoved");
        } catch (error) {
            console.error("Remove error:", error);
            resultMsg = `Error: ${error}`;
        }
    }

    function edit() {
        // For now, just show an alert
        // In a real implementation, this would open an edit form
        alert(`Edit NPC: ${name} (ID: ${id})`);
        
        // Example of how you might implement editing:
        // showEditForm = true;
        // editName = name;
    }

    async function saveEdit() {
        // This would be implemented to save the edited NPC
        // await invoke("update_combatten", { id, name: editName });
        // showEditForm = false;
        // dispatch("combattenUpdated");
    }
</script>

<div class="combatten-wrapper {combatten_type === 'pc' ? 'pc-type' : 'npc-type'}">
    <ListItem 
        name={initiative !== null ? `${initiative} - ${name}` : name} 
        displayName={name}
        on:remove={remove} 
        on:edit={edit} 
    />
</div>

<style>
    .combatten-wrapper :global(.cyberpunk-item) {
        border-left-color: var(--border-color);
        box-shadow: 0 0 10px var(--glow-color);
    }

    .combatten-wrapper :global(.cyberpunk-item::before) {
        background: linear-gradient(90deg, 
            var(--glow-color) 0%, 
            rgba(0, 0, 0, 0) 20%);
    }

    .pc-type {
        --border-color: #00f3ff;
        --glow-color: rgba(0, 243, 255, 0.2);
    }

    .npc-type {
        --border-color: #ef4444;
        --glow-color: rgba(239, 68, 68, 0.2);
    }
</style>

<!-- This would be the edit form implementation
{#if showEditForm}
    <div class="modal-overlay">
        <div class="modal-content">
            <h3 class="text-lg font-bold mb-4">Edit NPC</h3>
            <input
                type="text"
                bind:value={editName}
                class="input input-bordered w-full mb-4"
            />
            <div class="flex justify-between">
                <button 
                    class="btn btn-outline" 
                    onclick={() => (showEditForm = false)}
                >
                    Cancel
                </button>
                <button 
                    class="btn btn-primary" 
                    onclick={saveEdit}
                >
                    Save
                </button>
            </div>
        </div>
    </div>
{/if}
-->
