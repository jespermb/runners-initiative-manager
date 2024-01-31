<script lang="ts">
    import Router from "svelte-spa-router";
    import Header from "./lib/Header.svelte";
    import routes from "./routes";
</script>

<main class="mx-auto flex flex-col p-2">
    <div class="glow-border pr-4 pl-4 pb-4 flex flex-col">
        <Header />
        <Router {routes} />
    </div>
</main>

<style>
    :global(html) {
        --neon-text-color: #f40;
        --neon-border-color: #08f;
    }
    main {
        min-height: calc(100vh - 1rem);
    }
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
    .glow-border {
        border: 1px solid #fff;
        height: calc(100vh - 1rem);
        width: 100%;
        border-radius: 1rem;

        box-shadow: 0 0 4px #fff, inset 0 0 4px #fff,
            0 0 8px var(--neon-border-color),
            inset 0 0 8px var(--neon-border-color),
            0 0 16px var(--neon-border-color),
            inset 0 0 16px var(--neon-border-color);
    }
</style>
