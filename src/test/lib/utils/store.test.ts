import {store} from "$lib/utils"
describe("Testing store functions", () => {
    const user = {id: "1", email: "", name: ""}

    beforeEach(() => {
        store.Account.unLoadUser()
    })

    it("loggedIn function works", () => {
        store.Account.loadUser(user)
        expect(store.Account.loggedIn()).toBe(true);
    })

    it("logged out function works", () => {
        store.Account.loadUser(user)
        expect( store.Account.loggedIn()).toBe(true);
        store.Account.unLoadUser();
        expect(store.Account.loggedIn()).toBe(false);

    })
})