<script lang="ts">
    import { onDestroy } from 'svelte';
    import FileDisplay from '$lib/components/FileDisplay.svelte';
    import type { Filedata } from '$lib/dirFunctions';
    import { updateCurrentDir } from '$lib/dirFunctions';
    import DirectoryStore from '$lib/stores/DirectoryStore';

    let contents: Filedata[] = [];
    let dirName = '';

    let unSubscribe = DirectoryStore.subscribe((data) => {
        contents = data.siblings;
        dirName = data.dirName;
    });

    onDestroy(() => {
        unSubscribe();
    });

    updateCurrentDir();
</script>

<main>
    <h1>Direcory listing of <strong>{dirName}</strong></h1>
    <ul>
        {#each contents as content}
            <li><FileDisplay folderdata={content} /></li>
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
