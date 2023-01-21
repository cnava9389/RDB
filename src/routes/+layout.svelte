<script lang="ts">
  import "../app.css";
  import type { LayoutData } from "./$types";
  import { SideNav, MenuToggle } from "../lib/components";
  import { store, auth, types, UUIDGeneratorBrowser } from "$lib/utils";
  import { beforeNavigate } from "$app/navigation";

  const { UI, Account } = store

  beforeNavigate((nav) => {
    if (nav.to?.route.id === nav.from?.route.id || !$Account.loggedIn && !auth.checkIfPathIsAllowed({ url: nav.to?.url } as types.CTX)) nav.cancel()
  })

</script>

<SideNav />
<MenuToggle onClick={UI.toggleSideNav}/>
<main data-testid="layout" class="flex flex-col h-screen w-screen overflow-y-auto overflow-x-hidden relative">
  <slot />
</main>
