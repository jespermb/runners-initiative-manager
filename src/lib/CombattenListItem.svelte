<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher } from "svelte";
    import ListItem from "./ListItem.svelte";
    const dispatch = createEventDispatcher();

    const { name = "", id = 0 } = $props();
    let resultMsg = $state("");
    let showEditForm = $state(false);
    let editName = $state(name);

    async function remove() {
        resultMsg = await invoke("remove_combatten", { id });
        dispatch("combattenRemoved");
    }

    function edit() {
        // For now, just show an alert
        // In a real implementation, this would open an edit form
        alert(`Edit player: ${name} (ID: ${id})`);
        
        // Example of how you might implement editing:
        // showEditForm = true;
        // editName = name;
    }

    async function saveEdit() {
        // This would be implemented to save the edited combatten
        // await invoke("update_combatten", { id, name: editName });
        // showEditForm = false;
        // dispatch("combattenUpdated");
    }
</script>

<ListItem {name} on:remove={remove} on:edit={edit} />

<!-- This would be the edit form implementation
{#if showEditForm}
    <div class="modal-overlay">
        <div class="modal-content">
            <h3 class="text-lg font-bold mb-4">Edit Player</h3>
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
