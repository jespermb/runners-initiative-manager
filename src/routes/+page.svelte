<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { link } from "svelte-spa-router";
    import PageLayout from "../lib/layout/PageLayout.svelte";
    import CampaignListItem from "../lib/CampaignListItem.svelte";
    import FabButton from "../lib/FabButton.svelte";
    import AddCampaign from "../lib/forms/AddCampaign.svelte";
    import type { Campaign } from "../types/Campaign";

    let campaigns = $state([] as Campaign[]);
    let showAddForm = $state(false);
    let isTauriAvailable = $state(false);
    let isLoading = $state(true);

    // Check if Tauri is available
    async function checkTauriAvailability() {
        try {
            // Try to access the window.__TAURI__ object
            isTauriAvailable = typeof window !== 'undefined' && 'Tauri' in window;
            if (isTauriAvailable) {
                await getCampaigns();
            } else {
                // If Tauri is not available, use mock data for development
                campaigns = [];
            }
        } catch (error) {
            console.error("Error checking Tauri availability:", error);
            isTauriAvailable = false;
            // Use mock data
            campaigns = [];
        } finally {
            isLoading = false;
        }
    }

    async function getCampaigns() {
        if (!isTauriAvailable) return;
        try {
            // Learn more about Tauri commands at https://tauri.app/v2/guide/command
            campaigns = await invoke("get_all_campaigns");
        } catch (error) {
            console.error("Error fetching campaigns:", error);
        }
    }
    
    // Mock implementation of add_campaign for browser mode
    async function addCampaign(name: string, version: number) {
        if (!isTauriAvailable) {
            // Generate a unique ID (in a real app, this would be handled by the backend)
            const newId = Math.max(0, ...campaigns.map(c => c.id)) + 1;
            // Add the new campaign to the array
            campaigns = [...campaigns, { id: newId, name, version }];
            return `Added campaign "${name}"`;
        }
        
        try {
            return await invoke("add_campaign", { name, version });
        } catch (error) {
            console.error("Error adding campaign:", error);
            return null;
        }
    }

    async function removeCampaign(id: number) {
        if (!isTauriAvailable) {
            // In browser mode, just filter out the campaign
            campaigns = campaigns.filter(c => c.id !== id);
            return;
        }
        
        try {
            // Learn more about Tauri commands at https://tauri.app/v2/guide/command
            await invoke("remove_campaign", { id });
            await getCampaigns();
        } catch (error) {
            console.error("Error removing campaign:", error);
        }
    }

    function handleCampaignRemoved(event: CustomEvent<{ id: number }>) {
        removeCampaign(event.detail.id);
    }

    // Initialize
    checkTauriAvailability();
</script>

<PageLayout>
    <svelte:fragment slot="header">
        <h2 class="header-title">Campaigns</h2>
    </svelte:fragment>
    
    <svelte:fragment slot="content">
        <div class="flex flex-col gap-2 p-2">
            {#each campaigns as campaign}
                <a 
                    class="campaign-link"
                    href={"/campaign/" + campaign.id}
                    use:link
                >
                    <CampaignListItem 
                        {campaign} 
                        on:campaignRemoved={handleCampaignRemoved}
                    />
                </a>
            {/each}
        </div>
    </svelte:fragment>
    
    <svelte:fragment slot="fab">
        <FabButton onClick={() => showAddForm = true} />
    </svelte:fragment>
</PageLayout>

<!-- Add Campaign Form Modal -->
{#if showAddForm}
    <div class="modal-overlay">
        <div class="modal-content">
            <h3 class="text-lg font-bold mb-4">Add New Campaign</h3>
            <AddCampaign 
                on:campaignAdded={async (event) => {
                    if (!isTauriAvailable) {
                        // If we're in browser mode, we need to manually add the campaign
                        // Extract the name and version from the event if available, or use defaults
                        const name = event.detail?.name || "New Campaign";
                        const version = event.detail?.version || 6;
                        await addCampaign(name, version);
                    } else {
                        // In Tauri mode, the campaign was already added by the AddCampaign component
                        await getCampaigns();
                    }
                    showAddForm = false;
                }} 
            />
            <div class="flex justify-end mt-4">
                <button 
                    class="btn btn-outline" 
                    on:click={() => showAddForm = false}
                >
                    Cancel
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .campaign-link {
        text-decoration: none;
        color: inherit;
        display: block;
    }
    
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

    :global(.btn) {
        background-color: transparent;
        border: 1px solid #00f3ff;
        color: #00f3ff;
        text-transform: uppercase;
        letter-spacing: 1px;
        padding: 0.5rem 1rem;
        border-radius: 0.25rem;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    :global(.btn:hover) {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.7);
    }
</style>
