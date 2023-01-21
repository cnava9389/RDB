import { writable } from "svelte/store";
import { range, type types } from ".";

let accountStore: types.AccountStore = {
  user: { id: "", email: "", name: "" },
  dbs: {},
  loggedIn: false,
};
const AccountWritable = writable<types.AccountStore>({ ...accountStore });
const unSubtoAccount = AccountWritable.subscribe((value) => {
  accountStore = value;
});

export const Account = {
  subscribe: AccountWritable.subscribe,
  loggedIn: () => !!accountStore.user.id,
  loadUser: (data: Pick<types.AccountStore, "dbs" | "user">) => {
    AccountWritable.update((value) => {
      return { ...value, user: data.user, dbs: data.dbs, loggedIn: true };
    });
  },
  unLoadUser: () => {
    AccountWritable.update((value) => {
      return {
        ...value,
        user: { id: "", email: "", name: "" },
        dbs: {},
        loggedIn: false,
      };
    });
  },
  loadDbs: (dbs: Array<readonly [string, string]>) => {
    AccountWritable.update((value) => {
      const { dbs: map } = value;
      for (const i of range(dbs.length)) {
        map[dbs[i][0]] = dbs[i][1];
      }
      return { ...value, dbs: map };
    });
  },
};

let uiStore: types.UIState = {
  isSideNavOpen: false,
};

const UIWritable = writable<types.UIState>({ ...uiStore });
const unSubtoUI = UIWritable.subscribe((value) => {
  uiStore = value;
});
export const UI = {
  subscribe: UIWritable.subscribe,
  toggleSideNav: () => {
    UIWritable.update((value) => {
      return { ...value, isSideNavOpen: !value.isSideNavOpen };
    });
  },
};
