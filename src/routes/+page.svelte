<script lang="ts">
    import ActivityBar from "$components/activity-bar/ActivityBar.svelte";
    import AccountsCard from "../components/AccountsCard.svelte";
    import TransactionsCard from "../components/TransactionsCard.svelte";
	
</script>

<main class="flex h-full">
    <ActivityBar/>
    <section class="flex-1 p-5 space-y-3">
        <AccountsCard/>
        <TransactionsCard/>
    </section>
</main>

<style>
    main{
        display: flex;
    }
</style>

