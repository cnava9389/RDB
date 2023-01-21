<script lang="ts">
    import Icon from "./Icon.svelte";
    import { debounce, type types } from "$lib/utils";
    import { store, actions } from "$lib/utils";
    
    const { UI } = store;
    const exit = debounce(() => {
        UI.toggleSideNav()
    });

    let init = true

    const sideAnimation = {
        'true': "animate-enterLeft",
        'false': "animate-leaveLeft",
    }

    setTimeout(() => {
        init = false
    }, 1000)

    const links: Record<string, string> = {
        "home": "/",
        "connections": "/connections",
        "settings": "/settings",
        "login": "/login",
        "logout": "/logout"
    }

    const keys = Object.keys(links) as Array<types.Icons>
    
    const handleClick = actions.handleOutsideClick(() => $UI.isSideNavOpen ? exit() : null)
</script>
<span class={init ? "opacity-0" : ""} use:handleClick>

    <div
    class={`absolute border-y-2 border-r-2 w-5/6 md:w-1/4 h-full rounded-tr-lg rounded-br-lg bg-slate-800 flex flex-col items-center py-2 z-10 ${sideAnimation[`${$UI.isSideNavOpen}`]}`}
    >
    <div class="text-2xl border-b-2 w-11/12 text-center pb-2 relative point">
        MENU
        <Icon icon="close" className={"absolute top-0 right-0 transition w-8 h-8"} onClick={exit}/>
    </div>
    <ul class="h-full w-full flex flex-col items-center justify-center">
        <!-- <For each={links}>
            {(link) =>
                sideBarLink({ to: link, text: link.toUpperCase(), icon: link })
            }
        </For> -->
        {#each keys as link}
             <li class="w-full py-2">
                <a on:click={exit} href={links[link]} class="flex text-2xl w-full px-10 justify-between cursor-pointer">
                    {link}
                    <Icon icon={link}/>
                </a>
             </li>
        {/each}
    </ul>
</div>
</span>