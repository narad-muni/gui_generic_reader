<script lang="ts">
    import { getFileName } from "$lib/utils/utils";
    import { open } from "@tauri-apps/api/dialog";
    import {
        Button,
        Select,
        FloatingLabelInput,
    } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let config_path: string | null;
    export let file_path: string | null;
    export let selectedType: string;
    export let page_size: number;
    export let page_number: number;
    export const fileTypes = [
        {name: "CSV", value: "csv"},
        {name: "JSON", value: "json"},
        {name: "JSON Array", value: "json_array"},
        {name: "JSON Lines", value: "json_lines"},
        {name: "Fixed Native", value: "native"},
        {name: "Multi Native", value: "multi_native"},
    ];

    async function openConfigFile() {
        config_path = await open() as string;
    }

    async function openFile() {
        file_path = await open() as string;
    }
</script>

<div class="grid grid-cols-8 gap-4 mt-2">
    <Button on:click={openConfigFile}>
        {#if config_path === null}
            Select config
        {:else}
            {getFileName(config_path)}
        {/if}
    </Button>

    <Button on:click={openFile}>
        {#if file_path === null}
            Select file
        {:else}
            {getFileName(file_path)}
        {/if}
    </Button>

    <Select
        bind:value={selectedType}
        items={fileTypes}
    />

    <FloatingLabelInput bind:value={page_number} type="number">
        Page Number
    </FloatingLabelInput>

    <FloatingLabelInput bind:value={page_size} type="number">
        Page Size
    </FloatingLabelInput>

    <Button on:click={() => dispatch('submit')}>
        Load Table
    </Button>

    <Button on:click={() => dispatch('previous')}>Previous</Button>
    <Button on:click={() => dispatch('next')}>Next</Button>
</div>