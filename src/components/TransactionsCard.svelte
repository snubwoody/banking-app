<script lang="ts">
    import {accountStore} from "$lib/account.svelte";
    import Dialog from "./Dialog.svelte";
    import TextField from "$components/TextField.svelte";
    import Select from "./Select.svelte";
    import type { Account, Category } from "$lib/db";


    let category: Category;
    let account: Account;
    let amount: number;
    let date: string;
    async function addTransaction() {
        if (!category || !account || !amount ||!date){
            return;
        }

        await accountStore.addTransaction(amount, account, category, date);
    }
</script>

<section class="rounded-sm shadow-sm space-y-1 p-2.5 bg-white">
    <div class="flex items-center justify-between">
        <p class="text-lg">Transactions</p>
        <Dialog title="Add transactions">
            {#snippet trigger()}
                <i class="ph ph-plus-circle"></i>
            {/snippet}
            {#snippet body()}
                <div class="space-y-2.5">
                    <TextField bind:value={amount} label="Amount" type="number" name="amount"/>
                    <Select 
                        label="Account" 
                        placeholder="Select an account"
                        options={accountStore.accounts} 
                        bind:value={account}
                        format={(item) => item.name}
                    />
                    <TextField bind:value={date} label="Date" type="date" name="date"/>
                    <Select 
                        label="Category" 
                        placeholder="Select a category"
                        options={accountStore.categories} 
                        bind:value={category}
                        format={(item) => item.title}
                    />
                </div>
            {/snippet}
            {#snippet actions()}
                <button onclick={addTransaction} class="btn btn-primary">Add transaction</button>
            {/snippet}
        </Dialog>
    </div>
    <ul class="transaction-grid">
        <!--Table heading-->
        <li>Category</li>
        <li>Date</li>
        <li>Account</li>
        <li>Amount</li>
        <!--Table rows-->
        {#each accountStore.transactions as transaction (transaction.id)}
            <li>{transaction.category.title}</li>
            <!--TODO: format date-->
            <li>{transaction.date}</li>
            <li>{transaction.account.name}</li>
            <li>$ {transaction.amount}</li>
        {/each}
    </ul>
</section>

<style>
    .transaction-grid{
        display: grid;
        grid-template-columns: repeat(4,1fr);
        gap: 20px;
    }
</style>