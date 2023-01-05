<script lang='ts'>
    import { types } from '$lib/utils';

    export let type: types.Button['type'] = 'button'
    export let className: types.Button['className'] = ''
    export let color: types.Colors = ""
    export let onClick: types.Button['onClick'] = () => {}
</script>
<button on:click={onClick} class={`${types.colors[color]} px-4 rounded-lg text-xl ${className}`} type={type}>
    <slot/>
</button>