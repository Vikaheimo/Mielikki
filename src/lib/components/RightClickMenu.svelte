<script lang="ts">
    import type { MenuItem } from './Types';

    let position = {
        x: 0,
        y: 0
    };
    let menuSize = {
        height: 0,
        width: 0
    };
    let windowSize = {
        width: 0,
        height: 0
    };

    export let displayMenu = false;
    let menuItems: MenuItem[] = [];

    const calculateMenuSize = (element: HTMLElement) => {
        menuSize = {
            width: element.offsetWidth,
            height: element.offsetHeight
        };
    };

    export const closeMenu = () => {
        displayMenu = false;
    };

    export const openMenu = (event: MouseEvent, data: MenuItem[]) => {
        menuItems = data;
        displayMenu = true;

        windowSize = {
            width: window.innerWidth,
            height: window.innerHeight
        };
        position = {
            x: event.clientX,
            y: event.clientY
        };
        // Handle page overflow, eg if menu elements would be larger than the available space
        if (windowSize.height - position.y < menuSize.height) {
            position.y = position.y - menuSize.height;
        }

        if (windowSize.width - position.x < menuSize.height) {
            position.x = position.x - menuSize.width;
        }
    };
</script>

{#if displayMenu}
    <nav
        use:calculateMenuSize
        style="position: absolute; top:{position.y}px; left:{position.x}px"
        on:contextmenu|preventDefault
    >
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

<svelte:window on:click={closeMenu} />
<svelte:head>
    <script src="https://kit.fontawesome.com/87f235ba5f.js" crossorigin="anonymous"></script>
</svelte:head>

<style>
    hr {
        border: none;
        border-bottom: 1px solid black;
        margin: 5px 0px;
    }

    nav {
        display: inline-flex;
        border: 1px black solid;
        max-width: 500px;
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
        min-height: 30px;
        text-align: left;
        border: 0px;
        background-color: #2c2c2c;
        word-break: break-all;
        display: inline-flex;
        align-items: center;
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
