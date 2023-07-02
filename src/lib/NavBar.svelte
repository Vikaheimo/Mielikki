<script lang="ts">
    import SearchBar from './SearchBar.svelte';
    export let backButtonOnClick: () => void;
    export let isBackDisabled: boolean = false;
    export let forwardButtonOnClick: () => void;
    export let isforwardDisabled: boolean = true;
</script>

<nav>
    <button disabled={isBackDisabled} on:click={backButtonOnClick}>&#10092;</button>
    <button disabled={isforwardDisabled} on:click={forwardButtonOnClick}>&#10093;</button>
    <div><SearchBar /></div>
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
        grid-column: 1 / span 1;
        width: 2rem;
        height: 2rem;
        text-align: center;
    }

    div {
        grid-column: 5 / span 3;
    }
</style>
