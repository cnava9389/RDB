<script lang="ts">
  import { onMount } from "svelte";
  import "../app.css";
  import type { LayoutData } from "./$types";

  export let data: LayoutData;
</script>

<main data-testid="layout" class="flex flex-col h-screen w-screen overflow-y-auto overflow-x-hidden relative">
  <slot />
</main>
