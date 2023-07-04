<script lang="ts">
    import { goto } from '$app/navigation';
    import FileDisplay from '$lib/components/FileDisplay.svelte';
    import { changeDirectory, type Filedata } from '$lib/dirFunctions';
    import SearchStore from '$lib/stores/SearchStore';
    import { onDestroy } from 'svelte';

    let search = '';
    let files: Filedata[] = [];

    const unSubscribe = SearchStore.subscribe((searchData) => {
        search = searchData.search.name;
        files = searchData.data;
    });

    onDestroy(() => {
        unSubscribe();
    });

    const onFileClick = (data: Filedata) => {
        changeDirectory(data.path, true);
        goto('/');
    };
</script>

<main>
    <h1>Search results for <strong>{search}</strong></h1>

    <ul>
        {#each files as content}
            <li>
                <FileDisplay filedata={content} onClick={onFileClick} />
            </li>
        {/each}
    </ul>
</main>

<style>
    li {
        margin-bottom: 0.5rem;
        border-bottom: 2px solid gray;
    }

    h1 {
        font-size: 2rem;
        text-align: center;
        font-weight: 100;
    }

    strong {
        font-weight: 900;
    }
</style>
