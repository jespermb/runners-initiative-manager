<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";
    import ListItem from "./ListItem.svelte";
    const dispatch = createEventDispatcher();

    export let name = "";
    export let id = 0;
    let resultMsg = "";

    async function remove() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        resultMsg = await invoke("remove_combatten", { id });
        name = "";
        dispatch("combattenRemoved");
    }
</script>

<ListItem {name} on:remove={remove} />
