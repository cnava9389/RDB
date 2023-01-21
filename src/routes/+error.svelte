<script lang="ts">
    import { page } from '$app/stores'
    import { Container } from '$lib/components'
</script>
<Container>
    Error: {$page.error?.message}
</Container>