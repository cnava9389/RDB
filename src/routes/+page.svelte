<script lang='ts'>
import {store} from '$lib/utils'
import { Container } from '$lib/components';
const {Account} = store
</script>

<Container>
  <p>Hello, world!, {$Account.user.name}</p>
  <a href="/login"> login</a>
</Container>