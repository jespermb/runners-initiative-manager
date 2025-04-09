<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    
    // Define the tab type
    type Tab = {
        id: string;
        label: string;
        icon: string; // SVG path or component reference
    };
    
    // Props
    export let tabs: Tab[] = [];
    export let activeTab: string = tabs.length > 0 ? tabs[0].id : '';
    
    // Event dispatcher
    const dispatch = createEventDispatcher<{
        tabChange: { tabId: string }
    }>();
    
    // Handle tab click
    function handleTabClick(tabId: string) {
        activeTab = tabId;
        dispatch('tabChange', { tabId });
    }
</script>

<div class="cyberpunk-nav">
    {#each tabs as tab}
        <button 
            class={`cyberpunk-tab ${activeTab === tab.id ? "active" : ""}`}
            on:click={() => handleTabClick(tab.id)}
        >
            <div class="tab-content">
                {@html tab.icon}
                <span class="tab-label">{tab.label}</span>
            </div>
        </button>
    {/each}
</div>

<style>
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
</style>
