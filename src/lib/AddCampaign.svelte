<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    let name = "";
    let version = 6;
    let greetMsg = "";

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        greetMsg = await invoke("add_campaign", { name, version });
        name = "";
        version = 6;
        dispatch("campaignAdded");
    }
</script>

<div class="">
    <form class="flex gap-4" on:submit|preventDefault={greet}>
        <input
            id="campaign-name"
            placeholder="Enter a name..."
            bind:value={name}
            class="input input-bordered w-full max-w-xs"
        />
        <input
            id="game-version"
            placeholder="Set the version"
            bind:value={version}
            class="input input-bordered w-full max-w-xs"
        />
        <button type="submit" class="btn btn-primary">Save</button>
    </form>
    <p>{greetMsg}</p>
</div>
