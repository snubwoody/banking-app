<script lang="ts">
    import {createDialog} from "@melt-ui/svelte"; 
    import { type Snippet } from "svelte";

    type Props = {
        trigger: Snippet,
        body: Snippet
    };

    const {trigger: activate, body}: Props = $props();
    const {
        elements: {
            trigger,
            content,
            portalled,
        },
        states: {open}
    } = createDialog();
</script>

<button {...$trigger} use:trigger aria-label="Create account">
    {@render activate()}
</button>
{#if open}
    <div {...$portalled} use:portalled>
        <aside {...$content} use:content>
            {@render body()}
        </aside>
    </div>
{/if}

<style>
    aside{
        width: 100vw;
        height: 100vh;
        background-color: white;
        position: fixed;
        right: 0;
        top: 0;
        z-index: 100;
        width: clamp(150px,50vw,400px);
        border-left: 1px solid var(--color-neutral-200);
    }
</style>