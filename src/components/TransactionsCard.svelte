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

<section class="space-y-1.5">
    <header class="flex items-center justify-between">
        <h6>Transactions</h6>
        <Dialog title="Add transactions">
            {#snippet trigger()}
                <i class="ph ph-plus"></i>
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
    </header>
    <ul class="transaction-grid">
        <!--Table heading-->
        <li class="text-text-muted">Category</li>
        <li class="text-text-muted">Date</li>
        <li class="text-text-muted">Account</li>
        <li class="text-text-muted">Amount</li>
        <!--Table rows-->
        <div class="y-divider"></div>
        {#each accountStore.transactions as transaction (transaction.id)}
            {@const date = new Date(transaction.date)}
            {@const formatter = new Intl.NumberFormat("en-US", {style:"currency", currency:"USD"})}
            <li>{transaction.category.title}</li>
            <!--TODO: format date-->
            <li>{transaction.account.name}</li>
            <li>{date.toLocaleDateString("en-US", {year: "numeric", month:"long", day:"numeric"})}</li>
            <li>{formatter.format(transaction.amount)}</li>
            <div class="y-divider"></div>
        {/each}
    </ul>
</section>

<style>
    .transaction-grid{
        display: grid;
        grid-template-columns: repeat(4,1fr);
        gap: 12px;

        li{
            padding: 4px 0px;
        }
    }
</style>