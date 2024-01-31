<script lang="ts">
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();
    export let name: string = "";

    async function remove() {
        dispatch("remove");
    }
</script>

<div class="encounter flex flex-row justify-between">
    <h2>{name}</h2>
    <button on:click={remove}>Remove</button>
</div>
