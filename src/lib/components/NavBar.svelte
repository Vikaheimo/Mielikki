<script lang="ts">
    import type { SearchData } from '$lib/dirFunctions';
    import NavigationButton from './NavigationButton.svelte';
    import {
        changeToParentDirectory,
        moveForwardDir,
        updateCurrentDir,
        searchFiles
    } from '$lib/dirFunctions';
    import directoryStore from '$lib/stores/DirectoryStore';
    import searchStore from '$lib/stores/SearchStore';
    import { onDestroy } from 'svelte';
    import SearchBar from './SearchBar.svelte';
    import { goto } from '$app/navigation';
    import { text } from '@sveltejs/kit';

    let forward: string[] = [];
    let isAtRoot = false;

    const unSubscribe = directoryStore.subscribe((data) => {
        forward = data.forward;
        isAtRoot = data.isAtRoot;
    });

    onDestroy(() => {
        unSubscribe();
    });

    const handleSearch = (data: SearchData) => {
        searchStore.set({
            search: data,
            data: []
        });
        searchFiles(data);
        goto('search');
    };
</script>

<nav>
    <div class="buttons">
        <NavigationButton disabled={isAtRoot} onClick={changeToParentDirectory} text= {"❬"} />
        <NavigationButton disabled={forward.length === 0} onClick={moveForwardDir} text={"❭"} />
        <NavigationButton onClick={updateCurrentDir} text={"⟳"}/>
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

    .searchbar {
        grid-column: 5 / span 3;
    }

    .buttons {
        grid-column: 1 / span 1;
    }
</style>
