<script lang="ts" generics="Option">
    import {Select} from "melt/builders";

    type Props = {
        options: Option[],
        label: string,
        /** Describes how to format the select item */
        format?: (value: Option) => string,
        value?: Option
    };

    const select = new Select<Option>();

    let { 
        options, 
        label, 
        format,
        value = $bindable()
    }: Props = $props();

    $effect(() => {
        value = select.value;
    });
</script>

<div class="select">
    <label {...select.label}>{label}</label>
    <button {...select.trigger}>
        {#if !select.value}
            Select an item
        {:else}
            {format ? format(select.value) : select.value}
        {/if}
    </button>
</div>

<ul {...select.content}>
    {#each options as option (option)}
        <li {...select.getOption(option)}>
            {#if option === select.value}
                <i class="ph ph-check"></i>
                {format ? format(option) : select.value}
            {:else}
                {format ? format(option) : select.value}
            {/if}
        </li>
    {/each}
</ul>

<style>
    .select{
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    button{
        display: flex;
        align-items: center;
        padding: 8px 12px;
        gap: 8px;
        background: var(--color-neutral-50);
        border-radius: var(--radius-md);
    }

    ul{
        border-radius: var(--radius-sm);
        box-shadow: var(--shadow-md);


    }

    li{
        display: flex;
        align-items: center;
        padding: 8px 12px;
        gap: 8px;
        transition: all 250ms;

        &:hover{
            background: var(--color-neutral-100);
        }

        &[aria-selected="true"]{
            color: white;
            background: var(--color-blue-500);
        }
    }
</style>

