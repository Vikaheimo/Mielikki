<script lang="ts">
    import { SvelteComponent, onDestroy, onMount } from 'svelte';
    import FileDisplay from '$lib/components/FileDisplay.svelte';
    import type { Filedata } from '$lib/DirFunctions';
    import { changeDirectory, updateCurrentDir } from '$lib/DirFunctions';
    import DirectoryStore from '$lib/stores/DirectoryStore';
    import RightClickMenu from '$lib/components/RightClickMenu.svelte';
    import type { MenuItem } from '$lib/components/Types';
    import { MenuItemHr } from '$lib/components/Types';

    let contents: Filedata[] = [];
    let dirName = '';

    let unSubscribe = DirectoryStore.subscribe((data) => {
        contents = data.siblings;
        dirName = data.dirName;
    });

    let fileItems: MenuItem[] = [
        { icon: 'fa-solid fa-computer-mouse', text: 'Open', onClick: () => console.log('Open') },
        MenuItemHr,
        { icon: 'fa-solid fa-trash-can', text: 'Delete', onClick: () => console.log('delete') }
    ];
    let folderItems: MenuItem[] = [
        {
            icon: 'fa-sharp fa-solid fa-folder-plus',
            text: 'New Folder',
            onClick: () => console.log('Add folder')
        },
        {
            icon: 'fa-solid fa-file-circle-plus',
            text: 'New File',
            onClick: () => console.log('Add file')
        }
    ];
    let rightClickMenu: SvelteComponent;
    let openMenu: (event: MouseEvent) => void;
    let menuData = folderItems;
    onMount(() => {
        openMenu = (event: MouseEvent) => {
            rightClickMenu.openMenu(event, menuData);
        };
    });

    onDestroy(() => {
        unSubscribe();
    });

    const handleFileClick = (data: Filedata) => {
        if (data.filetype === 'Folder') {
            changeDirectory(data.path);
        } else if (data.filetype === 'Link') {
            changeDirectory(data.path, true);
        } else {
            // TODO, do something on file click?
        }
    };

    updateCurrentDir();
</script>

<main on:contextmenu|preventDefault={(e) => openMenu(e)}>
    <RightClickMenu bind:this={rightClickMenu} />
    <h1>Directory listing of <strong>{dirName}</strong></h1>
    <ul>
        {#each contents as file}
            <li
                on:mouseenter={() => {
                    menuData = fileItems;
                }}
                on:mouseleave={() => {
                    menuData = folderItems;
                }}
            >
                <FileDisplay filedata={file} onClick={handleFileClick} />
            </li>
            <hr />
        {/each}
    </ul>
</main>

<style>
    hr {
        border: none;
        margin: 5px 0px;
        border-bottom: 2px solid gray;
    }

    h1 {
        font-size: 2rem;
        text-align: center;
        font-weight: 100;
    }

    strong {
        font-weight: 900;
    }

    li {
        display: inline;
    }
</style>
