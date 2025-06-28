<script lang="ts">
    import type { Combatten } from "../../types/Combatten";

    interface Props {
        items: Combatten[];
        searchQuery: string;
        selectedId: number | null;
        emptyMessage?: string;
        onSearchChange: (query: string) => void;
        onSelectionChange: (id: number) => void;
        errors?: string[];
    }

    const { 
        items, 
        searchQuery, 
        selectedId, 
        emptyMessage = "No items found",
        onSearchChange,
        onSelectionChange,
        errors = []
    }: Props = $props();

    function handleSearchInput(event: Event) {
        const target = event.target as HTMLInputElement;
        onSearchChange(target.value);
    }

    function handleSelectionChange(event: Event) {
        const target = event.target as HTMLInputElement;
        onSelectionChange(parseInt(target.value));
    }
</script>

<div class="searchable-list">
    <div class="form-control">
        <label class="label" for="search">
            <span class="label-text">Search Items</span>
        </label>
        <input
            name="search"
            id="search"
            type="text"
            placeholder="Search by name..."
            value={searchQuery}
            class="input input-bordered w-full mb-2"
            oninput={handleSearchInput}
        />
    </div>
    
    <div class="form-control">
        <fieldset>
            <legend class="label-text mb-2">Select Item</legend>
            <div class="selection-container">
                {#if items.length === 0}
                    <div class="empty-message">
                        {emptyMessage}
                    </div>
                {:else}
                    {#each items as item}
                        <label 
                            class={`selection-item ${selectedId === item.id ? 'selected' : ''}`}
                        >
                            <input 
                                type="radio" 
                                name="selected-item" 
                                class="radio radio-primary mr-2" 
                                value={item.id}
                                checked={selectedId === item.id}
                                onchange={handleSelectionChange}
                            />
                            <span class="item-name">{item.name}</span>
                            <span class="item-type">{item.combatten_type.toUpperCase()}</span>
                        </label>
                    {/each}
                {/if}
            </div>
            {#if errors.length > 0}
                <p class="text-error text-sm mt-1">{errors[0]}</p>
            {/if}
        </fieldset>
    </div>
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

    .input-bordered {
        border-width: 1px;
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

    .selection-container {
        background-color: #1a1a1a;
        border: 1px solid #00f3ff;
        border-radius: 0.5rem;
        max-height: 12rem;
        overflow-y: auto;
    }

    .selection-item {
        display: flex;
        align-items: center;
        padding: 0.5rem;
        border-bottom: 1px solid #333;
        cursor: pointer;
        transition: background-color 0.2s ease;
    }

    .selection-item:last-child {
        border-bottom: none;
    }

    .selection-item:hover {
        background-color: #2a2a2a;
    }

    .selection-item.selected {
        background-color: rgba(0, 243, 255, 0.1);
        border-color: #00f3ff;
    }

    .item-name {
        flex: 1;
        margin-left: 0.5rem;
        color: white;
    }

    .item-type {
        font-size: 0.75rem;
        padding: 0.25rem 0.5rem;
        border-radius: 0.25rem;
        text-transform: uppercase;
        font-weight: 500;
        letter-spacing: 0.5px;
    }

    .empty-message {
        padding: 1rem;
        text-align: center;
        color: #666;
    }

    .radio {
        width: 1rem;
        height: 1rem;
        border: 2px solid #00f3ff;
        border-radius: 50%;
        background-color: transparent;
        cursor: pointer;
    }

    .radio:checked {
        background-color: #00f3ff;
        box-shadow: 0 0 5px rgba(0, 243, 255, 0.5);
    }

    .radio-primary {
        accent-color: #00f3ff;
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

    .mb-2 {
        margin-bottom: 0.5rem;
    }
</style>