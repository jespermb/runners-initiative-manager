<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher } from "svelte";
    import ListItem from "./ListItem.svelte";
    import EditInitiativeModal from "./forms/EditInitiativeModal.svelte";
    const dispatch = createEventDispatcher();

    const { name = "", id = 0, initiative = null, combatten_type = "npc", encounter_id = 0, physical = 0, stun = 0 } = $props();
    let resultMsg = $state("");
    let showEditModal = $state(false);

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
        showEditModal = true;
    }

    function handleEditClose() {
        showEditModal = false;
    }

    function handleEditSaved() {
        // Refresh the encounter to show updated initiative
        dispatch("combattenRemoved"); // Reusing this event to trigger refresh
    }
</script>

<div class="combatten-wrapper {combatten_type === 'pc' ? 'pc-type' : 'npc-type'}">
    <ListItem 
        name={initiative !== null ? `${initiative} - ${name}` : name} 
        displayName={name}
        {physical}
        {stun}
        on:remove={remove} 
        on:edit={edit} 
    />
</div>

<EditInitiativeModal
    combattenId={id}
    combattenName={name}
    currentInitiative={initiative || 0}
    encounterId={encounter_id}
    isOpen={showEditModal}
    onClose={handleEditClose}
    onSaved={handleEditSaved}
/>

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

