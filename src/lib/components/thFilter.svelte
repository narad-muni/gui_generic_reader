<script lang="ts">
    import type { Comparator, DataHandler } from "@vincjo/datatables";
    import { check } from '@vincjo/datatables';
    import { Input, Checkbox } from "flowbite-svelte";

    export let handler: DataHandler;
    export let filterBy: string;
    export let filterType: Comparator<any>;

    let value: string;
    let checked: boolean;
</script>

{#if [check.isTrue,check.isFalse,check.isNull,check.isNotNull].includes(filterType)}
    <Checkbox bind:checked on:change={() => {
        handler.filter(checked ? "x" : "", filterBy, filterType);
    }}></Checkbox>
{:else}
    <Input
    bind:value
    on:input={() => {
        if (filterBy) {
            handler.filter(value, filterBy, filterType);
        }
    }}
    ></Input>
{/if}