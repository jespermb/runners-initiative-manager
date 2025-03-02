<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    let name = $state("");
    let initiative = $state(0);
    let physical = $state(0);
    let stun = $state(0);
    let greetMsg = $state("");

    const { campaignId = 0, encounterId = 0 } = $props();

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v2/guide/command
        const combattenId = await invoke("add_combatten", { name, physical, stun, campaignId });
        greetMsg = await invoke("add_combatten_to_encounter", { encounterId, initiative, combattenId });
        name = "";
        initiative = 0;
        physical = 0;
        stun = 0;
        dispatch("combattenAdded");
    }
</script>

<div class="">
    <form class="flex gap-4" on:submit|preventDefault={greet}>
        <input
            id="combatten-name"
            placeholder="Enter a name"
            bind:value={name}
            class="input input-bordered w-full max-w-xs"
        />
        <input
            id="combatten-initiative"
            placeholder="Enter initiative"
            bind:value={initiative}
            class="input input-bordered w-full max-w-xs"
        />
        <input
            id="combatten-physical"
            placeholder="Enter physical health"
            bind:value={physical}
            class="input input-bordered w-full max-w-xs"
        />
        <input
            id="combatten-stun"
            placeholder="Enter stun health"
            bind:value={stun}
            class="input input-bordered w-full max-w-xs"
        />
        <button type="submit" class="btn btn-primary">Save</button>
    </form>
    <p>{greetMsg}</p>
</div>
