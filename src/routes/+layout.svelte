<script lang="ts">
    import { currentDirIsRoot, changeToParentDirectory, moveForwardDir } from '$lib/dirFunctions';
    import directoryStore from '$lib/stores/DirectoryStore';
    import '../global.css';
    import NavBar from '$lib/NavBar.svelte';
    import { onDestroy } from 'svelte';

    let forward: string[] = [];
    let unSubscribe = directoryStore.subscribe((data) => {
        forward = data.forward;
    });

    onDestroy(() => {
        unSubscribe();
    });

    const cannotMoveUp = (): boolean => {
        console.log('forward data:', forward);
        return forward.length === 0;
    };
</script>

<NavBar
    isBackDisabled={currentDirIsRoot()}
    backButtonOnClick={changeToParentDirectory}
    isforwardDisabled={forward.length === 0}
    forwardButtonOnClick={moveForwardDir}
/>

<main>
    <slot />
</main>
