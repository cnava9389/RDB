export const ssr = false;
export const prerender = true;
export const csr = true;

import type { LayoutLoad } from "./$types";
import { auth, store } from "$lib/utils";
import { goto } from "$app/navigation";

export const load = (async (ctx) => {
  // get the user from the session
  const [error, data] = await auth.getAccountFromCtx();
  // if there's no user and page isnt allowed, redirect to /login
  if (error && !auth.checkIfPathIsAllowed(ctx)) return await goto("/login");
  if (!error) store.Account.loadUser(data);
}) satisfies LayoutLoad;
