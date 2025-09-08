<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import type { Account, Transaction } from "../lib/db";
    import CreateAccount from "../components/CreateAccount.svelte";
    import AddTransaction from "../components/AddTransaction.svelte";
    import AccountsCard from "../components/AccountsCard.svelte";
	
	let accounts: Account[] = $state([]);
    let transactions: Transaction[] = $state([]);
    let activeAccount: number | null = $state(null);

    async function fetchAccounts() {
        accounts = await invoke("fetch_accounts");
    }

    $effect(() => {
        fetchAccounts();
        invoke<Transaction[]>("get_transactions", {account: activeAccount})
            .then(val => transactions = val);
    });
</script>

<main class="flex h-full">
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

