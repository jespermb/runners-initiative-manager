<script lang="ts">
    import FormField from "../ui/FormField.svelte";

    interface Props {
        name: string;
        combatantType: string;
        physical: number;
        stun: number;
        errors: Record<string, string[]>;
        onNameChange: (value: string) => void;
        onPhysicalChange: (value: number) => void;
        onStunChange: (value: number) => void;
        onTypeChange: (type: string) => void;
    }

    const { 
        name, 
        combatantType, 
        physical, 
        stun, 
        errors,
        onNameChange,
        onPhysicalChange,
        onStunChange,
        onTypeChange
    }: Props = $props();
</script>

<div class="new-combattent-form">
    <div class="form-control">
        <fieldset>
            <div class="type-selection">
                <label class="type-option" for="combatant-type-pc">
                    <span class="label-text mr-2">PC</span> 
                    <input 
                        type="radio" 
                        id="combatant-type-pc"
                        name="combatant-type" 
                        class="radio radio-primary" 
                        checked={combatantType === 'pc'}
                        onchange={() => onTypeChange('pc')}
                        value="pc"
                    />
                </label>
                <label class="type-option" for="combatant-type-npc">
                    <span class="label-text mr-2">NPC</span> 
                    <input 
                        type="radio" 
                        id="combatant-type-npc"
                        name="combatant-type" 
                        class="radio radio-primary" 
                        checked={combatantType === 'npc'}
                        onchange={() => onTypeChange('npc')}
                        value="npc"
                    />
                </label>
            </div>
        </fieldset>
    </div>
    
    <FormField
        label="Name"
        name="name"
        id="combatten-name"
        placeholder="Enter a name"
        value={name}
        errors={errors.name || []}
        required={true}
        onInput={onNameChange}
    />
    
    <FormField
        label="Physical Health"
        name="physical"
        id="combatten-physical"
        type="number"
        placeholder="Enter physical health"
        value={physical}
        errors={errors.physical || []}
        required={true}
        onInput={onPhysicalChange}
    />
    
    <FormField
        label="Stun Health"
        name="stun"
        id="combatten-stun"
        type="number"
        placeholder="Enter stun health"
        value={stun}
        errors={errors.stun || []}
        required={true}
        onInput={onStunChange}
    />
</div>

<style>
    .new-combattent-form {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .form-control {
        margin-bottom: 0.25rem;
    }

    fieldset {
        border: none;
        padding: 0;
        margin: 0;
    }

    legend {
        padding: 0;
        margin-bottom: 0.5rem;
    }

    .label-text {
        font-size: 0.875rem;
        font-weight: 500;
        color: #c0c0c0;
    }

    .type-selection {
        display: flex;
        gap: 1rem;
    }

    .type-option {
        display: flex;
        align-items: center;
        cursor: pointer;
    }

    .radio {
        width: 1rem;
        height: 1rem;
        border: 2px solid #00f3ff;
        border-radius: 50%;
        background-color: transparent;
        cursor: pointer;
        margin-left: 0.5rem;
    }

    .radio:checked {
        background-color: #00f3ff;
        box-shadow: 0 0 5px rgba(0, 243, 255, 0.5);
    }

    .radio-primary {
        accent-color: #00f3ff;
    }

    .mb-2 {
        margin-bottom: 0.5rem;
    }

    .mr-2 {
        margin-right: 0.5rem;
    }
</style>