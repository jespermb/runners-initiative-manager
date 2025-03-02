<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { slide } from 'svelte/transition';
	import { sineInOut } from 'svelte/easing';
    import EncCombattenListItem from "../../../lib/EncCombattenListItem.svelte";
    import CombattenMenu from "../../../lib/CombattenMenu.svelte";
    import type { Combatten } from "../../../types/Combatten";
    import type { Encounter } from "../../../types/Encounter";
    import AddNewCombatten from "../../../lib/forms/AddNewCombatten.svelte";
    import AddEncounterCombatten from "../../../lib/forms/AddEncounterCombatten.svelte";
    
    const { id = "" } = $props();
    let encounterId = $state(parseInt(id));
    let encounter = $state<Encounter>({
        name: "",
        id: 0,
        campaign_id: 0,
        combattens: []
    });
    let combattens = $state([] as Combatten[]);
    let showMenu = $state(false);
    let showCombattenMenu = $state(false);
    let showNewCombattenMenu = $state(false);
    
    async function getEncounter(id: number) {
        encounter = await invoke("get_encounter", { id: id });
        combattens = encounter.combattens;
    }

    getEncounter(encounterId);
    async function combattenRemoved() {
        getEncounter(encounterId);
    }
    function toggleMenu() {
        console.log("toggleMenu")
        showMenu = !showMenu
    }
    function showAddNewCombattenMenu() {
        showMenu = false
        showNewCombattenMenu = true
    }
    function showAddCombattenMenu() {
        showMenu = false
        showCombattenMenu = true
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
    <div transition:slide={{ delay: 100, duration: 250, axis: "y", easing: sineInOut }}>
        <CombattenMenu on:addNewCombatten={showAddNewCombattenMenu} on:addCombatten={showAddCombattenMenu} />
    </div>
{/if}

{#if showNewCombattenMenu }
    <div transition:slide={{ delay: 100, duration: 250, axis: "y", easing: sineInOut }}>
        <AddNewCombatten on:combattenAdded={combattenRemoved} encounterId={encounterId} campaignId={encounter.campaign_id} />
    </div>
{/if}

{#if showCombattenMenu }
    <div transition:slide={{ delay: 100, duration: 250, axis: "y", easing: sineInOut }}>
        <AddEncounterCombatten campaign_id={encounter.campaign_id} encounter_id={encounterId} />
    </div>
{/if}

<style>
    .edit-encounter {
        position: fixed;
        bottom: 30px;
        right: 30px;
        padding: 10px;
        border-radius: 26px;
        background-color: #fff;
    }
    .edit-encounter button {
        background-color: transparent;
        border-width: 0;
        font-family: inherit;
        font-size: inherit;
        font-style: inherit;
        font-weight: inherit;
        line-height: inherit;
        font-size: 1rem;
        width: 1.5rem;
        height: 1.5rem;
        color: #000;
        display: inline-block;
    }
</style>
