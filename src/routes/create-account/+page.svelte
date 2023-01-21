<script lang="ts">
    import { Container, Input, Button, Icon } from "$lib/components";
    import { goto } from "$app/navigation";
    import { auth, types } from "$lib/utils";

     const submitForm: types.FormEventHandler = (e) => {
        const form = new FormData(e.currentTarget);
        const pass = form.get('password') as string;
        const confirm = form.get('confirm') as string;
        if (pass !== confirm) {
            // !todo log error
            console.error('passwords do not match');
            return;
        }
        auth.registerUser(form.get('email') as string, pass)
        .then(([failed, _res]) => {
            if (failed) {
                // !todo log error
                console.error('create account failed');
            } else {
                goto('/login')
            }
        });
    }
</script>

<Container className="... gap-4">
    <h1 class="text-2xl md:text-4xl">Create a new account</h1>
    <form on:submit|preventDefault={submitForm} class="flex flex-col justify-center items-center gap-3 w-3/4">
        <Input name="email" type="text" placeholder="Email" required/>
        <Input name="password" type="password" placeholder="Password" required/>
        <Input name="confirm" type="password" placeholder="Confirm Password" required/>
        <Button className="flex justify-center items-center gap-1 w-full" color="green" type="submit">Create <Icon icon="add"/></Button>
    </form>
</Container>