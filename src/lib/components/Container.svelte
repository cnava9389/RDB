<script lang="ts">
    import { autoAnimate } from "$lib/utils";
    export let className = '';
    export let animate = false;
    const defaultClass = "flex flex-col justify-center items-center h-full w-full"
    const fn = animate ? autoAnimate : () => {};
</script>

<div class={className && className.includes("...") ? `${defaultClass} ${className}` : (className ? className : defaultClass)} use:fn>
    <slot />
</div>