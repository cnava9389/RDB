<script lang="ts">
  import { goto } from "$app/navigation";
    import { Container } from "$lib/components";

    const links = ["config", "account"]
    goto(`/settings/${links[0]}`)
</script>

<Container>
    <div class="flex flex-col md:flex-row gap-2 w-full h-full">
        <Container className="flex flex-col justify-center items-center w-full md:w-1/4 h-1/6 md:h-full">
            {#each links as link}
                 <a href={`/settings/${link}`}>{link}</a>
            {/each}
        </Container>
        <div class="md:h-full flex items-center justify-center">
            <hr class="vl md:h-5/6 h-0 w-5/6">
        </div>
        <Container className="... gap-2"><slot/></Container>

    </div>
</Container>