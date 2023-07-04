<script lang="ts">
    import SearchBar from './SearchBar.svelte';
    export let backButtonOnClick: () => void;
    export let isBackDisabled: boolean = false;
    export let forwardButtonOnClick: () => void;
    export let isforwardDisabled: boolean = true;
    export let refreshOnClick: () => void;
</script>

<nav>
    <div class="buttons">
        <button disabled={isBackDisabled} on:click={backButtonOnClick}>&#10092;</button>
        <button disabled={isforwardDisabled} on:click={forwardButtonOnClick}>&#10093;</button>
        <button on:click={refreshOnClick}>&#10227;</button>
    </div>
    <div class="searchbar">
        <SearchBar />
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
