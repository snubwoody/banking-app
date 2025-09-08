<script lang="ts">
    import type { Transaction } from "../lib/db";
    import AccountsCard from "../components/AccountsCard.svelte";
    import TransactionsCard from "../components/TransactionsCard.svelte";
	
</script>

<main class="flex h-full">
    <section class="flex-1 p-5 space-y-3">
        <section class="rounded-sm shadow-sm space-y-1 p-2.5 bg-white">
            <p>Total Net Worth</p>
            <h5>$ 156,242.24</h5>
        </section>
        <AccountsCard/>
        <TransactionsCard/>
    </section>
</main>

<style>
    main{
        background-color: #F5F6FA;
    }
</style>

