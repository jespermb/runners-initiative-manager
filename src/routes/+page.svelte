<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { link } from "svelte-spa-router";
    import AddCampaign from "../lib/forms/AddCampaign.svelte";
    import type { Campaign } from "../types/Campaign";

    let campaigns = $state([] as Campaign[]);

    async function getCampaigns() {
        // Learn more about Tauri commands at https://tauri.app/v2/guide/command
        campaigns = await invoke("get_all_campaigns");
    }

    async function removeCampaign(id: number) {
        // Learn more about Tauri commands at https://tauri.app/v2/guide/command
        await invoke("remove_campaign", { id });
        await getCampaigns();
    }

    getCampaigns();
</script>

<h2 class="mt-4">Campaigns</h2>
<div class="grow pt-2">
    {#each campaigns as campaign}
        <div class="flex flex-row w-full">
            <a class="grow text-base leading-9" href={"/campaign/" + campaign.id}><span class="font-bold">{campaign.name}</span> ({campaign.version}th edition)</a>
            <button
                class="border-rounded bg-red-500 hover:bg-red-600"
                style="padding: 0 3px; margin: 3px 0;"
                on:click={() => removeCampaign(campaign.id)}
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    style="height: 24px; width: 24px;"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    ><path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M6 18L18 6M6 6l12 12"
                    /></svg
                >
            </button>
        </div>
    {/each}
</div>
<AddCampaign on:campaignAdded={getCampaigns} />
