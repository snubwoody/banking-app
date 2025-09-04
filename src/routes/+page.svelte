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

<main>
    <button onclick={createAccount} class="bg-blue-500 text-white rounded-2xl p-3 cursor-pointer">
        Create an account
    </button>
    <ul class="flex flex-col gap-4">
        {#each accounts as account}
            <li>
                {account.name}
                <button onclick={()=>deleteAccount(account.id)}>Delete</button>
            </li>
        {/each}
    </ul>
</main>

<style>
    
</style>
