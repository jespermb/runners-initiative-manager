<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher } from "svelte";
    import type { Combatten } from "../../types/Combatten";
    import { z } from "zod";

    const dispatch = createEventDispatcher();

    let name = $state("");
    let initiative = $state(0);
    let physical = $state(0);
    let stun = $state(0);
    let greetMsg = $state("");

    const { campaignId = 0, encounterId = 0 } = $props();

    const schema = z.object({
        name: z.string().min(2, { message: "Name must be at least 2 characters long" }),
        initiative: z.number().int().min(1, { message: "Initiative must be a positive number" }),
        physical: z.number().int().min(1, { message: "Physical health must be a positive number" }),
        stun: z.number().int().min(1, { message: "Stun health must be a positive number" }),
    })
    type ValidationErrors = {
        name?: string[] | undefined;
        initiative?: string[] | undefined;
        physical?: string[] | undefined;
        stun?: string[] | undefined;
    };
    
    let errors = $state<ValidationErrors>({
        name: [],
        initiative: [],
        physical: [],
        stun: [],
    });

    async function save() {
        // Learn more about Tauri commands at https://tauri.app/v2/guide/command
        const phys = parseInt(`${physical}`);
        const stn = parseInt(`${stun}`);
        const validationResult = schema.safeParse({ 
            name, 
            initiative: parseInt(`${initiative}`), 
            physical: phys, 
            stun: stn 
        });
        console.log(validationResult)
        if (!validationResult.success) {
            errors = validationResult.error.flatten().fieldErrors;
            return;
        }
        const combatten: Combatten = await invoke("add_combatten", { name, physical: phys, stun: stn, campaignId });
        console.log(combatten);
        greetMsg = await invoke("add_combatten_to_encounter", { encounterId, initiative, combattenId: combatten.id });
        name = "";
        initiative = 0;
        physical = 0;
        stun = 0;
        dispatch("combattenAdded");
    }
</script>

<div class="">
    <form class="flex gap-3 flex-col" on:submit|preventDefault={save}>
        <label for="name">Name</label>
        <input
            name="name"
            id="combatten-name"
            placeholder="Enter a name"
            bind:value={name}
            class={`input input-bordered w-full ${errors.name && errors.name?.length > 0 ? " input-error" : ""}`}
        />
        {#if errors.name && errors.name?.length > 0}
            <p class="text-error">{errors.name?.[0]}</p>
        {/if}
        <label for="initiative">Initiative</label>
        <input
            name="initiative"
            id="combatten-initiative"
            placeholder="Enter initiative"
            bind:value={initiative}
            type="number"
            class={`input input-bordered w-full ${errors.initiative && errors.initiative?.length > 0 ? " input-error" : ""}`}
        />
        {#if errors.initiative && errors.initiative?.length > 0}
            <p class="text-error">{errors.initiative?.[0]}</p>
        {/if}
        <label for="physical">Physical health</label>
        <input
            name="physical"
            id="combatten-physical"
            placeholder="Enter physical health"
            bind:value={physical}
            type="number"
            class={`input input-bordered w-full ${errors.physical && errors.physical?.length > 0 ? " input-error" : ""}`}
        />
        {#if errors.physical && errors.physical?.length > 0}
            <p class="text-error">{errors.physical?.[0]}</p>
        {/if}
        <label for="stun">Stun health</label>
        <input
            name="stun"
            id="combatten-stun"
            placeholder="Enter stun health"
            bind:value={stun}
            type="number"
            class={`input input-bordered w-full ${errors.stun && errors.stun?.length > 0 ? " input-error" : ""}`}
        />
        {#if errors.stun && errors.stun?.length > 0}
            <p class="text-error">{errors.stun?.[0]}</p>
        {/if}
        <button type="submit" class="btn btn-primary justify-self-end">Save</button>
    </form>
    <p>{greetMsg}</p>
</div>
