<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher } from "svelte";
    import ListItem from "./ListItem.svelte";
    const dispatch = createEventDispatcher();

    const { name = "", id = 0, campaign_id = 0, encounter_id = 0 } = $props();
    let resultMsg = $state("");

    async function add() {
        resultMsg = await invoke("add_combatten_to_encounter", { campaign_id, encounter_id, id });
        dispatch("combattenAdded");
    }
</script>

<ListItem {name} on:add={add} />
