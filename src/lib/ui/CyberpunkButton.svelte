<script lang="ts">
    interface Props {
        variant: 'edit' | 'delete' | 'return' | 'primary' | 'secondary' | 'outline';
        size?: 'sm' | 'md' | 'lg';
        disabled?: boolean;
        onClick: () => void;
        ariaLabel?: string;
        children?: any;
    }

    const { 
        variant, 
        size = 'md', 
        disabled = false, 
        onClick, 
        ariaLabel,
        children
    }: Props = $props();

    const sizeClasses = {
        sm: 'w-6 h-6',
        md: 'w-8 h-8', 
        lg: 'w-10 h-10'
    };

    const variantClasses = {
        edit: 'border-[#00f3ff] text-[#00f3ff] hover:bg-[rgba(0,243,255,0.1)] hover:shadow-[0_0_8px_rgba(0,243,255,0.5)]',
        delete: 'border-[#ff00ff] text-[#ff00ff] hover:bg-[rgba(255,0,255,0.1)] hover:shadow-[0_0_8px_rgba(255,0,255,0.5)]',
        return: 'border-[#00f3ff] text-[#00f3ff] hover:bg-[rgba(0,243,255,0.1)] hover:shadow-[0_0_8px_rgba(0,243,255,0.5)]',
        primary: 'border-[#00f3ff] bg-[#00f3ff] text-[#0a0a0a] hover:shadow-[0_0_10px_rgba(0,243,255,0.7)]',
        secondary: 'border-[#ff00ff] bg-[#ff00ff] text-[#0a0a0a] hover:shadow-[0_0_10px_rgba(255,0,255,0.7)]',
        outline: 'border-[#00f3ff] text-[#00f3ff] hover:bg-[rgba(0,243,255,0.1)] hover:shadow-[0_0_8px_rgba(0,243,255,0.5)]'
    };

    function getIconSize() {
        return size === 'sm' ? 'h-4 w-4' : size === 'lg' ? 'h-6 w-6' : 'h-5 w-5';
    }
</script>

<button 
    class={`cyberpunk-btn ${sizeClasses[size]} ${variantClasses[variant]} ${disabled ? 'opacity-50 cursor-not-allowed' : ''}`}
    onclick={disabled ? undefined : onClick}
    aria-label={ariaLabel}
    {disabled}
>
    {#if variant === 'edit'}
        <svg xmlns="http://www.w3.org/2000/svg" class={getIconSize()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
        </svg>
    {:else if variant === 'delete'}
        <svg xmlns="http://www.w3.org/2000/svg" class={getIconSize()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
        </svg>
    {:else if variant === 'return'}
        <svg xmlns="http://www.w3.org/2000/svg" class={getIconSize()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
        </svg>
    {:else}
        <slot>{children}</slot>
    {/if}
</button>

<style>
    .cyberpunk-btn {
        border-radius: 0.25rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border: 1px solid;
        background-color: transparent;
        transition: all 0.2s ease;
        position: relative;
        z-index: 1;
        cursor: pointer;
    }

    .cyberpunk-btn:hover:not(:disabled) {
        transform: translateY(-1px);
    }

    .cyberpunk-btn:disabled {
        transform: none;
        box-shadow: none;
    }
</style>