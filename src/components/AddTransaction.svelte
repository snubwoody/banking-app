<script lang="ts">
    import Drawer from "./Drawer.svelte";
</script>

<Drawer>
    {#snippet trigger()}
        <i class="ph ph-plus-circle"></i>
    {/snippet}
    {#snippet body()}
        <form class="p-3">
            <h6>Add transaction</h6>
            <label class="space-y-1">
                <p>Amount</p>
                <input type="text" name="" id="" class="input-field">
            </label>
            <label class="space-y-1">
                <p>Account type</p>
                <input type="text" name="" id="" class="input-field">
            </label>
        </form>
    {/snippet}
</Drawer>