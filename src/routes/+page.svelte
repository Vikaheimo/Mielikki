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

    let rightClickMenu: SvelteComponent;
    let openMenu: (event: MouseEvent, data: MenuItem[]) => void;

    onMount(() => {
        openMenu = rightClickMenu.openMenu
    })

    onDestroy(() => {
        unSubscribe();
    });

    const handleFileClick = (data: Filedata) => {
        if (data.filetype === 'Folder') {
            changeDirectory(data.path);
        } else if (data.filetype === 'Link') {
            changeDirectory(data.path, true)
        } else {
            // TODO, do something on file click?
        }
    };

    let fileItems: MenuItem[] = [
        {"icon": "A", "text": "Kissa", "onClick": () => console.log("kissa")},
        MenuItemHr,
        {"icon": "A", "text": "Kissa", "onClick": () => console.log("kissa")}
    ]

    let otherItems: MenuItem[] = [
        {"icon": "A", "text": "Koira", "onClick": () => console.log("kissa")},
        MenuItemHr,
        {"icon": "A", "text": "Koira", "onClick": () => console.log("kissa")}
    ]

    updateCurrentDir();
</script>

<main>
    <RightClickMenu bind:this={rightClickMenu}/>
    <h1>Directory listing of <strong>{dirName}</strong></h1>
    <ul>
        {#each contents as file}
            <li on:contextmenu|preventDefault={(e) => openMenu(e, fileItems)}>
                <FileDisplay filedata={file} onClick={handleFileClick} />
            </li>
        {/each}
    </ul>
</main>

<style>
    li {
        margin-bottom: 0.5rem;
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
</style>
