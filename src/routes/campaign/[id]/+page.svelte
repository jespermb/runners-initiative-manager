<script lang="ts">
    import PageLayout from "../../../lib/layout/PageLayout.svelte";
    import BottomNav from "../../../lib/navigation/BottomNav.svelte";
    import EncounterListItem from "../../../lib/EncounterListItem.svelte";
    import CombattenListItem from "../../../lib/CombattenListItem.svelte";
    import type { PageProps } from './$types';
    import AddNewCombatten from "$lib/forms/AddNewCombatten.svelte";
    import AddNewEncounter from "$lib/forms/AddNewEncounter.svelte";
    import FabButton from "../../../lib/FabButton.svelte";
    import PageHeader from "../../../lib/ui/PageHeader.svelte";
    import TabContentContainer from "../../../lib/ui/TabContentContainer.svelte";
    import CyberpunkModal from "../../../lib/ui/CyberpunkModal.svelte";
    import { createCampaignDataManager } from "../../../lib/composables/CampaignDataManager";
    
    let { data }: PageProps = $props();
    
    const campaign = data.campaign;
    const campaignId = data.campaign.id;
    const dataManager = createCampaignDataManager(campaignId);
    
    // Initialize data from server props
    let encounters = $state([...data.encounters]);
    let combattens = $state([...data.combattens]);
    
    // Get only PC combattens for the Players tab
    const pcCombattens = $derived(combattens.filter(c => c.combatten_type === 'pc'));
    
    let showEncounterModal = $state(false);
    let showCombattenModal = $state(false);
    let visibleTab = $state("encounters");
    
    async function handleEncounterSaved() {
        encounters = await dataManager.getEncounters();
        showEncounterModal = false;
    }
    
    async function handleCombattenSaved() {
        combattens = await dataManager.getCombattens();
        showCombattenModal = false;
    }
    
    async function handleCombattenRemoved() {
        combattens = await dataManager.getCombattens();
    }
    
    async function handleEncounterRemoved() {
        encounters = await dataManager.getEncounters();
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
        <PageHeader
            title={campaign.name}
            backUrl="/"
            onEdit={() => alert('Edit campaign: ' + campaign.name)}
            editLabel="Edit Campaign"
        />
    </svelte:fragment>
    
    <svelte:fragment slot="content">
        {#if visibleTab === "encounters"}
            <div class="flex flex-col gap-2 py-4">
                {#each encounters as encounter}
                    <a class="grow text-base leading-9 font-bold" href={"/encounter/" + encounter.id}>
                        <EncounterListItem
                            name={encounter.name}
                            id={encounter.id}
                            on:encounterRemoved={handleEncounterRemoved}
                        />
                    </a>
                {/each}
            </div>
        {/if}
        {#if visibleTab === "combattens"}
            <div class="flex flex-col gap-2 py-4">
                {#each pcCombattens as combatten}
                    <CombattenListItem
                        {combatten}
                        on:combattenRemoved={handleCombattenRemoved}
                    />
                {/each}
            </div>
        {/if}
    </svelte:fragment>
    
    <svelte:fragment slot="fab">
        <FabButton 
            onClick={() => {
                if (visibleTab === "encounters") {
                    showEncounterModal = true;
                } else {
                    showCombattenModal = true;
                }
            }}
        />
    </svelte:fragment>
    
    <svelte:fragment slot="navigation">
        <BottomNav {tabs} activeTab={visibleTab} on:tabChange={handleTabChange} />
    </svelte:fragment>
</PageLayout>

<CyberpunkModal
    title="Add New Encounter"
    isOpen={showEncounterModal}
    onClose={() => showEncounterModal = false}
    size="lg"
>
    <AddNewEncounter {campaignId} onSave={handleEncounterSaved} />
</CyberpunkModal>

<CyberpunkModal
    title="Add New Player"
    isOpen={showCombattenModal}
    onClose={() => showCombattenModal = false}
    size="lg"
>
    <AddNewCombatten {campaignId} onSave={handleCombattenSaved} />
</CyberpunkModal>

