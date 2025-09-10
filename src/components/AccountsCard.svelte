<script lang="ts">
    import { accountStore } from "$lib/account.svelte";
    import type { AccountType } from "$lib/db";
    import Dialog from "./Dialog.svelte";
    import Select from "./Select.svelte";
    import TextField from "./TextField.svelte";

    let accountType: AccountType | undefined = $state();
    let accountName: string | undefined = $state();
    let startingBalance: number = $state(0);

    async function addAccount() {
        if (!accountType || !accountName){
            return;
        }

        accountStore.addAccount(accountName, accountType.id, startingBalance);
    }
</script>

<section>
    <header class="flex w-full items-center justify-between">
        <h6>Accounts</h6>
        <Dialog title="Add account">
            {#snippet trigger()}
                <i class="ph ph-plus"></i>
            {/snippet}
            {#snippet body()}
                <div class="space-y-2.5">
                    <TextField bind:value={accountName} label="Name" name="accountName" placeholder="Account name"/>
                    <Select 
                        label="Account type"
                        bind:value={accountType}
                        options={accountStore.accountTypes}
                        format={(item) => item.title}
                    />
                    <TextField bind:value={startingBalance} label="Starting balance" type="number" name="balance" placeholder="0.00"/>
                </div>
            {/snippet}
            {#snippet actions()}
                <button onclick={addAccount} class="btn btn-primary w-full">
                    <i class="ph ph-plus"></i>
                    Add account
                </button>
            {/snippet}
        </Dialog>
    </header>   
    <div class="flex flex-col gap-1">
        <p class="text-text-muted">Total balance</p>
        <h6>$ 24,264.35</h6>
    </div>
    <ul>
        <!--Table header-->
        <li class="text-text-muted">Name</li>
        <li class="text-text-muted">Account type</li>
        <li class="text-text-muted">Balance</li>
        <!--Table rows-->
        {#each accountStore.accounts as account (account.id)}
            <li>{account.name}</li>
            <li>{account.account_type.title}</li>
            <!--TODO: Replace with balance-->
            <li>$ {account.starting_balance}</li>
            <div class="y-divider"></div>
        {/each}
    </ul>

</section>

<style>
    @reference "../styles/app.css";

    section{
        display: flex;
        flex-direction: column;
        gap: 20px;
        align-items: start;
    }

    ul{
        width: 100%;
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 12px;
        @apply divide-y divide-neutral-100;

        li{
            padding: 4px 0px;
        }
    }
</style>