<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Encounter } from "../../types/Encounter";
    import { z } from "zod";

    let name = $state("");

    const { campaignId = 0, onSave } = $props();

    const schema = z.object({
        name: z.string().min(2, { message: "Name must be at least 2 characters long" }),
    });
    
    type ValidationErrors = {
        name?: string[] | undefined;
    };
    
    let errors = $state<ValidationErrors>({
        name: [],
    });

    async function save(evt: Event) {
        evt.preventDefault();
        
        const validationResult = schema.safeParse({ name });
        
        if (!validationResult.success) {
            errors = validationResult.error.flatten().fieldErrors;
            return;
        }
        
        await invoke("add_encounter", {
            name,
            campaignId,
        });
        
        name = "";
        onSave();
    }
</script>

<div class="">
    <form class="flex gap-3 flex-col" onsubmit={save}>
        <label for="name">Name</label>
        <input
            name="name"
            id="encounter-name"
            placeholder="Encounter name"
            bind:value={name}
            class={`input input-bordered w-full ${errors.name && errors.name?.length > 0 ? " input-error" : ""}`}
        />
        {#if errors.name && errors.name?.length > 0}
            <p class="text-error">{errors.name?.[0]}</p>
        {/if}
        <button type="submit" class="btn btn-primary justify-self-end">Save</button>
    </form>
</div>
