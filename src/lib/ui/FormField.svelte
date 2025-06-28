<script lang="ts">
    interface Props {
        label: string;
        name: string;
        id?: string;
        type?: 'text' | 'number' | 'search';
        placeholder?: string;
        value: string | number;
        errors?: string[];
        required?: boolean;
        onInput?: (value: string | number) => void;
    }

    const { 
        label, 
        name, 
        id = name, 
        type = 'text', 
        placeholder = '', 
        value = '', 
        errors = [], 
        required = false,
        onInput
    }: Props = $props();

    function handleInput(event: Event) {
        const target = event.target as HTMLInputElement;
        const newValue = type === 'number' ? parseInt(target.value) || 0 : target.value;
        onInput?.(newValue);
    }
</script>

<div class="form-control">
    <label class="label" for={id}>
        <span class="label-text">{label}{required ? ' *' : ''}</span>
    </label>
    <input
        {name}
        {id}
        {type}
        {placeholder}
        {value}
        class={`input input-bordered w-full ${errors.length > 0 ? "input-error" : ""}`}
        oninput={handleInput}
    />
    {#if errors.length > 0}
        <p class="text-error text-sm mt-1">{errors[0]}</p>
    {/if}
</div>

<style>
    .form-control {
        margin-bottom: 0.25rem;
    }

    .label {
        display: block;
        margin-bottom: 0.125rem;
    }

    .label-text {
        font-size: 0.875rem;
        font-weight: 500;
        color: #c0c0c0;
    }

    .input {
        width: 100%;
        padding: 0.375rem;
        background-color: #1a1a1a;
        border: 1px solid #00f3ff;
        border-radius: 0.25rem;
        color: white;
        transition: all 0.3s ease;
    }

    .input:focus {
        outline: none;
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
        border-color: #ff00ff;
    }

    .input-error {
        border-color: #ef4444;
    }

    .input-bordered {
        border-width: 1px;
    }

    .text-error {
        color: #ef4444;
    }

    .text-sm {
        font-size: 0.875rem;
    }

    .mt-1 {
        margin-top: 0.25rem;
    }

    .w-full {
        width: 100%;
    }
</style>