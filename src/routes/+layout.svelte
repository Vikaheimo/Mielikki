<script lang="ts">
    import { currentDirIsRoot, changeToParentDirectory } from '$lib/dirFunctions';
    import '../global.css';
    import NavBar from '$lib/NavBar.svelte';
</script>

<NavBar isBackDisabled={currentDirIsRoot} backButtonOnClick={changeToParentDirectory} />

<main>
    <slot />
</main>
