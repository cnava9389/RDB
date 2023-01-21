<script lang="ts">
    import { debounce, type types } from "$lib/utils";
    import Icon from "./Icon.svelte";
    export let onClick:types.Button['onClick'] = () => {};
</script>


<Icon icon="menu" className={"absolute top-2 right-2 transition z-10 w-8 h-8"} onClick={debounce(onClick)}/>