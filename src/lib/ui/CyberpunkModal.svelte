<script lang="ts">
    interface Props {
        title: string;
        isOpen: boolean;
        onClose: () => void;
        size?: 'sm' | 'md' | 'lg' | 'xl';
    }

    const { title, isOpen, onClose, size = 'md' }: Props = $props();

    const sizeClasses = {
        sm: 'max-w-sm',
        md: 'max-w-md', 
        lg: 'max-w-lg',
        xl: 'max-w-xl'
    };

    function handleBackdropClick(event: MouseEvent) {
        if (event.target === event.currentTarget) {
            onClose();
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') {
            onClose();
        }
    }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div 
        class="modal-backdrop" 
        onclick={handleBackdropClick}
        onkeydown={handleKeydown}
        role="dialog"
        aria-modal="true"
        aria-labelledby="modal-title"
    >
        <div class={`modal-content ${sizeClasses[size]}`}>
            <div class="modal-header">
                <h2 id="modal-title" class="modal-title">{title}</h2>
            </div>
            
            <div class="modal-body">
                <slot></slot>
            </div>
        </div>
    </div>
{/if}

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(0, 0, 0, 0.7);
        backdrop-filter: blur(3px);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        padding: 1rem;
    }

    .modal-content {
        background-color: #0a0a0a;
        color: white;
        border: 1px solid #00f3ff;
        border-radius: 0.5rem;
        box-shadow: 0 0 15px rgba(0, 243, 255, 0.5);
        animation: cyberpunk-glow 1.5s ease-in-out infinite alternate;
        width: 90%;
        max-height: 90vh;
        overflow-y: auto;
        position: relative;
    }

    .modal-content::before {
        content: '';
        position: absolute;
        top: -2px;
        left: -2px;
        right: -2px;
        height: 2px;
        background: linear-gradient(90deg, #ff00ff, #00f3ff);
        animation: cyberpunk-scan 2s linear infinite;
    }

    @keyframes cyberpunk-glow {
        from {
            box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
        }
        to {
            box-shadow: 0 0 20px rgba(0, 243, 255, 0.8), 0 0 30px rgba(255, 0, 255, 0.3);
        }
    }

    @keyframes cyberpunk-scan {
        0% {
            transform: translateY(0);
        }
        100% {
            transform: translateY(calc(100% + 4px));
        }
    }

    .modal-header {
        padding: 1rem 1rem 0.5rem 1rem;
        border-bottom: 1px solid #333;
    }

    .modal-title {
        font-size: 1.125rem;
        font-weight: 700;
        margin: 0;
        color: #00f3ff;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    .modal-body {
        padding: 1rem;
    }

    /* Global modal button styling */
    :global(.modal-content .btn) {
        background-color: transparent;
        border: 1px solid #00f3ff;
        color: #00f3ff;
        text-transform: uppercase;
        letter-spacing: 1px;
        position: relative;
        overflow: hidden;
        transition: all 0.3s ease;
        padding: 0.5rem 1rem;
        border-radius: 0.25rem;
        cursor: pointer;
    }

    :global(.modal-content .btn::before) {
        content: '';
        position: absolute;
        top: 0;
        left: -100%;
        width: 100%;
        height: 100%;
        background: linear-gradient(90deg, transparent, rgba(0, 243, 255, 0.2), transparent);
        transition: all 0.5s ease;
    }

    :global(.modal-content .btn:hover::before) {
        left: 100%;
    }

    :global(.modal-content .btn-primary) {
        background-color: #00f3ff;
        color: #0a0a0a;
    }

    :global(.modal-content .btn-secondary) {
        background-color: #ff00ff;
        border-color: #ff00ff;
        color: #0a0a0a;
    }

    :global(.modal-content .btn:hover) {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.7);
        transform: translateY(-2px);
    }

    :global(.modal-content .btn-primary:hover) {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.7);
    }

    :global(.modal-content .btn-secondary:hover) {
        box-shadow: 0 0 10px rgba(255, 0, 255, 0.7);
    }

    /* Global modal input styling */
    :global(.modal-content input) {
        background-color: #1a1a1a;
        border: 1px solid #00f3ff;
        color: white;
        transition: all 0.3s ease;
    }

    :global(.modal-content input:focus) {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
        outline: none;
        border-color: #ff00ff;
    }
</style>