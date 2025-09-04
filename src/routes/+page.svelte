<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	
	let accounts = $state([]);
	async function createAccount() {
		const account = await invoke("create_account", { name: "Transactional" });
        console.log(account);
	}

    async function fetchAccounts() {
        accounts = await invoke("fetch_accounts");
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
            </li>
        {/each}
    </ul>
</main>

<style>
    
</style>
