<script lang="ts">
  import Router from 'svelte-spa-router';
  import Header from "./lib/Header.svelte";
  import routes from './routes';
</script>

<main class="container flex">
  <Header />
  <Router {routes}/>
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
