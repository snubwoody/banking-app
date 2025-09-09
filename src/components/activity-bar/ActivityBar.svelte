<script>
    import Categories from "./Categories.svelte";

    let categoriesOpen = $state(false);
</script>

<section class="p-3 bg-white">
    <button aria-label="Toggle categories" onclick={() => categoriesOpen = !categoriesOpen}>
        <i class="ph ph-folder-simple text-[32px]"></i>
    </button>
</section>
{#if categoriesOpen}
    <section class="bg-neutral-50 border border-neutral-100 p-2.5 space-y-2.5 w-full max-w-40">
        <Categories/>
    </section>
{/if}