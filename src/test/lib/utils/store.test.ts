import { store } from "$lib/utils";
describe("Testing store functions", () => {
  const Account = {
    user: { id: "1", email: "", name: "" },
    dbs: {
      "c2a0ec9c-1d0e-45df-a28d-8942eec9e09c": "admin",
    },
  };

  beforeEach(() => {
    store.Account.unLoadUser();
  });

  it("loggedIn function works", () => {
    store.Account.loadUser(Account);
    expect(store.Account.loggedIn()).toBe(true);
  });

  it("logged out function works", () => {
    store.Account.loadUser(Account);
    expect(store.Account.loggedIn()).toBe(true);
    store.Account.unLoadUser();
    expect(store.Account.loggedIn()).toBe(false);
  });
});
