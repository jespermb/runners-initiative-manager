<script lang="ts">
    export interface Step {
        id: number;
        label: string;
        active: boolean;
        disabled?: boolean;
    }

    interface Props {
        steps: Step[];
        onStepClick?: (stepId: number) => void;
    }

    const { steps, onStepClick }: Props = $props();

    function handleStepClick(step: Step) {
        if (!step.disabled && onStepClick) {
            onStepClick(step.id);
        }
    }
</script>

<div class="cyberpunk-steps">
    {#each steps as step, index}
        <button 
            type="button"
            class={`cyberpunk-step ${step.active ? 'active' : ''}`}
            onclick={() => handleStepClick(step)}
            disabled={step.disabled}
        >
            <div class="step-content">
                <span class="step-label">{step.label}</span>
            </div>
        </button>
    {/each}
</div>

<style>
    .cyberpunk-steps {
        display: flex;
        background-color: #0a0a0a;
        border: 1px solid #00f3ff;
        overflow: hidden;
        position: relative;
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
    }

    .cyberpunk-steps::before {
        content: '';
        position: absolute;
        top: -2px;
        left: -2px;
        right: -2px;
        height: 2px;
        background: linear-gradient(90deg, #ff00ff, #00f3ff);
        animation: cyberpunk-scan 2s linear infinite;
    }

    @keyframes cyberpunk-scan {
        0% {
            transform: translateY(0);
        }
        100% {
            transform: translateY(calc(100% + 4px));
        }
    }

    .cyberpunk-step {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: transparent;
        border: none;
        color: #c0c0c0;
        position: relative;
        overflow: hidden;
        transition: all 0.3s ease;
        padding: 0.75rem 0;
        cursor: pointer;
        clip-path: polygon(
            0% 0%, 
            calc(100% - 10px) 0%, 
            100% 50%, 
            calc(100% - 10px) 100%, 
            0% 100%
        );
    }

    .cyberpunk-step:last-child {
        clip-path: polygon(
            10px 0%, 
            100% 0%, 
            100% 100%, 
            10px 100%, 
            0% 50%
        );
    }

    .cyberpunk-step::after {
        content: '';
        position: absolute;
        top: 0;
        right: -10px;
        width: 20px;
        height: 100%;
        background-color: #0a0a0a;
        border-right: 1px solid #00f3ff;
        transform: skewX(-20deg);
        z-index: 1;
    }

    .cyberpunk-step:last-child::after {
        display: none;
    }

    .cyberpunk-step::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 2px;
        background: linear-gradient(90deg, #ff00ff, #00f3ff);
        transform: translateY(-100%);
        transition: transform 0.3s ease;
    }

    .cyberpunk-step.active {
        color: #ffffff;
    }

    .cyberpunk-step.active::before {
        transform: translateY(0);
    }

    .cyberpunk-step.active {
        background-color: rgba(0, 243, 255, 0.1);
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5) inset, 
                    0 0 20px rgba(0, 243, 255, 0.3) inset, 
                    0 0 30px rgba(0, 243, 255, 0.1) inset;
    }

    .cyberpunk-step.active::after {
        border-right-color: #00f3ff;
        box-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
    }

    .cyberpunk-step:disabled {
        cursor: default;
    }

    .step-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        z-index: 1;
    }

    .step-label {
        font-size: 0.85rem;
        text-transform: uppercase;
        letter-spacing: 1px;
        text-shadow: 0 0 5px rgba(0, 243, 255, 0.7);
    }
</style>