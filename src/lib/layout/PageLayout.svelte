<script lang="ts">
    // PageLayout.svelte - A reusable layout component with cyberpunk styling
</script>

<div class="cyberpunk-container">
    <div class="cyberpunk-header">
        <slot name="header">
            <!-- Header content goes here -->
        </slot>
    </div>
    
    <div class="content-container">
        <slot name="content">
            <!-- Main content goes here -->
        </slot>
    </div>
    
    <slot name="fab">
        <!-- Floating action button goes here -->
    </slot>
    
    <slot name="navigation">
        <!-- Bottom navigation goes here -->
    </slot>
</div>

<style>
    .cyberpunk-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        position: relative;
    }

    .cyberpunk-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        border-bottom: 1px solid #00f3ff;
        margin-bottom: 1rem;
        position: relative;
        overflow: hidden;
    }

    .content-container {
        flex: 1;
        overflow-y: auto;
        max-height: calc(100vh - 15rem);
        margin-bottom: 1rem;
    }

    :global(.header-title) {
        color: white;
        font-size: 1.5rem;
        font-weight: 700;
        margin: 0;
        text-transform: uppercase;
        letter-spacing: 1px;
        position: relative;
        z-index: 1;
        text-shadow: 0 0 5px rgba(0, 243, 255, 0.7);
    }
</style>
