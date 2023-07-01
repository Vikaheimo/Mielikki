<script lang="ts">
    import { invoke} from "@tauri-apps/api/tauri"
    import SubDir from "./SubDir.svelte";
  
    let folder_name = "";
    let contents = [];

    type Filedata = {
        name: string,
        path: string,
        type: string
    }

    type FolderData = {
        name: string,
        files: Array<Filedata>
    }
    const getFolderName = () => { 
        invoke("get_current_folder", {}).then((data: FolderData) => {
            console.log(data)
            folder_name = data.name
            contents = data.files
        })
    }

    const changeDirectory = (path: string) => {
        console.log(path)
    }

    getFolderName()
</script>

<div>
    <h1>{folder_name}</h1>
    <ul>
        {#each contents as content}
            <li><SubDir folderdata={content} onClick={changeDirectory}/></li>
        {/each}
    </ul>
</div>

<style>
    li {
        margin-bottom: 0.5rem;
        border-bottom: 2px solid gray;
    }
</style>