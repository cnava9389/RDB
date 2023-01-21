<script lang="ts">
    import { store } from "$lib/utils";
    import { Input, Button, Icon } from "$lib/components";
    const { Account } = store;
</script>

<div class="text-2xl md:text-4xl">Account</div>
<form class="h-3/4 flex flex-col items-center text-2xl md:text-4xl gap-3 w-5/6">
    <label for="email">Email</label>
    <Input name="email" type="email" placeholder={$Account.user.email} />
    <label for="name">Name</label>
    <Input name="name" type="text" placeholder={$Account.user.name} />
    <div class="w-full flex flex-col gap-2 py-2 mt-auto">
        <Button className="flex justify-center items-center gap-1 w-full text-xl" color="red" type="submit">Password Reset <Icon icon="password" className="h-6 w-6 md:h-8 md:w-8"/></Button>
        <Button onClick={() => {}} className="flex justify-center items-center gap-1 w-full" color="green" type="button">Update <Icon icon="upload" /></Button>
    </div>
</form>