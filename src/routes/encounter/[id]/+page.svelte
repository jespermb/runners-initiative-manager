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
    import FabButton from "../../../lib/FabButton.svelte";
    
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
    let visibleTab = $state("combattens");
    
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

<div class="cyberpunk-container">
    <div class="cyberpunk-header">
        <h2 class="header-title">Encounter: {encounter.name}</h2>
        <button 
            class="cyberpunk-btn edit"
            onclick={() => {
                // Edit encounter functionality would go here
                // For now, just a placeholder
                alert('Edit encounter: ' + encounter.name);
            }}
        >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
            </svg>
        </button>
    </div>

    <div class="content-container">
        {#if visibleTab === "combattens"}
            <div class="flex flex-col gap-2 p-2">
                {#each combattens as combatten}
                    <EncCombattenListItem
                        name={combatten.name}
                        id={combatten.id}
                        on:combattenRemoved={combattenRemoved}
                    />
                {/each}
            </div>
        {/if}
        {#if visibleTab === "stats"}
            <div class="flex flex-col gap-2 p-2">
                <p class="text-white">Encounter statistics will be shown here.</p>
            </div>
        {/if}
    </div>

    <!-- Floating Action Button -->
    <FabButton onClick={toggleMenu} />

    <!-- Bottom Navigation -->
    <div class="cyberpunk-nav">
        <button 
            class={`cyberpunk-tab ${visibleTab === "combattens" ? "active" : ""}`}
            onclick={() => { visibleTab = "combattens"; }}
        >
            <div class="tab-content">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                </svg>
                <span class="tab-label">Combattens</span>
            </div>
        </button>
        <button 
            class={`cyberpunk-tab ${visibleTab === "stats" ? "active" : ""}`}
            onclick={() => { visibleTab = "stats"; }}
        >
            <div class="tab-content">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                </svg>
                <span class="tab-label">Stats</span>
            </div>
        </button>
    </div>
</div>

<!-- Menu Options -->
{#if showMenu}
    <div class="menu-overlay" transition:slide={{ delay: 100, duration: 250, axis: "y", easing: sineInOut }}>
        <div class="menu-content">
            <h3 class="text-lg font-bold mb-4">Add Combatant</h3>
            <div class="flex flex-col gap-3">
                <button 
                    class="btn btn-primary" 
                    onclick={showAddNewCombattenMenu}
                >
                    Create New NPC
                </button>
                <button 
                    class="btn btn-secondary" 
                    onclick={showAddCombattenMenu}
                >
                    Add Existing NPC
                </button>
                <button 
                    class="btn btn-outline mt-2" 
                    onclick={toggleMenu}
                >
                    Cancel
                </button>
            </div>
        </div>
    </div>
{/if}

<!-- Form for adding a new combatant -->
{#if showNewCombattenMenu}
    <div class="modal-overlay" transition:slide={{ delay: 100, duration: 250, axis: "y", easing: sineInOut }}>
        <div class="modal-content">
            <h3 class="text-lg font-bold mb-4">Create New NPC</h3>
            <AddNewCombatten 
                on:combattenAdded={combattenRemoved} 
                encounterId={encounterId} 
                campaignId={encounter.campaign_id} 
            />
            <div class="flex justify-end mt-4">
                <button 
                    class="btn btn-outline" 
                    onclick={() => showNewCombattenMenu = false}
                >
                    Cancel
                </button>
            </div>
        </div>
    </div>
{/if}

<!-- Form for adding an existing combatant -->
{#if showCombattenMenu}
    <div class="modal-overlay" transition:slide={{ delay: 100, duration: 250, axis: "y", easing: sineInOut }}>
        <div class="modal-content">
            <h3 class="text-lg font-bold mb-4">Add Existing NPC</h3>
            <AddEncounterCombatten 
                campaign_id={encounter.campaign_id} 
                encounter_id={encounterId} 
            />
            <div class="flex justify-end mt-4">
                <button 
                    class="btn btn-outline" 
                    onclick={() => showCombattenMenu = false}
                >
                    Cancel
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .cyberpunk-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        position: relative;
    }

    .cyberpunk-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        background-color: #0a0a0a;
        border-bottom: 1px solid #00f3ff;
        margin-bottom: 1rem;
        position: relative;
        overflow: hidden;
    }

    .cyberpunk-header::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: linear-gradient(90deg, 
            rgba(0, 243, 255, 0.1) 0%, 
            rgba(0, 0, 0, 0) 50%,
            rgba(255, 0, 255, 0.1) 100%);
        z-index: 0;
    }

    .header-title {
        color: white;
        font-size: 1.5rem;
        font-weight: 700;
        margin: 0;
        text-transform: uppercase;
        letter-spacing: 1px;
        position: relative;
        z-index: 1;
        text-shadow: 0 0 5px rgba(0, 243, 255, 0.7);
    }

    .content-container {
        flex: 1;
        overflow-y: auto;
        max-height: calc(100vh - 15rem);
        margin-bottom: 1rem;
    }

    /* Cyberpunk Navigation Styling */
    .cyberpunk-nav {
        display: flex;
        background-color: #0a0a0a;
        border-top: 1px solid #00f3ff;
        z-index: 10;
        height: 4rem;
        box-shadow: 0 -5px 15px rgba(0, 243, 255, 0.2);
        margin: 0 -1rem -0.5rem -1rem; /* Negative margin to extend to edges */
        border-bottom-left-radius: 0.5rem;
        border-bottom-right-radius: 0.5rem;
    }

    .cyberpunk-tab {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: transparent;
        border: none;
        color: #c0c0c0;
        position: relative;
        overflow: hidden;
        transition: all 0.3s ease;
    }

    .cyberpunk-tab::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 2px;
        background: linear-gradient(90deg, #ff00ff, #00f3ff);
        transform: translateY(-100%);
        transition: transform 0.3s ease;
    }

    .cyberpunk-tab.active {
        color: #ffffff;
    }

    .cyberpunk-tab.active::before {
        transform: translateY(0);
    }

    .cyberpunk-tab.active {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5), 
                    0 0 20px rgba(0, 243, 255, 0.3), 
                    0 0 30px rgba(0, 243, 255, 0.1);
    }

    .tab-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 0.5rem 0;
        z-index: 1;
    }

    .tab-label {
        font-size: 0.75rem;
        margin-top: 0.25rem;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    /* Cyberpunk Modal Styling */
    .menu-overlay, .modal-overlay {
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

    .menu-content, .modal-content {
        background-color: #0a0a0a;
        color: white;
        padding: 1.5rem;
        border-radius: 0.5rem;
        width: 90%;
        max-width: 500px;
        border: 1px solid #00f3ff;
        box-shadow: 0 0 15px rgba(0, 243, 255, 0.5);
    }

    /* Cyberpunk Button Styling */
    :global(.menu-content .btn, .modal-content .btn) {
        background-color: transparent;
        border: 1px solid #00f3ff;
        color: #00f3ff;
        text-transform: uppercase;
        letter-spacing: 1px;
        position: relative;
        overflow: hidden;
    }

    :global(.menu-content .btn-primary, .modal-content .btn-primary) {
        background-color: #00f3ff;
        color: #0a0a0a;
    }

    :global(.menu-content .btn-secondary, .modal-content .btn-secondary) {
        background-color: #ff00ff;
        border-color: #ff00ff;
        color: #0a0a0a;
    }

    :global(.menu-content .btn:hover, .modal-content .btn:hover) {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.7);
    }

    :global(.menu-content .btn-primary:hover, .modal-content .btn-primary:hover) {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.7);
    }

    :global(.menu-content .btn-secondary:hover, .modal-content .btn-secondary:hover) {
        box-shadow: 0 0 10px rgba(255, 0, 255, 0.7);
    }

    /* Cyberpunk Input Styling */
    :global(.modal-content input) {
        background-color: #1a1a1a;
        border: 1px solid #00f3ff;
        color: white;
    }

    :global(.modal-content input:focus) {
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
