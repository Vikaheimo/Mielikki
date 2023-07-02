<script lang="ts">
    import { onDestroy } from 'svelte';
    import SubDir from './SubDir.svelte';
    import type { Filedata } from './dirFunctions';
    import { updateCurrentDir, changeDirectory, changeToParentDirectory } from './dirFunctions';
    import DirectoryStore from './stores/DirectoryStore';

    let contents: Array<Filedata> = [];
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

<div>
    <h1>Direcory listing of <strong>{dirName}</strong></h1>
    <ul>
        {#each contents as content}
            <li><SubDir folderdata={content} /></li>
        {/each}
    </ul>
</div>

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
