<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { link } from "svelte-spa-router";
    import EncounterListItem from "../../../lib/EncounterListItem.svelte";
    import CombattenListItem from "../../../lib/CombattenListItem.svelte";
    import type { Combatten } from "../../../types/Combatten";
    import type { Encounter } from "../../../types/Encounter";
    import type { PageProps } from './$types';
    
    let campaign = $state({ name: "", id: 0 });
    let campaignId = $state(0);
    let showEncounterForm = $state(false);
    let showCombattenForm = $state(false);
    let combattens = $state([] as Combatten[]);
    let encounters = $state([] as Encounter[]);
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
    }
    let combattenName = $state("");
    let visibleTab = $state("encounters");
    async function addCombatten() {
        await invoke("add_combatten", {
            name: combattenName,
            campaignId: campaignId,
        });
        combattenName = "";
        combattens = await invoke("get_all_combattens", {
            campaignId: campaignId,
        });
    }
    async function combattenRemoved() {
        getCombattens(campaignId);
    }
    async function encounterRemoved() {
        getEncounters(campaignId);
    }
</script>

<h2 class="mb-3">Campaign: {campaign.name}</h2>
<ul
    class="flex flex-wrap text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:border-gray-700 dark:text-gray-400"
>
    <li class="mr-2">
        <div
            on:click={() => {
                visibleTab = "encounters";
            }}
            on:keypress={() => {
                visibleTab = "encounters";
            }}
            role="button"
            tabindex="0"
            aria-current="page"
            class={`inline-block p-2 rounded hover:text-gray-600 hover:bg-gray-50 dark:hover:bg-gray-800 dark:hover:text-gray-300${
                visibleTab === "encounters"
                    ? " active dark:bg-gray-800 text-white-500"
                    : "dark:hover:bg-gray-800 dark:hover:text-gray-300"
            }`}
        >
            Encounters
        </div>
    </li>
    <li class="mr-2">
        <div
            on:click={() => {
                visibleTab = "combattens";
            }}
            on:keypress={() => {
                visibleTab = "combattens";
            }}
            role="button"
            tabindex="0"
            class={`inline-block p-2 rounded hover:text-gray-600 hover:bg-gray-50 dark:hover:bg-gray-800 dark:hover:text-gray-300${
                visibleTab === "combattens"
                    ? " active dark:bg-gray-800 text-white-500"
                    : "dark:hover:bg-gray-800 dark:hover:text-gray-300"
            }`}
        >
            Combattens
        </div>
    </li>
</ul>
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
{#if showEncounterForm}
    <form on:submit={addEncounter} class="justify-self-end">
        <input
            id="add-encounter"
            placeholder="Add encounter"
            bind:value={encounterName}
            class="input input-bordered w-full max-w-xs"
        />
        <input class="btn btn-primary" type="submit" value="Add" />
    </form>
{/if}
{#if showCombattenForm}
    <form on:submit={addCombatten} class="justify-self-end">
        <input
            id="add-encounter"
            placeholder="Add combatten"
            bind:value={combattenName}
            class="input input-bordered w-full max-w-xs"
        />
        <input class="btn btn-primary" type="submit" value="Add" />
    </form>
{/if}
<div class="btm-nav flex gap-4">
    <div class="btm-nav-label">
        <button
            class="btn btn-sm btn-outline"
            on:click={() => (showEncounterForm = !showEncounterForm)}
            >Add encounter</button
        >
    </div>
    <div class="btm-nav-label">
        <button
            class="btn btn-sm btn-outline"
            on:click={() => (showCombattenForm = !showCombattenForm)}
            >Add combatten</button
        >
    </div>
</div>

<style>
    .btm-nav {
        background: none;
    }
</style>
