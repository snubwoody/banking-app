<script lang="ts">
    import {createDialog,melt} from "@melt-ui/svelte"; 
    import {fly} from "svelte/transition";
    import {invoke} from "@tauri-apps/api/core";
    import type { AccountType } from "$lib/db";
    import {Select} from "melt/builders";
    import { onMount } from "svelte";

    let accountTypes = $state<AccountType[]>([]);
    let accountType: AccountType | undefined;
    let name: string = $state("");
    let startingBalance: number = $state(0);
    onMount(()=>{
        invoke<AccountType[]>("get_account_types").then(val => accountTypes = val);
    });

    const {
        elements: {
            trigger,
            overlay,
            title,
            description,
            close,
            content,
            portalled,
        },
        states: {open}
    } = createDialog();

    const select = new Select<AccountType>({
        onValueChange: (value) => {accountType = value}
    });

    async function createAccount(){
        if(name === "" || !accountType){
            return
        }
    }
</script>

<button {...$trigger} use:trigger aria-label="Create account">
    <i class="ph ph-plus-circle"></i>
</button>
{#if open}
    <div {...$portalled} use:portalled>
        <aside transition:fly={{x: 250}} {...$content} use:content>
            <h2 {...$title} use:title class="text-2xl">Add account</h2>
            <div class="flex flex-col gap-5">
                <label class="space-y-2">
                    <p>Name</p>
                    <input bind:value={name} type="text" class="input-field">
                </label>
                <label class="space-y-2">
                    <p>Starting balance</p>
                    <input bind:value={startingBalance} type="number" class="input-field">
                </label>
                <label {...select.label} class="space-y-2">
                    <p>Account type</p>
                    <button {...select.trigger} class="input-field flex justify-between items-center w-full">
                        {select.value?.title ?? "Select an account type"}
                        <i class="ph ph-caret-down"></i>
                    </button>
                </label>
                <ul {...select.content} class="shadow bg-white rounded-2xl p-2">
                    {#each accountTypes as accountType}
                         <li {...select.getOption(accountType)} class="hover:bg-neutral-50 transition-all p-2 rounded-2xl">
                            {accountType.title}
                        </li>
                    {/each}
                </ul>
            </div>
            <button class="bg-[#005F78] p-3 rounded-lg text-white mt-auto">Confirm</button>
        </aside>
    </div>
{/if}

<style>
    aside{
        width: 100vw;
        height: 100vh;
        display: grid;
        grid-template-rows: auto 1fr auto; 
        gap: 20px;
        position: fixed;
        right: 0;
        top: 0;
        z-index: 100;
        width: clamp(150px,50vw,400px);
        padding: 24px;
        border-left: 1px solid var(--color-neutral-200);
    }

    .input-field{
        width: 100%;
        border: 1px solid var(--color-neutral-200);
        border-radius: 8px;
        padding: 12px;
    }
</style>