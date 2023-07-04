<script lang="ts">
    import { changeToParentDirectory, moveForwardDir, updateCurrentDir } from '$lib/dirFunctions';
    import directoryStore from '$lib/stores/DirectoryStore';

    import NavBar from '$lib/NavBar.svelte';
    import { onDestroy } from 'svelte';

    let forward: string[] = [];
    let isAtRoot = false;
    let unSubscribe = directoryStore.subscribe((data) => {
        forward = data.forward;
        isAtRoot = data.isAtRoot;
    });

    onDestroy(() => {
        unSubscribe();
    });
</script>

<NavBar
    isBackDisabled={isAtRoot}
    backButtonOnClick={changeToParentDirectory}
    isforwardDisabled={forward.length === 0}
    forwardButtonOnClick={moveForwardDir}
    refreshOnClick={updateCurrentDir}
/>

<main>
    <slot />
</main>
