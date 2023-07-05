<script lang="ts">
    export let text: string;
    export let disabled = false;
    export let onClick: () => void;
</script>

<button disabled={disabled} on:click={onClick}>{text}</button>

<style>
    button {
        background-color: #343434;
        color: #eaeaea;
        border: black solid 1px;
        border-radius: 5px;
        width: 2rem;
        height: 2rem;
        text-align: center;
    }
</style>