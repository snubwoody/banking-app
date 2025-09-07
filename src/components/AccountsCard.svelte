<script lang="ts">
    import { accountStore } from "$lib/account.svelte";
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
                <!--Replace with balance-->
                <h6>$ {account.starting_balance}</h6>
            </li>
        {/each}
    </ul>
    <button>Add account</button>
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