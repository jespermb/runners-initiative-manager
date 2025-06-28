<script lang="ts">
    import type { Combatten } from "../../types/Combatten";
    import CyberpunkSteps, { type Step } from "../ui/CyberpunkSteps.svelte";
    import CyberpunkActionButtons from "../ui/CyberpunkActionButtons.svelte";
    import SelectCombattentStep from "./SelectCombattentStep.svelte";
    import InitiativeStep from "./InitiativeStep.svelte";
    import { useCombattentForm } from "../composables/useCombattentForm";
    
    interface Props {
        campaign_id?: number;
        encounter_id?: number;
        oncombattenAdded?: (combatten: Combatten) => void;
        onclose?: () => void;
        existingCombattens?: number[];
    }
    
    const { campaign_id = 0, encounter_id = 0, oncombattenAdded, onclose, existingCombattens = [] }: Props = $props();
    
    // Form state
    let currentStep = $state(1);
    let addType = $state("existing");
    let combatantType = $state("pc");
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
    let errors = $state<Record<string, string[]>>({});

    // Use business logic composable
    const {
        validateNewCombatant,
        validateExistingCombatant,
        validateInitiative,
        createNewCombatant,
        addCombattentToEncounter,
        loadCombattens,
        filterCombattens,
        isStep1Valid
    } = useCombattentForm();

    // Steps configuration
    const steps: Step[] = $derived([
        {
            id: 1,
            label: "Select Combatant",
            active: currentStep === 1,
            disabled: currentStep === 1
        },
        {
            id: 2,
            label: "Set Initiative",
            active: currentStep === 2,
            disabled: currentStep === 2
        }
    ]);

    // Filter combattens based on search query and existing combattens
    $effect(() => {
        filteredCombattens = filterCombattens(combattens, searchQuery, existingCombattens);
    });

    // Update selectedCombatten when selectedCombattenId changes
    $effect(() => {
        if (selectedCombattenId) {
            selectedCombatten = combattens.find(c => c.id === selectedCombattenId) || null;
        } else {
            selectedCombatten = null;
        }
    });

    // Event handlers
    function handleStepClick(stepId: number) {
        if (stepId === 1 && currentStep === 2) {
            goBack();
        }
    }

    function handleAddTypeChange(type: string) {
        addType = type;
        errors = {};
    }

    function handleSearchChange(query: string) {
        searchQuery = query;
    }

    function handleSelectionChange(id: number) {
        selectedCombattenId = id;
    }

    function handleNameChange(value: string | number) {
        name = String(value);
    }

    function handlePhysicalChange(value: string | number) {
        physical = Number(value);
    }

    function handleStunChange(value: string | number) {
        stun = Number(value);
    }

    function handleTypeChange(type: string) {
        combatantType = type;
    }

    function handleInitiativeChange(value: string | number) {
        initiative = Number(value);
    }

    async function handleCreateNewCombatant() {
        const validation = await validateNewCombatant(name, physical, stun);
        
        if (!validation.valid) {
            errors = validation.errors;
            return;
        }

        const result = await createNewCombatant(name, combatantType, physical, stun, campaign_id);
        
        if (result.success && result.combatten) {
            newCombatten = result.combatten;
            currentStep = 2;
            errors = {};
        } else {
            resultMsg = result.error || "Unknown error occurred";
        }
    }

    async function handleValidateExisting() {
        const validation = await validateExistingCombatant(selectedCombattenId);
        
        if (!validation.valid) {
            errors = validation.errors;
            return false;
        }

        currentStep = 2;
        errors = {};
        return true;
    }

    async function handleFinalizeAdd() {
        const validation = await validateInitiative(initiative);
        
        if (!validation.valid) {
            errors = validation.errors;
            return;
        }

        const combattenId = addType === "new" ? newCombatten?.id : selectedCombattenId;
        
        if (!combattenId) {
            resultMsg = "Error: No combatant selected";
            return;
        }

        const result = await addCombattentToEncounter(encounter_id, combattenId, initiative);
        
        if (result.success) {
            resultMsg = result.message || "Success";
            
            // Notify parent component
            const addedCombatten = addType === "new" ? newCombatten : selectedCombatten;
            if (addedCombatten) {
                oncombattenAdded?.(addedCombatten);
            }
            
            // Reset form
            resetForm();
        } else {
            resultMsg = result.error || "Unknown error occurred";
        }
    }

    function resetForm() {
        currentStep = 1;
        selectedCombattenId = null;
        selectedCombatten = null;
        newCombatten = null;
        name = "";
        initiative = 0;
        physical = 10;
        stun = 10;
        errors = {};
    }

    async function handleNextStep() {
        errors = {};
        
        if (currentStep === 1) {
            if (addType === "new") {
                await handleCreateNewCombatant();
            } else {
                await handleValidateExisting();
            }
        } else if (currentStep === 2) {
            await handleFinalizeAdd();
        }
    }

    function goBack() {
        currentStep = 1;
        errors = {};
    }

    // Initialize combattens on component mount
    $effect(() => {
        async function initializeCombattens() {
            const result = await loadCombattens(campaign_id);
            
            if (result.success && result.combattens) {
                combattens = result.combattens;
            } else {
                resultMsg = result.error || "Failed to load combattens";
            }
        }
        
        initializeCombattens();
    });
