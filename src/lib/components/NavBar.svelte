<script lang="ts">
    import type { SearchData } from '$lib/dirFunctions';
    import { changeToParentDirectory, moveForwardDir, updateCurrentDir } from '$lib/dirFunctions';
    import directoryStore from '$lib/stores/DirectoryStore';
    import { onDestroy } from 'svelte';
    import SearchBar from './SearchBar.svelte';

    let forward: string[] = [];
    let isAtRoot = false;

    let unSubscribe = directoryStore.subscribe((data) => {
        forward = data.forward;
        isAtRoot = data.isAtRoot;
    });

    onDestroy(() => {
        unSubscribe();
    });

    const handleSearch = (data: SearchData) => {};
</script>

<nav>
    <div class="buttons">
        <button disabled={isAtRoot} on:click={changeToParentDirectory}>&#10092;</button>
        <button disabled={forward.length === 0} on:click={moveForwardDir}>&#10093;</button>
        <button on:click={updateCurrentDir}>&#10227;</button>
    </div>
    <div class="searchbar">
        <SearchBar searchHandler={handleSearch} />
    </div>
</nav>

<style>
    nav {
        padding: 1rem;
        background-color: #2c2c2c;
        border: black solid 1px;
        display: grid;
    }

    button {
        background-color: #343434;
        color: #eaeaea;
        border: black solid 1px;
        border-radius: 5px;
        width: 2rem;
        height: 2rem;
        text-align: center;
    }

    .searchbar {
        grid-column: 5 / span 3;
    }

    .buttons {
        grid-column: 1 / span 1;
    }
</style>
