export const ssr = false;
export const prerender = true;
export const csr = true;

import type { LayoutLoad } from "./$types";
import { auth } from "$lib/utils";
import { goto } from "$app/navigation";

export const load = (async (ctx) => {
  // get the user from the session
  const user = auth.getAccount(ctx);
  // if there's no user and page isnt allowed, redirect to /login
  if (!user && !auth.checkIfPathIsAllowed(ctx)) return await goto("/login");
  return {
    user,
  };
}) satisfies LayoutLoad;
