<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Greet from "./lib/Greet.svelte";

  let combattens = [];

  async function getCombattens() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    combattens = await invoke("get_all_items");
  }

  getCombattens();
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>
  <p>Current combattens:</p>
  <div class="row">
    {#each combattens as combatten}
      <div>{combatten}</div>
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
</style>
