<script lang="ts">
    import type { Filedata } from '$lib/dirFunctions';

    export let filedata: Filedata;
    export let onClick: (data: Filedata) => void;
    let filetype = filedata.filetype;
    let path = filedata.path;
    let name = filedata.name;

    let imgSrc = `/images/${filetype}.png`;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div on:click={() => onClick(filedata)}>
    <img src={imgSrc} alt="OOPS" />
    <div>
        <h2>{name}</h2>
        <p>{path}</p>
    </div>
</div>

<style>
    img {
        width: 100%;
        max-width: 60px;
        display: inline;
    }
    div div {
        padding-left: 0.5rem;
        display: inline;
        line-height: 1.5;
    }
    div {
        display: flex;
        align-items: center;
    }
    h2 {
        display: inline;
        font-size: larger;
    }
</style>
