<script lang="ts">
    type Filedata = {
        name: String,
        path: string,
        filetype: string
    }

    export let folderdata: Filedata;
    export let onClick;

    let filetype = folderdata.filetype
    let path = folderdata.path
    let name = folderdata.name

    let imgSrc= `/images/${filetype}.png`
    const handleClick = () => {
        if (filetype == "File"){
            return
        }
        onClick(path)
    }
</script>

<div on:click={handleClick}>
    <img src={imgSrc} alt="Italian Trulli">
    <div>
        <h2>{name}</h2>
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

    }
    div {
        display: flex;
        align-items: center;
    }
    h2 {
        display: inline;
    }
</style>