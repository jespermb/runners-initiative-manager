<script lang="ts">
  import Header from "../lib/Header.svelte";
  import "../styles.css";

  let { children } = $props<{}>();
</script>

<main class="mx-auto flex flex-col p-2 pt-4">
  <div class="cyberpunk-container flex flex-col">
      <Header />
      <div class="content-wrapper px-4">
        {@render children()}
      </div>
  </div>
</main>

<style>
  :global(html) {
      --neon-text-color: #ff00ff;
      --neon-border-color: #00f3ff;
      --neon-bg-color: #0a0a0a;
      background-color: black;
      color: white;
  }

  :global(body) {
      background-color: black;
      color: white;
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      margin: 0;
      padding: 0;
      overflow: hidden;
  }

  :global(button) {
      cursor: pointer;
  }

  :global(a) {
      color: var(--neon-border-color);
      text-decoration: none;
  }

  :global(a:hover) {
      text-shadow: 0 0 5px var(--neon-border-color);
  }

  main {
      min-height: calc(100vh - 1rem);
  }

  .cyberpunk-container {
      border: 1px solid var(--neon-border-color);
      border-radius: 0.5rem;
      height: calc(100vh - 2rem);
      width: 100%;
      background-color: var(--neon-bg-color);
      position: relative;
      overflow: hidden;
      display: flex;
      flex-direction: column;

      box-shadow: 0 0 4px var(--neon-border-color), 
                  inset 0 0 4px var(--neon-border-color),
                  0 0 8px var(--neon-border-color),
                  inset 0 0 8px var(--neon-border-color);
  }
  
  .content-wrapper {
      flex: 1;
      display: flex;
      flex-direction: column;
      overflow: hidden;
  }

  .cyberpunk-container::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background: linear-gradient(45deg, 
          rgba(0, 243, 255, 0.05) 0%, 
          rgba(0, 0, 0, 0) 70%);
      z-index: 0;
      pointer-events: none;
  }

  .cyberpunk-container::after {
      content: '';
      position: absolute;
      top: 0;
      right: 0;
      width: 30%;
      height: 1px;
      background: linear-gradient(90deg, 
          rgba(0, 0, 0, 0) 0%, 
          var(--neon-text-color) 100%);
      box-shadow: 0 0 10px var(--neon-text-color);
      z-index: 1;
      pointer-events: none;
  }
</style>
