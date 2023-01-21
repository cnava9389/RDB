<script lang='ts'>
    import type { types } from '$lib/utils';

    export let type: types.Input['type'] = 'text';
    export let placeholder: types.Input['placeholder'] = '';
    export let className: types.Input['className'] = '';
    export let name: types.Input['name'] = '';
    export let required: types.Input['required'] = false;
</script>

<input name={name} class={`text-center w-full h-8 text-xl md:h-10 md:text-4xl ${className}`} type={type} placeholder={placeholder} required={required}>

<style>
    input{
        color: black;
    }
</style>