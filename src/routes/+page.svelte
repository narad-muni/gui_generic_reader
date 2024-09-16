<script lang="ts">
    import DataTable from "$lib/components/dataTable.svelte";
    import DataTableOptions from "$lib/components/dataTableOptions.svelte";
    import FileOpeners from "$lib/components/fileOpeners.svelte";
    import type { column } from "$lib/types";
    import { invoke } from "@tauri-apps/api/tauri";
    import { Modal, Table, TableBody, TableBodyCell, TableBodyRow } from "flowbite-svelte";

    // File Opener props
    let config_path: string | null = null;
    let file_path: string | null = null;
    let page_size = 10;
    let page_number = 1;
    let selectedType = "csv";
    
    // Data Table props
    let data: object[] = [];
    let columns: string[] = [];
    let selectedColumns: string[] = [];
    
    //Modal
    let openModal = false;
    let modalData = {};
    
    $: showSelected = selectedColumns.length > 0;
    
    $: columnsProps = Array.from(columns).map(column => {
        return {name: column, value: column} as column;
    });

    // File Opener functions
    async function openFile() {
        await loadData();
        await setSelectedColumns();
    }

    async function loadData() {
        // Validation
        if (config_path === null || file_path === null) {
            alert("Please select config and file");
            return;
        }

        if (page_size < 1 || page_number < 1) {
            alert("Page size and page number must be greater than 0");
            return;
        }

        // Fetch data
        data = await invoke("open", {
            configPath: config_path,
            filePath: file_path,
            type: selectedType,
            from: (page_number - 1) * page_size,
            len: Number(page_size),
        });

        // Add keys
        columns = [];

        data.forEach(el => {
            Object.keys(el).forEach(key => {
                if(!columns.includes(key)){
                    columns = [...columns, key];
                }
            });
        });
    }

    async function setSelectedColumns() {
        // Set selected Columns only if current selection is not empty
        let selectedColumnsResult: Map<string, boolean> = await invoke("get_columns", {
            configPath: config_path,
            type: selectedType,
        });

        // Set new selected columns
        // Retain previous selection
        for(const [column, isSelected] of Object.entries(selectedColumnsResult)) {
            if(isSelected && !selectedColumns.includes(column)) {
                selectedColumns = [...selectedColumns, column];
            }
        }
    }

    async function previous(){

        if (page_number <= 1) {
            alert("Already at beginning");
            return;
        }

        page_number -= 1;

        await loadData();

        // Only add selection if current selection is not empty
        if(selectedColumns.length > 0) {
            await setSelectedColumns();
        }
    }

    async function next(){
        page_number = Number(page_number) + 1;

        await loadData();

        if(data.length == 0) {
            page_number -= 1;
            await loadData();

            alert("No more data");
        }

        // Only add selection if current selection is not empty
        if(selectedColumns.length > 0) {
            await setSelectedColumns();
        }
    }

    async function displayFullMessage(e: any) {
        openModal = true;

        modalData = data[e.detail.id];
    }
</script>

<div class="flex flex-col h-full mx-2">
    <FileOpeners
        bind:selectedType
        bind:page_size
        bind:page_number
        bind:config_path
        bind:file_path
        on:submit={openFile}
        on:next={next}
        on:previous={previous}
    />

    <DataTableOptions
        bind:selectedColumns
        columns = {columnsProps}
    />

    <DataTable
        {selectedColumns}
        {showSelected}
        {data}
        {columns}
        on:rowClick={displayFullMessage}
    />
</div>

<Modal bind:open={openModal} autoclose>
    <Table>
        <TableBody>
            {#each Object.entries(modalData) as [key, value]}
                <TableBodyRow>
                    <TableBodyCell>{key}</TableBodyCell>
                    <TableBodyCell>{value}</TableBodyCell>
                </TableBodyRow>
            {/each}
        </TableBody>
    </Table>
</Modal>