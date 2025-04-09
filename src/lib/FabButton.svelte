<script lang="ts">
    // Define props
    const { onClick } = $props();
</script>

<div class="fab">
    <button on:click={onClick}>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
    </button>
</div>

<style>
    /* FAB Styling - Cyberpunk theme */
    .fab {
        position: absolute;
        bottom: 2rem;
        left: 50%;
        transform: translateX(-50%);
        z-index: 20;
    }

    .fab button {
        width: 3.5rem;
        height: 3.5rem;
        border-radius: 50%;
        background-color: #0a0a0a;
        color: #00f3ff;
        display: flex;
        align-items: center;
        justify-content: center;
        border: 1px solid #00f3ff;
        box-shadow: 0 0 8px rgba(0, 243, 255, 0.5);
        transition: all 0.2s ease;
        position: relative;
        z-index: 1;
    }

    .fab button:hover {
        background-color: #0a0a0a;
        border-color: #00f3ff;
        box-shadow: 0 0 15px rgba(0, 243, 255, 0.7);
    }
</style>
