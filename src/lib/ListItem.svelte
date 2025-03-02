<script lang="ts">
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();
    const { name = "" } = $props<{ name: string }>();

    async function remove() {
        dispatch("remove");
    }
</script>

<div class="encounter flex flex-row justify-between">
    <h2>{name}</h2>
    <button onclick={remove}>Remove</button>
</div>
