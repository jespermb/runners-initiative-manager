<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher, onMount } from "svelte";
    import type { Combatten } from "../../types/Combatten";
    import EncounterCombattenItem from "../EncounterCombattenItem.svelte";
    const dispatch = createEventDispatcher();

    const { campaign_id = 0, encounter_id = 0, id = 0 } = $props();
    let displayName = $state("");
    let resultMsg = $state("");
    let combattens = $state([] as Combatten[]);

    async function add() {
        // Learn more about Tauri commands at https://tauri.app/v2/guide/command
        resultMsg = await invoke("remove_combatten", { id });
        displayName = "";
        dispatch("combattenRemoved");
    }

    // Use onMount for async initialization
    onMount(async () => {
        combattens = await invoke("get_all_campaign_combattens", { campaign_id, encounter_id });
        console.log(combattens);
    });
</script>

{#each combattens as combatten}
  <EncounterCombattenItem name={combatten.name} id={combatten.id} on:click={add} />
{/each}
