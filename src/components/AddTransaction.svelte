<script lang="ts">
    import Drawer from "./Drawer.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Account, Transaction } from "../lib/db";
	

    async function createTransaction(){
        await invoke("add_transaction", {
            account: 1,
            amount: 500,
            category: 2,
            date: "2025-10-10"
        });
    }
</script>

<Drawer>
    {#snippet trigger()}
        <i class="ph ph-plus-circle"></i>
    {/snippet}
    {#snippet body()}
        <div class="p-3">
            <h6>Add transaction</h6>
        </div>
    {/snippet}
</Drawer>