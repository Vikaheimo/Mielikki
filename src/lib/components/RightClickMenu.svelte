<script lang="ts">
    import { text } from "@sveltejs/kit";
    import type { MenuItem } from "./Types";

    let pos = {
        x: 0,
        y: 0
    };

    let menuConfig = {
        height: 0,
        width: 0
    };

    let displayMenu = false;
    export let menuItems: MenuItem[] = [];

    const closeMenu = () => {
        displayMenu = false;
    };

    const openMenu = () => {
        displayMenu = true;
    }

</script>

{#if displayMenu}
<nav style="position: absolute; top:{pos.y}px; left:{pos.x}px">
    <div>
        <ul>
            {#each menuItems as item}
                {#if item.text === "hr"}
                    <hr>
                {:else}
                    <li>
                        <button on:click={item.onClick}>
                            <i class={item.icon}></i>
                            {item.text}
                        </button>
                    </li>
                {/if}
            {/each}
        </ul>
    </div>
</nav>
{/if}

<svelte:window on:contextmenu|preventDefault={openMenu} on:click={closeMenu}/>

<style>

    hr{
        border: none;
        border-bottom: 1px solid #ccc;
        margin: 5px 0px;
    }
</style>