<script lang='ts'>
    import type { types } from '$lib/utils';

    export let type: types.Input['type'] = 'text';
    export let placeholder: types.Input['placeholder'] = '';
    export let className: types.Input['className'] = '';
    export let name: types.Input['name'] = '';
</script>

<input name={name} class={`text-center h-8 text-xl ${className}`} type={type} placeholder={placeholder}>

<style>
    input{
        color: black;
    }
</style>