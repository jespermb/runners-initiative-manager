<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import EncCombattenListItem from "../lib/EncCombattenListItem.svelte";
    import CombattenMenu from "../lib/CombattenMenu.svelte";
    import type { Combatten } from "../types/Combatten";
    import type { Encounter } from "../types/Encounter";
    export let params = {
        id: "",
    };
    let encounterId = parseInt(params.id);
    let encounter: Encounter = {
        name: "",
        id: 0,
        campaign_id: 0,
        combattens: []
    };
    let combattens: Combatten[] = [];
    let showMenu = false
    async function getEncounter(id: number) {
        encounter = await invoke("get_encounter", { encounterId: id });
        combattens = encounter.combattens;
    }

    getEncounter(encounterId);
    async function combattenRemoved() {
        getEncounter(encounterId);
    }
    function toggleMenu() {
        console.log("toggleMenu")
    }
</script>

<h2>Encounter: {encounter.name}</h2>

<div class="flex flex-col gap-2 p-2">
    {#each combattens as combatten}
        <EncCombattenListItem
            name={combatten.name}
            id={combatten.id}
            on:combattenRemoved={combattenRemoved}
        />
    {/each}
</div>

<div class="edit-encounter">
    <button on:click={toggleMenu}>+</button>
</div>

{#if showMenu }
    <CombattenMenu />
{/if}
