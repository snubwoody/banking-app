<script lang="ts">
    import "../app.css";
    import type { Snippet } from "svelte";

    type Props = {
        children: Snippet
    };

    const {children}: Props = $props();
</script>

<svelte:head>
    <link
        rel="stylesheet"
        type="text/css"
        href="https://cdn.jsdelivr.net/npm/@phosphor-icons/web@2.1.1/src/regular/style.css"
    />
    <link
        rel="stylesheet"
        type="text/css"
        href="https://cdn.jsdelivr.net/npm/@phosphor-icons/web@2.1.1/src/fill/style.css"
    />
</svelte:head>
{@render children()}