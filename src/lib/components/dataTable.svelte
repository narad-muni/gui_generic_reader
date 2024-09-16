<script lang="ts">
    import { DataHandler } from '@vincjo/datatables';
    import {
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
        Input,
        Select,
        TableHeadCell
    } from "flowbite-svelte";
    import { check } from '@vincjo/datatables';
    import { createEventDispatcher } from 'svelte';
    import ThFilter from './thFilter.svelte';

    const dispatch = createEventDispatcher();
    let filterType: any = {};

    export let data: any[];
    export let columns: string[];
    export let selectedColumns: string[] = [];
    export let showSelected: boolean = false;

    const filterTypes = [
        {name:"isLike", value: check.isLike},
        {name:"isNotLike", value: check.isNotLike},
        {name:"startsWith", value: check.startsWith},
        {name:"endsWith", value: check.endsWith},
        {name:"isEqualTo", value: check.isEqualTo},
        {name:"isNotEqualTo", value: check.isNotEqualTo},
        {name:"isGreaterThan", value: check.isGreaterThan},
        {name:"isGreaterThanOrEqualTo", value: check.isGreaterThanOrEqualTo},
        {name:"isLessThan", value: check.isLessThan},
        {name:"isLessThanOrEqualTo", value: check.isLessThanOrEqualTo},
        // {name:"isBetween", value: check.isBetween},
        // {name:"isStrictlyBetween", value: check.isStrictlyBetween},
        {name:"isTrue", value: check.isTrue},
        {name:"isFalse", value: check.isFalse},
        {name:"isNull", value: check.isNull},
        {name:"isNotNull", value: check.isNotNull},
    ];

    $: console.log(data);
    $: handler = new DataHandler(data);
    $: rows = handler.getRows();
</script>

<Table divClass="mt-2 flex-grow overflow-y-auto mb-1">
    <TableHead>
        {#each Array.from(columns) as column}
            {#if selectedColumns.includes(column) || !showSelected}
                <TableHeadCell class="sticky top-0 *:mt-2 text-white bg-orange-500">
                    {column}
                    <Select bind:value={filterType[column]} items={filterTypes}></Select>
                    <ThFilter {handler} filterBy={column} filterType={filterType[column]}></ThFilter>
                </TableHeadCell>
            {/if}
        {/each}
    </TableHead>
    <TableBody>
        {#each $rows as row, i}
        <TableBodyRow on:dblclick={() => dispatch('rowClick', {id: i})}>
            {#each Array.from(columns) as column}
                {#if selectedColumns.includes(column) || !showSelected}
                    <TableBodyCell>{row[column]}</TableBodyCell>
                {/if}
            {/each}
        </TableBodyRow>
        {/each}
    </TableBody>
</Table>