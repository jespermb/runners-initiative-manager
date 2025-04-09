<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { link } from "svelte-spa-router";
    import PageLayout from "../../../lib/layout/PageLayout.svelte";
    import BottomNav from "../../../lib/navigation/BottomNav.svelte";
    import EncounterListItem from "../../../lib/EncounterListItem.svelte";
    import CombattenListItem from "../../../lib/CombattenListItem.svelte";
    import type { Combatten } from "../../../types/Combatten";
    import type { Encounter } from "../../../types/Encounter";
    import type { PageProps } from './$types';
    import AddNewCombatten from "$lib/forms/AddNewCombatten.svelte";
    import FabButton from "../../../lib/FabButton.svelte";
    
    let campaign = $state({ name: "", id: 0 });
    let campaignId = $state(0);
    let showEncounterForm = $state(false);
    let showCombattenForm = $state(false);
    let combattens = $state([] as Combatten[]);
    let encounters = $state([] as Encounter[]);
    let combDialog = $state<HTMLDialogElement | null>(null);
    let visibleTab = $state("encounters");
    
    async function getCombattens(id: number) {
        combattens = await invoke("get_all_combattens", { campaignId: id });
    }
    
    async function getEncounters(id: number) {
        encounters = await invoke("get_all_encounters", { campaignId: id });
    }

	let { data }: PageProps = $props();
    campaign = data.campaign;
    campaignId = data.campaign.id;
    encounters = data.encounters;
    combattens = data.combattens;
    
    let encounterName = $state("");
    async function addEncounter() {
        await invoke("add_encounter", {
            name: encounterName,
            campaignId: campaignId,
        });
        encounterName = "";
        getEncounters(campaignId);
        showEncounterForm = false;
    }
    
    let combattenName = $state("");
    async function addCombatten() {
        await invoke("add_combatten", {
            name: combattenName,
            campaignId: campaignId,
        });
        combattenName = "";
        getCombattens(campaignId);
    }
    
    async function combattenRemoved() {
        getCombattens(campaignId);
    }
    
    async function encounterRemoved() {
        getEncounters(campaignId);
    }
    
    function handleTabChange(event: CustomEvent<{ tabId: string }>) {
        visibleTab = event.detail.tabId;
    }
    
    // Navigation tabs configuration
    const tabs = [
        {
            id: "encounters",
            label: "Encounters",
            icon: `<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>`
        },
        {
            id: "combattens",
            label: "Players",
            icon: `<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                  </svg>`
        }
    ];
</script>

<PageLayout>
    <svelte:fragment slot="header">
        <h2 class="header-title">Campaign: {campaign.name}</h2>
        <button 
            class="cyberpunk-btn edit"
            on:click={() => {
                // Edit campaign functionality would go here
                // For now, just a placeholder
                alert('Edit campaign: ' + campaign.name);
            }}
        >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
            </svg>
        </button>
    </svelte:fragment>
    
    <svelte:fragment slot="content">
        {#if visibleTab === "encounters"}
            <div class="flex flex-col gap-2 p-2">
                {#each encounters as encounter}
                <a
                    class="grow text-base leading-9 font-bold"
                    href={"/encounter/" + encounter.id}
                    use:link>
                        <EncounterListItem
                            name={encounter.name}
                            id={encounter.id}
                            on:encounterRemoved={encounterRemoved}
                        />
                </a>
                {/each}
            </div>
        {/if}
        {#if visibleTab === "combattens"}
            <div class="flex flex-col gap-2 p-2">
                {#each combattens as combatten}
                    <CombattenListItem
                        name={combatten.name}
                        id={combatten.id}
                        on:combattenRemoved={combattenRemoved}
                    />
                {/each}
            </div>
        {/if}
    </svelte:fragment>
    
    <svelte:fragment slot="fab">
        <FabButton 
            onClick={() => {
                if (visibleTab === "encounters") {
                    showEncounterForm = true;
                } else {
                    combDialog?.showModal();
                }
            }}
        />
    </svelte:fragment>
    
    <svelte:fragment slot="navigation">
        <BottomNav {tabs} activeTab={visibleTab} on:tabChange={handleTabChange} />
    </svelte:fragment>
</PageLayout>

<!-- Forms and Dialogs -->
{#if showEncounterForm}
    <div class="modal-overlay">
        <div class="modal-content">
            <form on:submit|preventDefault={addEncounter} class="justify-self-end">
                <h3 class="text-lg font-bold mb-4">Add New Encounter</h3>
                <input
                    id="add-encounter"
                    placeholder="Encounter name"
                    bind:value={encounterName}
                    class="input input-bordered w-full mb-4"
                />
                <div class="flex justify-between">
                    <button 
                        type="button" 
                        class="btn btn-outline" 
                        on:click={() => (showEncounterForm = false)}
                    >
                        Cancel
                    </button>
                    <input class="btn btn-primary" type="submit" value="Add" />
                </div>
            </form>
        </div>
    </div>
{/if}

<dialog class="modal cyberpunk-modal" bind:this={combDialog}>
    <div class="modal-box">
        <h3 class="text-lg font-bold mb-4">Add New Player</h3>
        <AddNewCombatten campaignId={campaignId} />
        <div class="modal-action">
            <button class="btn btn-outline" on:click={() => combDialog?.close()}>Close</button>
        </div>
    </div>
</dialog>

<style>
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(0, 0, 0, 0.7);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 30;
        backdrop-filter: blur(3px);
    }

    .modal-content {
        background-color: #0a0a0a;
        color: white;
        padding: 1.5rem;
        border-radius: 0.5rem;
        width: 90%;
        max-width: 500px;
        border: 1px solid #00f3ff;
        box-shadow: 0 0 15px rgba(0, 243, 255, 0.5);
    }

    dialog.cyberpunk-modal::backdrop {
        background-color: rgba(0, 0, 0, 0.7);
        backdrop-filter: blur(3px);
    }

    dialog.cyberpunk-modal .modal-box {
        background-color: #0a0a0a;
        color: white;
        border: 1px solid #00f3ff;
        box-shadow: 0 0 15px rgba(0, 243, 255, 0.5);
    }

    :global(dialog.cyberpunk-modal .btn) {
        background-color: transparent;
        border: 1px solid #00f3ff;
        color: #00f3ff;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    :global(dialog.cyberpunk-modal .btn:hover) {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.7);
    }

    :global(dialog.cyberpunk-modal input) {
        background-color: #1a1a1a;
        border: 1px solid #00f3ff;
        color: white;
    }

    :global(dialog.cyberpunk-modal input:focus) {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
        outline: none;
    }

    .cyberpunk-btn {
        width: 2.5rem;
        height: 2.5rem;
        border-radius: 0.25rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border: 1px solid;
        background-color: transparent;
        transition: all 0.2s ease;
        position: relative;
        z-index: 1;
    }

    .cyberpunk-btn.edit {
        border-color: #00f3ff;
        color: #00f3ff;
    }

    .cyberpunk-btn.edit:hover {
        background-color: rgba(0, 243, 255, 0.1);
        box-shadow: 0 0 8px rgba(0, 243, 255, 0.5);
    }
</style>
