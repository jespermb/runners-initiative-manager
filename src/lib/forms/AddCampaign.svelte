<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    let name = $state("");
    let version = $state(6);
    let greetMsg = $state("");

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v2/guide/command
        const campaignVersion = parseInt(`${version}`);
        greetMsg = await invoke("add_campaign", { name, version: campaignVersion });
        name = "";
        version = 6;
        dispatch("campaignAdded");
    }
</script>

<div class="">
    {#if greetMsg}
        <div class="alert alert-success p-1">{greetMsg}</div>
    {/if}
    <form class="flex gap-4" on:submit|preventDefault={greet}>
        <div class="flex gap-2 flex-col">
            <label for="campaign-name">Campaign Name</label>
            <input
                id="campaign-name"
                placeholder="Enter a name..."
                bind:value={name}
                class="input input-bordered w-full max-w-xs"
            />
        </div>
        <div class="flex gap-2 flex-col">
            <label for="game-version">Game Version</label>
            <select
                name="version"
                id="game-version"
                placeholder="Set the version"
                bind:value={version}
                class="input input-bordered w-full max-w-xs"
            >
                <option selected={(version == 6) ? true : false} value="6">6th edition</option>
                <option selected={(version == 5) ? true : false} value="5">5th edition</option>
                <option selected={(version == 4) ? true : false} value="4">4th edition</option>
            </select>
        </div>
        <button type="submit" class="btn btn-primary self-end">Save</button>
    </form>
</div>
