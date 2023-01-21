<script>
    import { goto } from '$app/navigation';
    import { auth, store } from '$lib/utils';
    
    auth.logoutUser().then((res) => {
        store.Account.unLoadUser()
        goto('/login');
    });
</script>