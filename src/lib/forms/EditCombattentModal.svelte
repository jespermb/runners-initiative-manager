<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Combatten } from "../../types/Combatten";
    import FormField from "../ui/FormField.svelte";
    import CyberpunkActionButtons from "../ui/CyberpunkActionButtons.svelte";
    import { z } from "zod";

    interface Props {
        combatten: Combatten;
        isOpen: boolean;
        onClose: () => void;
        onSaved: () => void;
    }

    const { combatten, isOpen, onClose, onSaved }: Props = $props();

    // Form state
    let name = $state(combatten.name);
    let physical = $state(combatten.physical);
    let stun = $state(combatten.stun);
    let resultMsg = $state("");
    let errors = $state<Record<string, string[]>>({});
    let isSubmitting = $state(false);

    // Validation schema
    const editSchema = z.object({
        name: z.string().min(2, { message: "Name must be at least 2 characters long" }),
        physical: z.number().int().min(1, { message: "Physical health must be a positive number" }),
        stun: z.number().int().min(1, { message: "Stun health must be a positive number" }),
    });

    // Reset form when combatten changes
    $effect(() => {
        name = combatten.name;
        physical = combatten.physical;
        stun = combatten.stun;
        errors = {};
        resultMsg = "";
    });

    function handleNameChange(value: string | number) {
        name = String(value);
    }

    function handlePhysicalChange(value: string | number) {
        physical = Number(value);
    }

    function handleStunChange(value: string | number) {
        stun = Number(value);
    }

    async function handleSave() {
        errors = {};
        resultMsg = "";
        
        // Validate
        const validation = editSchema.safeParse({ 
            name, 
            physical: parseInt(`${physical}`), 
            stun: parseInt(`${stun}`) 
        });
        if (!validation.success) {
            errors = validation.error.flatten().fieldErrors;
            return;
        }

        isSubmitting = true;
        
        try {
            const result = await invoke("edit_combatten", {
                id: combatten.id,
                name: name.trim(),
                physical: parseInt(`${physical}`),
                stun: parseInt(`${stun}`)
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
        name = combatten.name;
        physical = combatten.physical;
        stun = combatten.stun;
        errors = {};
        resultMsg = "";
        onClose();
    }
</script>

{#if isOpen}
    <div class="modal-overlay" onclick={handleCancel}>
        <div class="modal-content" onclick={(e) => e.stopPropagation()}>
            <div class="modal-header">
                <h2 class="modal-title">Edit Combatten</h2>
            </div>
            
            <div class="modal-body">
                <div class="combatten-info">
                    <p class="info-item">
                        <span class="info-label">Type:</span>
                        {combatten.combatten_type.toUpperCase()}
                    </p>
                    <p class="info-item">
                        <span class="info-label">ID:</span>
                        {combatten.id}
                    </p>
                </div>
                
                <FormField
                    label="Name"
                    name="name"
                    id="edit-combatten-name"
                    placeholder="Enter combatten name"
                    value={name}
                    errors={errors.name || []}
                    required={true}
                    onInput={handleNameChange}
                />
                
                <FormField
                    label="Physical Health"
                    name="physical"
                    id="edit-combatten-physical"
                    type="number"
                    placeholder="Enter physical health"
                    value={physical}
                    errors={errors.physical || []}
                    required={true}
                    onInput={handlePhysicalChange}
                />
                
                <FormField
                    label="Stun Health"
                    name="stun"
                    id="edit-combatten-stun"
                    type="number"
                    placeholder="Enter stun health"
                    value={stun}
                    errors={errors.stun || []}
                    required={true}
                    onInput={handleStunChange}
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
                        disabled: isSubmitting || name.trim().length < 2 || physical < 1 || stun < 1,
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