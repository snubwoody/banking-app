<script lang="ts">
    import { accountStore } from "$lib/account.svelte";
    import type { AccountType } from "$lib/db";
    import Dialog from "./Dialog.svelte";
    import Select from "./Select.svelte";
    import TextField from "./TextField.svelte";

    let accountType: AccountType | undefined = $state();
    let accountName: string | undefined = $state();
    let startingBalance: number | undefined = $state();

    async function addAccount() {
        if (!accountType || !accountName || !startingBalance){
            return
        }

        accountStore.addAccount(accountName,accountType.id,startingBalance);
    }

</script>

<section>   
    <h6>Accounts</h6>
    <ul>
        {#each accountStore.accounts as account (account.id)}
            <li class="flex justify-between items-center">
                <div>
                    <p>{account.name}</p>
                    <p class="text-sm">{account.account_type.title}</p>
                </div>
                <!--TODO: Replace with balance-->
                <h6>$ {account.starting_balance}</h6>
            </li>
        {/each}
    </ul>
    <Dialog title="Add account">
        {#snippet trigger()}
            Add account
        {/snippet}
        {#snippet body()}
            <div>
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
</section>

<style>
    @reference "../styles/app.css";

    section{
        display: flex;
        flex-direction: column;
        gap: 20px;
        align-items: start;
        background: white;
        border-radius: var(--radius-md);
        padding: --spacing(2.5);
        box-shadow: var(--shadow-sm);
    }

    ul{
        width: 100%;
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(250px,1fr));
        gap: 24px;

        li{
            background: var(--color-neutral-50);
            border-radius: var(--radius-md);
            padding: 12px 16px;
        }
    }
</style>