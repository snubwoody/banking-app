<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import type { Account } from "../lib/db";
	
	let accounts: Account[] = $state([]);
	async function createAccount() {
		const account = await invoke("create_account", { name: "Transactional" });
        await fetchAccounts();
	}

    async function fetchAccounts() {
        accounts = await invoke("fetch_accounts");
    }

    async function deleteAccount(id: string){
        await invoke("delete_account", {id})
        await fetchAccounts();
    }


    $effect(()=>{
        fetchAccounts();
    })
</script>

<main class="flex h-full">
    <aside class="max-w-[250px] flex-1 h-full bg-neutral-50 border-l border-neutral-300 p-3 space-y-4">
        <header class="flex items-center justify-between">
            <p class="text-lg">Accounts</p>
            <button onclick={createAccount} aria-label="Create account">
                <i class="ph ph-plus-circle"></i>
            </button>
        </header>
        <ul class="flex flex-col gap-4">
            {#each accounts as account}
                <li class="flex items-center justify-between">
                    {account.name}
                    <button aria-label="Delete account" onclick={()=>deleteAccount(account.id)}>
                        <i class="ph ph-x-circle"></i>
                    </button>
                </li>
            {/each}
        </ul>
    </aside>
    <section class="flex-1">

    </section>
</main>

<style>
    
</style>
