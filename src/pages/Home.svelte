<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { link } from "svelte-spa-router";
    import AddCampaign from "../lib/AddCampaign.svelte";
    import type { Campaign } from "../types/Campaign";

    let campaigns: Campaign[] = [];

    async function getCampaigns() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        campaigns = await invoke("get_all_campaigns");
    }

    async function removeCampaign(id) {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        await invoke("remove_campaign", { id });
        await getCampaigns();
    }

    getCampaigns();
</script>

<h2 class="mt-4">Campaigns</h2>
<div class="grow pt-2">
    {#each campaigns as campaign}
        <div class="flex flex-row w-full">
            <a
                class="grow text-base leading-9 font-bold"
                href={"/campaign/" + campaign.id}
                use:link>{campaign.name}</a
            >
            <button
                class="border-rounded bg-red-500 hover:bg-red-600"
                style="padding: 0.3rem 0.4rem;"
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

<style>
    .logo.vite:hover {
        filter: drop-shadow(0 0 2em #747bff);
    }

    .logo.svelte:hover {
        filter: drop-shadow(0 0 2em #ff3e00);
    }
    .list {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }
    .row {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 1em;
    }
    .row p {
        flex-grow: 1;
    }
    .row button {
        justify-self: flex-end;
    }
</style>
