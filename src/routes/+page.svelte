<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	
	
	async function createAccount() {
		const account = await invoke("create_account", { name: "Transactional" });
        console.log(account);
	}
</script>

<main>
    <button onclick={createAccount} class="bg-blue-500 text-white rounded-2xl p-3 cursor-pointer">
        Create account
    </button>
</main>

<style>
    
</style>