</script>

<style>
    .container {
        display: flex;
        flex-direction: column;
        height: 100%;
        position: relative;
        padding: 0.5rem;
        gap: 0.5rem;
    }

    .header {
        margin-bottom: 0.25rem;
    }

    .title {
        font-size: 1.25rem;
        font-weight: 700;
        color: white;
        margin: 0;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    .form-container {
        flex: 1;
        overflow-y: auto;
        padding-bottom: 3.5rem;
    }

    .result-message {
        margin-top: 1rem;
        padding: 0.75rem;
        background-color: rgba(0, 243, 255, 0.1);
        border: 1px solid #00f3ff;
        border-radius: 0.25rem;
        color: #00f3ff;
        font-size: 0.875rem;
    }
</style>

<div class="container">
    <div class="header">
        <h2 class="title">Add Combatten</h2>
    </div>
    
    <CyberpunkSteps {steps} onStepClick={handleStepClick} />
    
    <div class="form-container">
        {#if currentStep === 1}
            <SelectCombattentStep
                {addType}
                {combattens}
                {filteredCombattens}
                {searchQuery}
                {selectedCombattenId}
                {name}
                {combatantType}
                {physical}
                {stun}
                {errors}
                onAddTypeChange={handleAddTypeChange}
                onSearchChange={handleSearchChange}
                onSelectionChange={handleSelectionChange}
                onNameChange={handleNameChange}
                onPhysicalChange={handlePhysicalChange}
                onStunChange={handleStunChange}
                onTypeChange={handleTypeChange}
            />
        {:else}
            <InitiativeStep
                {addType}
                {selectedCombatten}
                {name}
                {combatantType}
                {physical}
                {stun}
                {initiative}
                {errors}
                onInitiativeChange={handleInitiativeChange}
            />
        {/if}
    </div>
    
    <CyberpunkActionButtons
        position="fixed"
        buttons={currentStep === 1 ? [
            {
                label: "Cancel",
                type: "outline",
                flex: true,
                onclick: () => onclose?.()
            },
            {
                label: addType === "new" ? "Save" : "Next",
                type: "primary",
                flex: true,
                disabled: !isStep1Valid(addType, name, physical, stun, selectedCombattenId),
                onclick: handleNextStep
            }
        ] : [
            {
                label: "Back",
                type: "outline",
                onclick: goBack
            },
            {
                label: "Cancel",
                type: "outline",
                onclick: () => onclose?.()
            },
            {
                label: "Add",
                type: "primary",
                flex: true,
                onclick: handleNextStep
            }
        ]}
    />
    
    {#if resultMsg}
        <div class="result-message">
            {resultMsg}
        </div>
    {/if}
</div>
