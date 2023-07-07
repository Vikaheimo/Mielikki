<script lang="ts">
    import type { MenuItem } from './Types';

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
    };
</script>

{#if displayMenu}
    <nav style="position: absolute; top:{pos.y}px; left:{pos.x}px">
        <ul>
            {#each menuItems as item}
                {#if item.text === 'hr'}
                    <hr />
                {:else}
                    <li>
                        <button on:click={item.onClick}>
                            <i class={item.icon} />
                            {item.text}
                        </button>
                    </li>
                {/if}
            {/each}
        </ul>
    </nav>
{/if}

<svelte:window on:contextmenu|preventDefault={openMenu} on:click={closeMenu} />

<style>
    hr {
        border: none;
        border-bottom: 1px solid black;
        margin: 5px 0px;
    }

    nav {
        display: inline-flex;
        border: 1px black solid;
        width: 170px;
        background-color: #2c2c2c;
        border-radius: 10px;
        overflow: hidden;
        flex-direction: column;
    }

    ul {
        margin: 6px;
    }

    li {
        display: block;
        list-style-type: none;
        width: 1fr;
    }

    button {
        color: #eaeaea;
        font-size: 1rem;
        width: 100%;
        height: 30px;
        text-align: left;
        border: 0px;
        background-color: #2c2c2c;
    }

    button:hover {
        text-align: left;
        border-radius: 5px;
        background-color: #343434;
    }

    i {
        padding: 0px 15px 0px 10px;
    }
</style>
