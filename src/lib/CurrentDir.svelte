<script lang="ts">
    import { onDestroy } from 'svelte';
    import SubDir from './SubDir.svelte';
    import type { Filedata } from './dirFunctions';
    import { updateCurrentDir, changeDirectory, changeToParentDirectory } from './dirFunctions';
    import DirectoryStore from './stores/DirectoryStore';

    let contents: Array<Filedata> = [];
    let unSubscribe = DirectoryStore.subscribe((data) => {
        contents = data.siblings;
    });

    onDestroy(() => {
        unSubscribe();
    });

    updateCurrentDir();
</script>

<div>
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
</style>
