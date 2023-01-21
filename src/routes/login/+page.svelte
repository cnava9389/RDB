<script lang='ts'>
    import { goto } from '$app/navigation';
    import { Input, Button, Icon, Container } from '$lib/components';
    import type { types } from "$lib/utils"
    import { auth, store} from "$lib/utils";
    const { Account } = store
    // ! get correct type
    const submitForm: types.FormEventHandler = (e) => {
        const form = new FormData(e.currentTarget);
        auth.loginUser(form.get('email') as string, form.get('password') as string)
        .then(([failed, data]) => {
            if (failed) {
                // !todo log error
                console.error('login failed');
            } else {
                Account.loadUser(data)
                goto('/')
            }
        });
    }
</script>
<Container className="... gap-4">
    <h1 class="text-2xl md:text-4xl">Sign in or create account</h1>
    <form on:submit|preventDefault={submitForm} class="flex flex-col justify-center items-center gap-3 w-3/4">
        <Input name="email" type="text" placeholder="Email" required />
        <Input name="password" type="password" placeholder="Password" required />
        <div class="w-full flex gap-2">
            <Button className="flex justify-center items-center gap-1 w-full" color="blue" type="submit">Login <Icon icon="login"/></Button>
            <Button onClick={() => goto('/create-account')} className="flex justify-center items-center gap-1 w-full" color="green" type="button">Create <Icon icon="add" /></Button>
        </div>
        <a href="/recover" class="flex justify-center items-center gap-1 cursor-pointer hover:underline hover:text-sky-400">Forgot password <Icon icon="question" /></a>
    </form>
</Container>
