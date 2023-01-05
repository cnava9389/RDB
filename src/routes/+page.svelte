<script lang='ts'>
import {store} from '$lib/utils'
const {Account} = store
</script>

<div class="flex flex-col justify-center items-center h-full">
  <p>Hello, world!, {$Account.user.name}</p>
  <a href="/login"> login</a>
</div>