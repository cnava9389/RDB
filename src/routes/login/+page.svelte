<script lang='ts'>
    import { goto } from '$app/navigation';
    import { Input, Button, Icon } from '$lib/components';
    import type { types } from "$lib/utils"
    import { auth, store} from "$lib/utils";
    const { Account } = store
    // ! get correct type
    const submitForm: types.FormEventHandler = (e) => {
        const form = new FormData(e.currentTarget);
        const [failed, user] = auth.loginUser(form.get('email') as string, form.get('password') as string);
        if (failed) {
            // !todo log error
            console.log('login failed');
        } else {
            Account.loadUser(user)
            goto('/')
        }
    }
</script>
<div class="flex flex-col justify-center items-center h-full gap-4">
    <h1 class="text-2xl">Sign In or Create Account</h1>
    <form on:submit|preventDefault={submitForm} class="flex flex-col justify-center items-center gap-3">
        <Input name="email" type="email" placeholder="email" />
        <Input name="password" type="password" placeholder="Password" />
        <Button color="blue" type="submit">Login</Button>
        <Button onClick={() => goto('/create-account')} className="flex justify-center items-center gap-1" color="green" type="button">Create <Icon icon="add" /></Button>
        <a href="/recover" class="flex justify-center items-center gap-1 cursor-pointer hover:underline hover:text-sky-400">Forgot password <Icon icon="question" /></a>
    </form>
</div>
