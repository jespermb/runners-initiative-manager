<script lang="ts">
    import type { Combatten } from "../../types/Combatten";
    import FormField from "../ui/FormField.svelte";

    interface Props {
        addType: string;
        selectedCombatten: Combatten | null;
        name: string;
        combatantType: string;
        physical: number;
        stun: number;
        initiative: number;
        errors: Record<string, string[]>;
        onInitiativeChange: (value: number) => void;
    }

    const { 
        addType,
        selectedCombatten,
        name,
        combatantType,
        physical,
        stun,
        initiative,
        errors,
        onInitiativeChange
    }: Props = $props();
</script>

<div class="initiative-step">
    <div class="selected-combatant-summary">
        <h3 class="summary-title">Selected Combatant</h3>
        <div class="summary-content">
            <p class="summary-item">
                <span class="summary-label">Name:</span> 
                {addType === "new" ? name : selectedCombatten?.name}
            </p>
            <p class="summary-item">
                <span class="summary-label">Type:</span> 
                {addType === "new" 
                    ? (combatantType === "pc" ? "PC" : "NPC") 
                    : (selectedCombatten?.combatten_type === "pc" ? "PC" : "NPC")
                }
            </p>
            {#if addType === "new"}
                <p class="summary-item">
                    <span class="summary-label">Physical Health:</span> {physical}
                </p>
                <p class="summary-item">
                    <span class="summary-label">Stun Health:</span> {stun}
                </p>
            {/if}
        </div>
    </div>
    
    <FormField
        label="Initiative"
        name="initiative"
        id="combatten-initiative"
        type="number"
        placeholder="Enter initiative value"
        value={initiative}
        errors={errors.initiative || []}
        required={true}
        onInput={onInitiativeChange}
    />
</div>

<style>
    .initiative-step {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .selected-combatant-summary {
        background-color: #1a1a1a;
        border: 1px solid #00f3ff;
        border-radius: 0.5rem;
        padding: 0.5rem;
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.2);
    }

    .summary-title {
        font-size: 0.9rem;
        font-weight: 600;
        margin: 0 0 0.25rem 0;
        color: #00f3ff;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    .summary-content {
        display: flex;
        flex-direction: column;
        gap: 0.125rem;
    }

    .summary-item {
        margin: 0;
        color: #c0c0c0;
        font-size: 0.875rem;
    }

    .summary-label {
        font-weight: 600;
        color: white;
        margin-right: 0.5rem;
    }
</style>