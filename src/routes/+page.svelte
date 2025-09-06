<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import type { Account, Category, Transaction } from "../lib/db";
    import CreateAccount from "../components/CreateAccount.svelte";
	
	let accounts: Account[] = $state([]);
    let categories: Category[] = $state([]);
    let transactions: Transaction[] = $state([]);
    let activeAccount: number | null = $state(null);

    async function fetchAccounts() {
        accounts = await invoke("fetch_accounts");
    }

    async function deleteAccount(id: number){
        await invoke("delete_account", {id});
        await fetchAccounts();
    }

    async function createTransaction(){
        if (!activeAccount){
            return;
        }

        await invoke("add_transaction", {
            account: activeAccount,
            amount: 500,
            category: 2,
            date: "2025-10-10"
        });
        transactions = await invoke("get_transactions", {account: activeAccount});
    }


    $effect(() => {
        fetchAccounts();
        invoke<Transaction[]>("get_transactions", {account: activeAccount})
            .then(val => transactions = val);
        invoke<Category[]>("get_categories")
            .then(val => categories = val);
    });

    const options = ["Phone", "Groceries"] as const;
    type Option = (typeof options)[number];
</script>

<main class="flex h-full">
    <aside class="max-w-[250px] flex-1 h-full bg-neutral-50 border-l border-neutral-300 p-3 space-y-4">
        <header class="flex items-center justify-between">
            <p class="text-lg">Accounts</p>
            <CreateAccount/>
        </header>
        <ul class="flex flex-col gap-4">
            {#each accounts as account}
                <li class="flex items-center justify-between">
                    <button onclick={() => { activeAccount = account.id; }}>
                        {account.name}
                    </button>
                    <button aria-label="Delete account" onclick={() => deleteAccount(account.id)}>
                        <i class="ph ph-x-circle"></i>
                    </button>
                </li>
            {/each}
        </ul>
    </aside>
    <section class="flex-1 p-5">
        <div class="flex items-center justify-between">
            <p class="text-lg">Transactions</p>
            <button aria-label="Add transaction" onclick={createTransaction}>
                <i class="ph ph-plus-circle"></i>
            </button>
        </div>
        <ul class="transaction-grid">
            <!--Table heading-->
            <li>Category</li>
            <li>Date</li>
            <li>Account</li>
            <li>Amount</li>
            <!--Table rows-->
            {#each transactions as transaction}
                <li>{transaction.category.title}</li>
                <!--TODO: format date-->
                <li>{transaction.date}</li>
                <li>{transaction.account.name}</li>
                <li>$ {transaction.amount}</li>
            {/each}
        </ul>
    </section>
</main>

<style>
    .transaction-grid{
        display: grid;
        grid-template-columns: repeat(4,1fr);
        gap: 20px;
    }
</style>

