<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import FormField from "../ui/FormField.svelte";
    import CyberpunkActionButtons from "../ui/CyberpunkActionButtons.svelte";
    import { z } from "zod";

    interface Props {
        combattenId: number;
        combattenName: string;
        currentInitiative: number;
        encounterId: number;
        isOpen: boolean;
        onClose: () => void;
        onSaved: () => void;
    }

    const { combattenId, combattenName, currentInitiative, encounterId, isOpen, onClose, onSaved }: Props = $props();

    // Form state
    let initiative = $state(currentInitiative);
    let resultMsg = $state("");
    let errors = $state<Record<string, string[]>>({});
    let isSubmitting = $state(false);

    // Validation schema
    const initiativeSchema = z.object({
        initiative: z.number().int().min(0, { message: "Initiative must be a non-negative number" }),
    });

    // Reset form when modal opens or current initiative changes
    $effect(() => {
        initiative = currentInitiative;
        errors = {};
        resultMsg = "";
    });

    function handleInitiativeChange(value: string | number) {
        initiative = Number(value);
    }

    async function handleSave() {
        errors = {};
        resultMsg = "";
        
        // Validate
        const validation = initiativeSchema.safeParse({ initiative });
        if (!validation.success) {
            errors = validation.error.flatten().fieldErrors;
            return;
        }

        isSubmitting = true;
        
        try {
            // Remove old entry and add new one with updated initiative
            await invoke("remove_combatten_from_encounter", {
                encounterId: encounterId,
                combattenId: combattenId
            });
            
            const result = await invoke("add_combatten_to_encounter", {
                encounterId: encounterId,
                combattenId: combattenId,
                initiative: initiative
            });
            
            resultMsg = String(result);
            onSaved();
            onClose();
        } catch (error) {
            resultMsg = `Error: ${error}`;
        } finally {
            isSubmitting = false;
        }
    }

    function handleCancel() {
        // Reset form
        initiative = currentInitiative;
        errors = {};
        resultMsg = "";
        onClose();
    }
</script>

{#if isOpen}
    <div class="modal-overlay" onclick={handleCancel}>
        <div class="modal-content" onclick={(e) => e.stopPropagation()}>
            <div class="modal-header">
                <h2 class="modal-title">Edit Initiative</h2>
            </div>
            
            <div class="modal-body">
                <div class="combatten-info">
                    <p class="info-item">
                        <span class="info-label">Combatten:</span>
                        {combattenName}
                    </p>
                    <p class="info-item">
                        <span class="info-label">Current:</span>
                        {currentInitiative}
                    </p>
                </div>
                
                <FormField
                    label="New Initiative"
                    name="initiative"
                    id="edit-initiative"
                    type="number"
                    placeholder="Enter initiative value"
                    value={initiative}
                    errors={errors.initiative || []}
                    required={true}
                    onInput={handleInitiativeChange}
                />
                
                {#if resultMsg}
                    <div class="result-message">
                        {resultMsg}
                    </div>
                {/if}
            </div>
            
            <CyberpunkActionButtons
                position="relative"
                buttons={[
                    {
                        label: "Cancel",
                        type: "outline",
                        onclick: handleCancel
                    },
                    {
                        label: "Save",
                        type: "primary",
                        flex: true,
                        disabled: isSubmitting || initiative < 0,
                        onclick: handleSave
                    }
                ]}
            />
        </div>
    </div>
{/if}

<style>
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(0, 0, 0, 0.7);
        backdrop-filter: blur(3px);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .modal-content {
        background-color: #0a0a0a;
        color: white;
        border: 1px solid #00f3ff;
        border-radius: 0.5rem;
        box-shadow: 0 0 15px rgba(0, 243, 255, 0.5);
        animation: cyberpunk-glow 1.5s ease-in-out infinite alternate;
        max-width: 400px;
        width: 90%;
        max-height: 90vh;
        overflow-y: auto;
        position: relative;
    }

    .modal-content::before {
        content: '';
        position: absolute;
        top: -2px;
        left: -2px;
        right: -2px;
        height: 2px;
        background: linear-gradient(90deg, #ff00ff, #00f3ff);
        animation: cyberpunk-scan 2s linear infinite;
    }

    @keyframes cyberpunk-glow {
        from {
            box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
        }
        to {
            box-shadow: 0 0 20px rgba(0, 243, 255, 0.8), 0 0 30px rgba(255, 0, 255, 0.3);
        }
    }

    @keyframes cyberpunk-scan {
        0% {
            transform: translateY(0);
        }
        100% {
            transform: translateY(calc(100% + 4px));
        }
    }

    .modal-header {
        padding: 1rem 1rem 0.5rem 1rem;
        border-bottom: 1px solid #333;
    }

    .modal-title {
        font-size: 1.125rem;
        font-weight: 700;
        margin: 0;
        color: #00f3ff;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    .modal-body {
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .combatten-info {
        background-color: #1a1a1a;
        border: 1px solid #333;
        border-radius: 0.25rem;
        padding: 0.5rem;
        display: flex;
        gap: 1rem;
    }

    .info-item {
        margin: 0;
        font-size: 0.875rem;
        color: #c0c0c0;
    }

    .info-label {
        font-weight: 600;
        color: white;
        margin-right: 0.25rem;
    }

    .result-message {
        padding: 0.5rem;
        background-color: rgba(0, 243, 255, 0.1);
        border: 1px solid #00f3ff;
        border-radius: 0.25rem;
        color: #00f3ff;
        font-size: 0.875rem;
    }
</style>