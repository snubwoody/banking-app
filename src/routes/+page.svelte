<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import type { Account, Transaction } from "../lib/db";
    import CreateAccount from "../components/CreateAccount.svelte";
    import AddTransaction from "../components/AddTransaction.svelte";
    import AccountsCard from "../components/AccountsCard.svelte";
	
    // TODO: make global transaction store
	let accounts: Account[] = $state([]);
    let transactions: Transaction[] = $state([]);
    let activeAccount: number | null = $state(null);

    async function fetchAccounts() {
        accounts = await invoke("fetch_accounts");
    }

    async function deleteAccount(id: number){
        await invoke("delete_account", {id});
        await fetchAccounts();
    }

    $effect(() => {
        fetchAccounts();
        invoke<Transaction[]>("get_transactions", {account: activeAccount})
            .then(val => transactions = val);
    });
</script>

<main class="flex h-full">
    <aside class="max-w-[250px] flex-1 h-full  border-l border-neutral-300 p-3 space-y-4">
        <header class="flex items-center justify-between">
            <p class="text-lg">Accounts</p>
            <CreateAccount/>
        </header>
        <ul class="flex flex-col gap-4">
            {#each accounts as account (account.id)}
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
    <section class="flex-1 p-5 space-y-3">
        <section class="rounded-sm shadow-sm space-y-1 p-2.5 bg-white">
            <p>Total Net Worth</p>
            <h5>$ 156,242.24</h5>
        </section>
        <AccountsCard/>
        <section class="rounded-sm shadow-sm space-y-1 p-2.5 bg-white">
            <div class="flex items-center justify-between">
                <p class="text-lg">Transactions</p>
                <AddTransaction/>
            </div>
            <ul class="transaction-grid">
                <!--Table heading-->
                <li>Category</li>
                <li>Date</li>
                <li>Account</li>
                <li>Amount</li>
                <!--Table rows-->
                {#each transactions as transaction (transaction.id)}
                    <li>{transaction.category.title}</li>
                    <!--TODO: format date-->
                    <li>{transaction.date}</li>
                    <li>{transaction.account.name}</li>
                    <li>$ {transaction.amount}</li>
                {/each}
            </ul>
        </section>
    </section>
</main>

<style>
    main{
        background-color: #F5F6FA;
    }

    .transaction-grid{
        display: grid;
        grid-template-columns: repeat(4,1fr);
        gap: 20px;
    }
</style>

