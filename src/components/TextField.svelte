<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLInputTypeAttribute } from "svelte/elements";

    type Props = {
        type?: HTMLInputTypeAttribute,
        name: string,
        label: string,
        placeholder?: string,
        prefixIcon?: Snippet,
        suffixIcon?: Snippet,
    };

    const {
        type="text",
        name, 
        prefixIcon,
        suffixIcon,
        label,
        placeholder,
    }: Props = $props();
</script>

<label for={name}>
    <p>{label}</p>
    <div>
        {@render prefixIcon?.()}
        <input {type} {name} {placeholder}>
        {@render suffixIcon?.()}
    </div>
</label>

<style>
    label{
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    div{
        display: flex;
        align-items: center;
        padding: 8px 12px;
        gap: 8px;
        background: var(--color-neutral-50);
        border-radius: var(--radius-md);

        &:has(input:focus){
            outline: 2px solid var(--color-blue-500);
        }
    }

    input{
        outline: none;
        width: 100%;
    }
</style>

