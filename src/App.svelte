<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Greet from "./lib/Greet.svelte";
    import { get } from "svelte/store";

  let combattens = [];

  async function getCombattens() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    combattens = await invoke("get_all_combattens");
  }

  async function removeCombatten(id) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.log("Id: " + id)
    await invoke("remove_combatten", { id });
    await getCombattens();
  }

  getCombattens();
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>
  <p>Current combattens:</p>
  <div class="list">
    {#each combattens as combatten}
      <div class="row"><p>{combatten.name}</p><button on:click={() => removeCombatten(combatten.id)}>Remove</button></div>
    {/each}
  </div>

  <div class="row">
    <Greet on:combattenAdded={getCombattens} />
  </div>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
  .list {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1em;
  }
  .row p {
    flex-grow: 1;
  }
  .row button {
    justify-self: flex-end;
  }
</style>
