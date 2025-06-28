<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Combatten } from "../../types/Combatten";
    import { z } from "zod";
    
    interface Props {
        campaign_id?: number;
        encounter_id?: number;
        oncombattenAdded?: (combatten: Combatten) => void;
        onclose?: () => void;
    }
    
    const { campaign_id = 0, encounter_id = 0, oncombattenAdded, onclose }: Props = $props();
    
    // Form state
    let currentStep = $state(1); // Step 1: Select/Create combatant, Step 2: Set initiative
    let addType = $state("existing"); // "existing" or "new"
    let combatantType = $state("pc"); // "pc" or "npc"
    let selectedCombattenId = $state<number | null>(null);
    let selectedCombatten = $state<Combatten | null>(null);
    let newCombatten = $state<Combatten | null>(null);
    let name = $state("");
    let initiative = $state(0);
    let physical = $state(10);
    let stun = $state(10);
    let resultMsg = $state("");
    let combattens = $state([] as Combatten[]);
    let filteredCombattens = $state([] as Combatten[]);
    let searchQuery = $state("");

    // Validation schemas
    const newCombatantSchema = z.object({
        name: z.string().min(2, { message: "Name must be at least 2 characters long" }),
        physical: z.number().int().min(1, { message: "Physical health must be a positive number" }),
        stun: z.number().int().min(1, { message: "Stun health must be a positive number" }),
    });

    const existingCombatantSchema = z.object({
        selectedCombattenId: z.number({ 
            required_error: "Please select a combatant",
            invalid_type_error: "Please select a combatant" 
        }),
    });

    const initiativeSchema = z.object({
        initiative: z.number().int().min(0, { message: "Initiative must be a non-negative number" }),
    });

    type ValidationErrors = {
        name?: string[] | undefined;
        physical?: string[] | undefined;
        stun?: string[] | undefined;
        initiative?: string[] | undefined;
        selectedCombattenId?: string[] | undefined;
    };
    
    let errors = $state<ValidationErrors>({});

    // Filter combattens based on search query
    $effect(() => {
        if (searchQuery.trim() === "") {
            filteredCombattens = combattens;
        } else {
            const query = searchQuery.toLowerCase();
            filteredCombattens = combattens.filter(c => 
                c.name.toLowerCase().includes(query)
            );
        }
    });

    // Check if form is valid for step 1
    const isStep1Valid = $derived(() => {
        if (addType === "new") {
            return name.trim().length >= 2 && physical > 0 && stun > 0;
        } else {
            return selectedCombattenId !== null;
        }
    });

    // Update selectedCombatten when selectedCombattenId changes
    $effect(() => {
        if (selectedCombattenId) {
            selectedCombatten = combattens.find(c => c.id === selectedCombattenId) || null;
        } else {
            selectedCombatten = null;
        }
    });

    async function createNewCombatant() {
        // Validate form for step 1
        const validationResult = newCombatantSchema.safeParse({ 
            name, 
            physical: parseInt(`${physical}`), 
            stun: parseInt(`${stun}`) 
        });

        if (!validationResult.success) {
            errors = validationResult.error.flatten().fieldErrors;
            return;
        }

        try {
            // Add new combatant
            const combatten: Combatten = await invoke("add_combatten", { 
                name, 
                combattenType: combatantType,
                physical: parseInt(`${physical}`), 
                stun: parseInt(`${stun}`), 
                campaignId: campaign_id 
            });
            
            // Store the new combatant for step 2
            newCombatten = combatten;
            
            // Move to step 2
            currentStep = 2;
            errors = {};
        } catch (error) {
            resultMsg = `Error: ${error}`;
        }
    }

    function validateExistingCombatant() {
        // Validate form for step 1
        const validationResult = existingCombatantSchema.safeParse({ 
            selectedCombattenId
        });

        if (!validationResult.success) {
            errors = validationResult.error.flatten().fieldErrors;
            return false;
        }

        // Move to step 2
        currentStep = 2;
        errors = {};
        return true;
    }

    async function finalizeAddCombatant() {
        // Validate initiative
        const validationResult = initiativeSchema.safeParse({ 
            initiative: parseInt(`${initiative}`)
        });

        if (!validationResult.success) {
            errors = validationResult.error.flatten().fieldErrors;
            return;
        }

        try {
            // Determine which combatant to add
            const combattenId = addType === "new" 
                ? newCombatten?.id 
                : selectedCombattenId;
            
            if (!combattenId) {
                resultMsg = "Error: No combatant selected";
                return;
            }
            
            console.log("Adding combatten to encounter:", { 
                encounterId: encounter_id, 
                initiative: parseInt(`${initiative}`), 
                combattenId: combattenId 
            });
            
            // Add to encounter
            resultMsg = await invoke("add_combatten_to_encounter", { 
                encounterId: encounter_id, 
                initiative: parseInt(`${initiative}`), 
                combattenId: combattenId 
            });
            
            console.log("Add combatten result:", resultMsg);
            
            // Notify parent component first (before resetting form)
            const addedCombatten = addType === "new" 
                ? newCombatten 
                : selectedCombatten;
            if (addedCombatten) {
                oncombattenAdded?.(addedCombatten);
            }
            
            // Reset form
            currentStep = 1;
            selectedCombattenId = null;
            selectedCombatten = null;
            newCombatten = null;
            name = "";
            initiative = 0;
            physical = 10;
            stun = 10;
            errors = {};
            
        } catch (error) {
            console.error("Error adding combatten to encounter:", error);
            resultMsg = `Error: ${error}`;
        }
    }

    function handleNextStep(event: Event) {
        event.preventDefault();
        errors = {};
        
        if (currentStep === 1) {
            if (addType === "new") {
                createNewCombatant();
            } else {
                validateExistingCombatant();
            }
        } else if (currentStep === 2) {
            finalizeAddCombatant();
        }
    }

    function goBack() {
        currentStep = 1;
        errors = {};
    }

    // Initialize combattens on component mount
    $effect(() => {
        async function loadCombattens() {
            try {
                combattens = await invoke("get_all_campaign_combattens", { campaignId: campaign_id });
                filteredCombattens = combattens;
            } catch (error) {
                resultMsg = `Error loading combattens: ${error}`;
            }
        }
        loadCombattens();
    });
