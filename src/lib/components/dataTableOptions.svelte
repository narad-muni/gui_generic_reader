<script lang="ts">
    import type { column } from "$lib/types";
    import {
        MultiSelect,
        Button,
    } from "flowbite-svelte";
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let columns: column[] = [];
    export let selectedColumns: string[] = [];
</script>

<div class="mt-2">
    <MultiSelect placeholder="select columns" items={columns} bind:value={selectedColumns}/>
</div>