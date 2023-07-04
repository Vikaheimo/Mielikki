<script lang="ts">
    import type { SearchData } from '$lib/dirFunctions';

    export let searchHandler: (data: SearchData) => void;

    const handleSubmit = (event: any) => {
        const formData = new FormData(event.target);
        const data: SearchData = {
            files: formData.get('file') === 'on',
            folders: formData.get('folder') === 'on',
            links: formData.get('link') === 'on',
            filename: formData.get('search') as string
        };
        searchHandler(data);
    };
</script>

<form on:submit|preventDefault={handleSubmit}>
    <input type="text" id="search" placeholder="Search" name="search" />
    <div class="options">
        <div>
            <input type="checkbox" id="file" checked={true} name="file" />
            <label for="file">Files</label>
        </div>

        <div>
            <input type="checkbox" id="folder" checked={true} name="folder" />
            <label for="folder">Folders</label>
        </div>

        <div>
            <input type="checkbox" id="link" checked={true} name="link" />
            <label for="link">Links</label>
        </div>
    </div>
</form>

<style>
    form {
        display: flex;
        align-items: center;
        column-gap: 2rem;
    }
    input[type='text'] {
        background-color: #343434;
        border: 1px solid black;
        border-radius: 2px;
        height: 2rem;
        min-width: 400px;
        color: #eaeaea;
        padding-left: 1rem;
    }

    .options {
        display: flex;
        align-items: center;
        justify-content: center;
        column-gap: 1rem;
    }
</style>
