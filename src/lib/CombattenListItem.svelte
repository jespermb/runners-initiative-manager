<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher } from "svelte";
    import ListItem from "./ListItem.svelte";
    import EditCombattentModal from "./forms/EditCombattentModal.svelte";
    import type { Combatten } from "../types/Combatten";
    const dispatch = createEventDispatcher();

    const { combatten }: { combatten: Combatten } = $props();
    let resultMsg = $state("");
    let showEditModal = $state(false);

    async function remove() {
        resultMsg = await invoke("remove_combatten", { id: combatten.id });
        dispatch("combattenRemoved");
    }

    function edit() {
        showEditModal = true;
    }

    function handleEditClose() {
        showEditModal = false;
    }

    function handleEditSaved() {
        // Refresh the campaign to show updated combatten
        dispatch("combattenRemoved"); // Reusing this event to trigger refresh
    }
</script>

<ListItem 
    name={combatten.name} 
    physical={combatten.physical} 
    stun={combatten.stun} 
    on:remove={remove} 
    on:edit={edit} 
/>

<EditCombattentModal
    {combatten}
    isOpen={showEditModal}
    onClose={handleEditClose}
    onSaved={handleEditSaved}
/>

