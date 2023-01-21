<script lang="ts">
    import { Container, Icon, Button } from "$lib/components";
    import { store, listify, actions } from "$lib/utils";
    import { goto } from "$app/navigation";

    const { Account } = store;
    
    let connections = listify($Account.dbs, (key, value) => ({id:key, name: value}))
</script>

<Container>
    {#if connections.length > 0}
        <Container className="flex flex-col px-2 md:px-0 md:place-items-center md:flex-none md:grid md:grid-cols-3 gap-4 h-5/6 w-full overflow-y-scroll hideScroll" animate>
            {#each connections as item}
                <div class="bg-slate-600 p-6 rounded-lg shadow-lg md:h-[15vh] md:w-[25vw] flex flex-col justify-center items-center" use:actions.handleBounce>
                    <div>name: {item.name}</div>
                    <div>id: {item.id}</div>
                    <div>connected: false</div>
                </div>
            {/each}
        </Container>
        <Button onClick={() => goto('/connections/create')} color="blue" className="md:w-1/2 mt-8 flex justify-center items-center gap-1 ">Add Connection <Icon icon="add"/></Button>
    {:else}
     <a class="flex flex-col justify-center items-center text-2xl md:text-4xl" href="/connections/create">
        <Icon className="w-[25vw]" icon="add"/>
        <span>No connections add a new one</span>
    </a>
    {/if}
</Container>