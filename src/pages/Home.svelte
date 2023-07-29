<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import AddCampaign from "../lib/AddCampaign.svelte";

  let combattens = [];

  async function getCampaigns() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    combattens = await invoke("get_all_campaigns");
  }

  async function removeCampaign(id) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.log("Id: " + id);
    await invoke("remove_campaign", { id });
    await getCampaigns();
  }

  getCampaigns();
</script>

<h2>Campaigns</h2>
<div class="list flex">
  {#each combattens as combatten}
    <div class="row">
      <p>{combatten.name}</p>
      <button on:click={() => removeCampaign(combatten.id)}>Remove</button>
    </div>
  {/each}
</div>

<div class="row">
  <AddCampaign on:campaignAdded={getCampaigns} />
</div>

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