</script>

<style>
    /* Cyberpunk Steps Styling */
    .cyberpunk-steps {
        display: flex;
        background-color: #0a0a0a;
        border: 1px solid #00f3ff;
        overflow: hidden;
        position: relative;
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
    }

    .cyberpunk-steps::before {
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

    .cyberpunk-step {
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
        padding: 0.75rem 0;
        cursor: pointer;
        clip-path: polygon(
            0% 0%, 
            calc(100% - 10px) 0%, 
            100% 50%, 
            calc(100% - 10px) 100%, 
            0% 100%
        );
    }

    .cyberpunk-step:last-child {
        clip-path: polygon(
            10px 0%, 
            100% 0%, 
            100% 100%, 
            10px 100%, 
            0% 50%
        );
    }

    .cyberpunk-step::after {
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

    .cyberpunk-step:last-child::after {
        display: none;
    }

    .cyberpunk-step::before {
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

    .cyberpunk-step.active {
        color: #ffffff;
    }

    .cyberpunk-step.active::before {
        transform: translateY(0);
    }

    .cyberpunk-step.active {
        background-color: rgba(0, 243, 255, 0.1);
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5) inset, 
                    0 0 20px rgba(0, 243, 255, 0.3) inset, 
                    0 0 30px rgba(0, 243, 255, 0.1) inset;
    }

    .cyberpunk-step.active::after {
        border-right-color: #00f3ff;
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
    }

    .cyberpunk-step:disabled {
        cursor: default;
    }

    .step-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        z-index: 1;
    }

    .step-label {
        font-size: 0.85rem;
        text-transform: uppercase;
        letter-spacing: 1px;
        text-shadow: 0 0 5px rgba(0, 243, 255, 0.7);
    }

    /* Fixed Action Buttons Styling */
    .fixed-action-buttons {
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        padding: 1rem;
        background-color: #0a0a0a;
        border-top: 1px solid #00f3ff;
        box-shadow: 0 -5px 15px rgba(0, 243, 255, 0.2);
        z-index: 1000;
        margin-top: auto;
    }

    .fixed-action-buttons::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 1px;
        background: linear-gradient(90deg, #ff00ff, #00f3ff);
        animation: cyberpunk-scan-horizontal 2s linear infinite;
    }

    @keyframes cyberpunk-scan-horizontal {
        0% {
            transform: translateX(-100%);
        }
        100% {
            transform: translateX(100%);
        }
    }

    .fixed-action-buttons .btn {
        position: relative;
        overflow: hidden;
    }

    .fixed-action-buttons .btn::after {
        content: '';
        position: absolute;
        top: -50%;
        left: -50%;
        width: 200%;
        height: 200%;
        background: linear-gradient(rgba(0, 243, 255, 0.1), transparent);
        transform: rotate(30deg);
        animation: cyberpunk-btn-shine 3s linear infinite;
    }

    @keyframes cyberpunk-btn-shine {
        0% {
            transform: rotate(30deg) translateY(-100%);
        }
        100% {
            transform: rotate(30deg) translateY(100%);
        }
    }
</style>

<div class="p-4 flex flex-col h-full relative">
    <h2 class="text-xl font-bold mb-4">Add Combatant to Encounter</h2>
    
    <!-- Cyberpunk Step Tabs -->
    <div class="cyberpunk-steps mb-6">
        <button 
            type="button"
            class={`cyberpunk-step ${currentStep === 1 ? 'active' : ''}`}
            onclick={() => currentStep === 2 ? goBack() : null}
            disabled={currentStep === 1}
        >
            <div class="step-content">
                <span class="step-label">Select Combatant</span>
            </div>
        </button>
        <button 
            type="button"
            class={`cyberpunk-step ${currentStep === 2 ? 'active' : ''}`}
            disabled={currentStep === 2}
        >
            <div class="step-content">
                <span class="step-label">Set Initiative</span>
            </div>
        </button>
    </div>
    <form class="flex flex-col gap-3 flex-grow overflow-y-auto pb-20" onsubmit={handleNextStep}>
        {#if currentStep === 1}
            <!-- Step 1: Select or Create Combatant -->
            <!-- Toggle between new and existing -->
            <div class="tabs tabs-boxed mb-4">
                <button 
                    type="button"
                    class={`tab ${addType === 'existing' ? 'tab-active' : ''}`} 
                    onclick={() => { addType = 'existing'; errors = {}; }}
                >
                    Use Existing Combatant
                </button>
                <button 
                    type="button"
                    class={`tab ${addType === 'new' ? 'tab-active' : ''}`} 
                    onclick={() => { addType = 'new'; errors = {}; }}
                >
                    Create New Combatant
                </button>
            </div>
            
            {#if addType === "new"}
                <!-- New Combatant Form -->
                <div class="form-control">
                    <fieldset>
                        <legend class="label-text mb-2">Combatant Type</legend>
                        <div class="flex gap-4">
                        <label class="label cursor-pointer" for="combatant-type-pc">
                            <span class="label-text mr-2">PC</span> 
                            <input 
                                type="radio" 
                                id="combatant-type-pc"
                                name="combatant-type" 
                                class="radio radio-primary" 
                                bind:group={combatantType} 
                                value="pc"
                            />
                        </label>
                        <label class="label cursor-pointer" for="combatant-type-npc">
                            <span class="label-text mr-2">NPC</span> 
                            <input 
                                type="radio" 
                                id="combatant-type-npc"
                                name="combatant-type" 
                                class="radio radio-primary" 
                                bind:group={combatantType} 
                                value="npc"
                            />
                        </label>
                        </div>
                    </fieldset>
                </div>
                
                <div class="form-control">
                    <label class="label" for="name">
                        <span class="label-text">Name</span>
                    </label>
                    <input
                        name="name"
                        id="combatten-name"
                        placeholder="Enter a name"
                        bind:value={name}
                        class={`input input-bordered w-full ${errors.name && errors.name?.length > 0 ? "input-error" : ""}`}
                    />
                    {#if errors.name && errors.name?.length > 0}
                        <p class="text-error text-sm mt-1">{errors.name?.[0]}</p>
                    {/if}
                </div>
                
                <div class="form-control">
                    <label class="label" for="physical">
                        <span class="label-text">Physical Health</span>
                    </label>
                    <input
                        name="physical"
                        id="combatten-physical"
                        placeholder="Enter physical health"
                        bind:value={physical}
                        type="number"
                        class={`input input-bordered w-full ${errors.physical && errors.physical?.length > 0 ? "input-error" : ""}`}
                    />
                    {#if errors.physical && errors.physical?.length > 0}
                        <p class="text-error text-sm mt-1">{errors.physical?.[0]}</p>
                    {/if}
                </div>
                
                <div class="form-control">
                    <label class="label" for="stun">
                        <span class="label-text">Stun Health</span>
                    </label>
                    <input
                        name="stun"
                        id="combatten-stun"
                        placeholder="Enter stun health"
                        bind:value={stun}
                        type="number"
                        class={`input input-bordered w-full ${errors.stun && errors.stun?.length > 0 ? "input-error" : ""}`}
                    />
                    {#if errors.stun && errors.stun?.length > 0}
                        <p class="text-error text-sm mt-1">{errors.stun?.[0]}</p>
                    {/if}
                </div>
            {:else}
                <!-- Existing Combatant Form -->
                <div class="form-control">
                    <label class="label" for="search">
                        <span class="label-text">Search Combatants</span>
                    </label>
                    <input
                        name="search"
                        id="search-combattens"
                        placeholder="Search by name..."
                        bind:value={searchQuery}
                        class="input input-bordered w-full mb-2"
                    />
                </div>
                
                <div class="form-control">
                    <fieldset>
                        <legend class="label-text mb-2">Select Combatant</legend>
                        <div class="bg-base-200 rounded-lg max-h-48 overflow-y-auto">
                        {#if filteredCombattens.length === 0}
                            <div class="p-4 text-center text-base-content/70">
                                No combatants found
                            </div>
                        {:else}
                            {#each filteredCombattens as combatten}
                                <label 
                                    class={`p-3 border-b border-base-300 cursor-pointer hover:bg-base-300 ${selectedCombattenId === combatten.id ? 'bg-primary/20' : ''} flex items-center`}
                                >
                                    <input 
                                        type="radio" 
                                        name="selected-combatten" 
                                        class="radio radio-primary mr-2" 
                                        value={combatten.id}
                                        bind:group={selectedCombattenId}
                                    />
                                    <span>{combatten.name}</span>
                                </label>
                            {/each}
                        {/if}
                        </div>
                        {#if errors.selectedCombattenId && errors.selectedCombattenId?.length > 0}
                            <p class="text-error text-sm mt-1">{errors.selectedCombattenId?.[0]}</p>
                        {/if}
                    </fieldset>
                </div>
            {/if}
            
        {:else}
            <!-- Step 2: Set Initiative -->
            <div class="bg-base-200 p-4 rounded-lg mb-4">
                <h3 class="font-bold mb-2">Selected Combatant</h3>
                <p class="mb-1">
                    <span class="font-semibold">Name:</span> 
                    {addType === "new" ? name : selectedCombatten?.name}
                </p>
                <p class="mb-1">
                    <span class="font-semibold">Type:</span> 
                    {addType === "new" 
                        ? (combatantType === "pc" ? "PC" : "NPC") 
                        : (selectedCombatten?.combattenType === "pc" ? "PC" : "NPC")
                    }
                </p>
                {#if addType === "new"}
                    <p class="mb-1">
                        <span class="font-semibold">Physical Health:</span> {physical}
                    </p>
                    <p>
                        <span class="font-semibold">Stun Health:</span> {stun}
                    </p>
                {/if}
            </div>
            
            <div class="form-control">
                <label class="label" for="initiative">
                    <span class="label-text">Initiative</span>
                </label>
                <input
                    name="initiative"
                    id="combatten-initiative"
                    placeholder="Enter initiative value"
                    bind:value={initiative}
                    type="number"
                    class={`input input-bordered w-full ${errors.initiative && errors.initiative?.length > 0 ? "input-error" : ""}`}
                />
                {#if errors.initiative && errors.initiative?.length > 0}
                    <p class="text-error text-sm mt-1">{errors.initiative?.[0]}</p>
                {/if}
            </div>
        {/if}
    </form>
    
    <!-- Fixed Action Buttons -->
    <div class="fixed-action-buttons">
        {#if currentStep === 1}
            <div class="flex gap-2">
                <button type="button" class="btn btn-outline flex-1" onclick={() => onclose?.()}>
                    Cancel
                </button>
                <button 
                    type="button" 
                    class="btn btn-primary flex-1" 
                    onclick={handleNextStep}
                    disabled={!isStep1Valid()}
                >
                    {addType === "new" ? "Save & Continue" : "Next"}
                </button>
            </div>
        {:else}
            <div class="flex gap-2">
                <button type="button" class="btn btn-outline" onclick={goBack}>
                    Back
                </button>
                <button type="button" class="btn btn-outline" onclick={() => onclose?.()}>
                    Cancel
                </button>
                <button type="button" class="btn btn-primary flex-1" onclick={handleNextStep}>
                    Add
                </button>
            </div>
        {/if}
    </div>
    
    {#if resultMsg}
        <div class="mt-4 p-2 bg-success/20 text-success-content rounded">
            {resultMsg}
        </div>
    {/if}
</div>
