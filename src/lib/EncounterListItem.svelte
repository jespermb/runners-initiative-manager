<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher } from "svelte";
    import ListItem from "./ListItem.svelte";
    const dispatch = createEventDispatcher();

    const { name = "", id = 0 } = $props();
    let resultMsg = $state("");

    async function remove() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        resultMsg = await invoke("remove_encounter", { id });
        dispatch("encounterRemoved");
    }
</script>

<ListItem {name} on:remove={remove} />
