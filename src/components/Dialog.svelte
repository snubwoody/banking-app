<script lang="ts">
    import {createDialog} from "@melt-ui/svelte"; 
    import { type Snippet } from "svelte";
    import { fade } from "svelte/transition";

    type Props = {
        trigger: Snippet,
        body: Snippet,
        actions?: Snippet,
        title: string
    };

    const {
        trigger: activate, 
        body, 
        actions,
        title
    }: Props = $props();

    const {
        elements: {
            trigger,
            content,
            portalled,
            overlay
        },
        states: {open}
    } = createDialog();
</script>

<button {...$trigger} use:trigger>
    {@render activate()}
</button>
{#if open}
    <div {...$portalled} use:portalled>
        <div {...$overlay} use:overlay transition:fade class="fixed z-50 inset-0 bg-black/20 transition-all"></div>
        <div {...$content} use:content class="dialog-content">
            <header class="grid place-items-center">
                <h6>{title}</h6>
            </header>
            {@render body()}
            {#if actions}
                <footer class="flex gap-1 items-center justify-end">
                    {@render actions?.()}
                </footer>
            {/if}
        </div>
    </div>
{/if}

<style>
    .dialog-content{
        display: flex;
        flex-direction: column;
        background-color: white;
        position: fixed;
        max-width: 400px;
        height: fit-content;
        inset: 0;
        margin: auto;
        z-index: 100;
        padding: 24px;
        gap: 24px;
        border-radius: var(--radius-2xl);
        box-shadow: var(--shadow-lg);
    }
</style>