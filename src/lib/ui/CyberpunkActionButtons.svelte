<script lang="ts">
    interface Button {
        label: string;
        type?: 'primary' | 'secondary' | 'outline';
        disabled?: boolean;
        flex?: boolean;
        onclick: () => void;
    }

    interface Props {
        buttons: Button[];
        position?: 'fixed' | 'relative';
    }

    const { buttons, position = 'fixed' }: Props = $props();
</script>

<div class={`cyberpunk-action-buttons ${position === 'fixed' ? 'fixed-position' : ''}`}>
    <div class="button-container">
        {#each buttons as button}
            <button 
                type="button" 
                class={`btn ${button.type === 'primary' ? 'btn-primary' : button.type === 'secondary' ? 'btn-secondary' : 'btn-outline'} ${button.flex ? 'flex-1' : ''}`}
                onclick={button.onclick}
                disabled={button.disabled}
            >
                {button.label}
            </button>
        {/each}
    </div>
</div>

<style>
    .cyberpunk-action-buttons {
        padding: 1rem;
        background-color: #0a0a0a;
        border-top: 1px solid #00f3ff;
        box-shadow: 0 -5px 15px rgba(0, 243, 255, 0.2);
        z-index: 1000;
    }

    .cyberpunk-action-buttons.fixed-position {
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        margin-top: auto;
    }

    .cyberpunk-action-buttons::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 1px;
        background: linear-gradient(90deg, #ff00ff, #00f3ff);
        animation: cyberpunk-scan-horizontal 2s linear infinite;
    }

    @keyframes cyberpunk-scan-horizontal {
        0% {
            transform: translateX(-100%);
        }
        100% {
            transform: translateX(100%);
        }
    }

    .button-container {
        display: flex;
        gap: 0.5rem;
    }

    .btn {
        position: relative;
        overflow: hidden;
        background-color: transparent;
        border: 1px solid #00f3ff;
        color: #00f3ff;
        text-transform: uppercase;
        letter-spacing: 1px;
        transition: all 0.3s ease;
        padding: 0.5rem 1rem;
        border-radius: 0.25rem;
        cursor: pointer;
    }

    .btn::after {
        content: '';
        position: absolute;
        top: -50%;
        left: -50%;
        width: 200%;
        height: 200%;
        background: linear-gradient(rgba(0, 243, 255, 0.1), transparent);
        transform: rotate(30deg);
        animation: cyberpunk-btn-shine 3s linear infinite;
    }

    @keyframes cyberpunk-btn-shine {
        0% {
            transform: rotate(30deg) translateY(-100%);
        }
        100% {
            transform: rotate(30deg) translateY(100%);
        }
    }

    .btn-primary {
        background-color: #00f3ff;
        color: #0a0a0a;
        border-color: #00f3ff;
    }

    .btn-secondary {
        background-color: #ff00ff;
        color: #0a0a0a;
        border-color: #ff00ff;
    }

    .btn-outline {
        background-color: transparent;
        color: #00f3ff;
        border-color: #00f3ff;
    }

    .btn:hover {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.7);
        transform: translateY(-2px);
    }

    .btn-primary:hover {
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.7);
    }

    .btn-secondary:hover {
        box-shadow: 0 0 10px rgba(255, 0, 255, 0.7);
    }

    .btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
        transform: none;
        box-shadow: none;
    }

    .flex-1 {
        flex: 1;
    }
</style>