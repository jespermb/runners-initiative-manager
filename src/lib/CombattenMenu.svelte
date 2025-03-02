<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
</script>

<div id="encounter-menu">
  <ul>
    <li><button on:click={() => dispatch("addNewCombatten")} class="menu-link">Add new combatten</button></li>
    <li><button on:click={() => dispatch("addCombatten")} class="menu-link">Add combatten</button></li>
  </ul>
</div>

<style>
  #encounter-menu {
    position: absolute;
    bottom: 90px;
    right: 30px;
    background-color: red;
    z-index: 100;
    padding: 10px;
    border-radius: 12px;
    border: solid 1px #fff;
    transition: all 0.2s ease-in-out;
  }
  .menu-link {
    color: #fff;
    text-decoration: none;
  }
</style>
