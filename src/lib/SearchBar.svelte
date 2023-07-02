<form>
    <input type="text" id="search" placeholder="Search" />
    <div class="options">
        <div>
            <input type="checkbox" id="file" checked={true} />
            <label for="file">Files</label>
        </div>

        <div>
            <input type="checkbox" id="folder" checked={true} />
            <label for="folder">Folders</label>
        </div>

        <div>
            <input type="checkbox" id="link" checked={true} />
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
