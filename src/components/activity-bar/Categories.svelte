<script lang="ts">
    import { accountStore } from "$lib/account.svelte";
    let addCategory = $state(false);
</script>

<div class="flex items-center justify-between">
    <h6>Categories</h6>
    <button aria-label="Add category" onclick={()=>addCategory = true}>
        <i class="ph ph-plus"></i>
    </button>
</div>
{#if addCategory}
    <div class="flex items-center">
        <input placeholder="Category" type="text" class="w-full border border-neutral-200 bg-neutral-100 px-1.5 py-0.5 rounded-sm">
        <button aria-label="Cancel" class="ml-2 mr-1" onclick={()=>addCategory = false}>
            <i class="ph ph-x"></i>
        </button>
        <button aria-label="Confirm">
            <i class="ph ph-check"></i>
        </button>
    </div>
{/if}

<ul class="space-y-2 text-neutral-700">
    {#each accountStore.categories as category}
        <li>{category.title}</li>
    {/each}
</ul>