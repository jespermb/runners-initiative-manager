<script lang="ts">
    import type { Combatten } from "../../types/Combatten";
    import SearchableList from "../ui/SearchableList.svelte";
    import NewCombattentForm from "./NewCombattentForm.svelte";

    interface Props {
        addType: string;
        combattens: Combatten[];
        filteredCombattens: Combatten[];
        searchQuery: string;
        selectedCombattenId: number | null;
        name: string;
        combatantType: string;
        physical: number;
        stun: number;
        errors: Record<string, string[]>;
        onAddTypeChange: (type: string) => void;
        onSearchChange: (query: string) => void;
        onSelectionChange: (id: number) => void;
        onNameChange: (value: string) => void;
        onPhysicalChange: (value: number) => void;
        onStunChange: (value: number) => void;
        onTypeChange: (type: string) => void;
    }

    const { 
        addType,
        combattens,
        filteredCombattens,
        searchQuery,
        selectedCombattenId,
        name,
        combatantType,
        physical,
        stun,
        errors,
        onAddTypeChange,
        onSearchChange,
        onSelectionChange,
        onNameChange,
        onPhysicalChange,
        onStunChange,
        onTypeChange
    }: Props = $props();

    function handleAddTypeChange(type: string) {
        onAddTypeChange(type);
    }
</script>

<div class="select-combattent-step">
    <!-- Toggle between new and existing -->
    <div class="cyberpunk-tabs mb-4">
        <button 
            type="button"
            class={`cyberpunk-tab ${addType === 'existing' ? 'active' : ''}`} 
            onclick={() => handleAddTypeChange('existing')}
        >
            <div class="tab-content">
                <span class="tab-label">Existing</span>
            </div>
        </button>
        <button 
            type="button"
            class={`cyberpunk-tab ${addType === 'new' ? 'active' : ''}`} 
            onclick={() => handleAddTypeChange('new')}
        >
            <div class="tab-content">
                <span class="tab-label">Create New</span>
            </div>
        </button>
    </div>
    
    {#if addType === "new"}
        <NewCombattentForm
            {name}
            {combatantType}
            {physical}
            {stun}
            {errors}
            {onNameChange}
            {onPhysicalChange}
            {onStunChange}
            {onTypeChange}
        />
    {:else}
        <SearchableList
            items={filteredCombattens}
            {searchQuery}
            selectedId={selectedCombattenId}
            emptyMessage="No combatants found"
            {onSearchChange}
            {onSelectionChange}
            errors={errors.selectedCombattenId || []}
        />
    {/if}
</div>

<style>
    .select-combattent-step {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .cyberpunk-tabs {
        display: flex;
        background-color: #0a0a0a;
        border: 1px solid #00f3ff;
        overflow: hidden;
        position: relative;
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
        margin-bottom: 0.25rem;
    }

    .cyberpunk-tabs::before {
        content: '';
        position: absolute;
        top: -2px;
        left: -2px;
        right: -2px;
        height: 2px;
        background: linear-gradient(90deg, #ff00ff, #00f3ff);
        animation: cyberpunk-scan 2s linear infinite;
    }

    @keyframes cyberpunk-scan {
        0% {
            transform: translateY(0);
        }
        100% {
            transform: translateY(calc(100% + 4px));
        }
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
        padding: 0.5rem 0;
        cursor: pointer;
        clip-path: polygon(
            0% 0%, 
            calc(100% - 10px) 0%, 
            100% 50%, 
            calc(100% - 10px) 100%, 
            0% 100%
        );
    }

    .cyberpunk-tab:last-child {
        clip-path: polygon(
            10px 0%, 
            100% 0%, 
            100% 100%, 
            10px 100%, 
            0% 50%
        );
    }

    .cyberpunk-tab::after {
        content: '';
        position: absolute;
        top: 0;
        right: -10px;
        width: 20px;
        height: 100%;
        background-color: #0a0a0a;
        border-right: 1px solid #00f3ff;
        transform: skewX(-20deg);
        z-index: 1;
    }

    .cyberpunk-tab:last-child::after {
        display: none;
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
        background-color: rgba(0, 243, 255, 0.1);
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5) inset, 
                    0 0 20px rgba(0, 243, 255, 0.3) inset, 
                    0 0 30px rgba(0, 243, 255, 0.1) inset;
    }

    .cyberpunk-tab.active::after {
        border-right-color: #00f3ff;
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
    }

    .tab-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        z-index: 1;
    }

    .tab-label {
        font-size: 0.85rem;
        text-transform: uppercase;
        letter-spacing: 1px;
        text-shadow: 0 0 5px rgba(0, 243, 255, 0.7);
    }

    .mb-4 {
        margin-bottom: 0.75rem;
    }
</style>