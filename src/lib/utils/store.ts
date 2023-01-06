import { writable } from "svelte/store"
import type { types } from "."

type AccountStore = {user:types.USER}
let accountStore: AccountStore = {user: {id: "", email: "", name: ""}}
const AccountWritable = writable<AccountStore>({...accountStore})
const unSubtoAccount = AccountWritable.subscribe((value) => {
    accountStore = value
})
export const Account = {
    subscribe: AccountWritable.subscribe,
    loggedIn: () => !!accountStore.user.id,
    loadUser: (user:types.USER) => {
        AccountWritable.update((value) => {
            return {...value, user}
        })
    },
    unLoadUser: () => {
        AccountWritable.update((value) => {
            return {...value, user: {id: "", email: "", name: ""}}
        })
    }
}