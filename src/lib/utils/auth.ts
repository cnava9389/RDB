import type { CTX, USER } from "./types";
import { Credentials } from "./types";
import { z } from "zod";
import { type types, tryCatch, proxied } from ".";

const DEV_ACCOUNT: () => types.AccountStore = () => ({
  user: {
    name: "",
    email: "cnava9389@gmail.com",
  },
  dbs: {
    "c2a0ec9c-1d0e-45df-a28d-8942eec9e09c": "admin",
  },
  loggedIn: true,
});

const SERVER = `http://${import.meta.env.VITE_SERVER_HOST}:${
  import.meta.env.VITE_SERVER_PORT
}`;

export const useFetch = (
  url: string,
  opts?: {
    method?: RequestInit["method"];
    body?: Record<string | number, unknown>;
  }
) => {
  return fetch(`${SERVER}${url}`, {
    ...opts,
    body: opts && "body" in opts ? JSON.stringify(opts.body) : undefined,
    headers: {
      Accept: "application/json, text/plain, */*",
      "Content-type": "application/json; charset=UTF-8",
    },
    credentials: "include",
  });
};
// on each request this will check the context and return a user if user has been authenticated
export async function getAccountFromCtx(): Promise<
  readonly [boolean, Pick<types.AccountStore, "user" | "dbs">]
> {
  if (isDev()) return [false, DEV_ACCOUNT()];
  // !todo
  return await tryCatch(async () => {
    const res = await useFetch("/auth/account", { method: "GET" });
    if (!res.ok) throw Error(res.statusText);
    return await res.json();
  });
}

const publicPaths = ["/login", "/create-account", "/logout", "/recover"];
const pathChecker = (path: string) =>
  z.string().startsWith(path, { message: "Path not allowed" });
// check if the path is allowed
export function checkIfPathIsAllowed(ctx: CTX): boolean {
  const { pathname } = ctx.url;
  for (const path of publicPaths) {
    try {
      if (pathChecker(path).parse(pathname)) return true;
    } catch (error) {
      if (error instanceof z.ZodError) continue;
      // maybe log error
      else throw error;
    }
  }
  return false;
}

// login a user returning a tuple with any failure and user
export const loginUser: (
  email: string,
  password: string
) => Promise<
  readonly [boolean, Pick<types.AccountStore, "user" | "dbs">]
> = async (email, password) => {
  if (isDev()) {
    return [false, DEV_ACCOUNT()] as const;
  }

  return await tryCatch(async () => {
    const { id, password: pass } = Credentials.parse({
      id: email,
      password,
    });
    const res = await useFetch("/auth/login", {
      method: "POST",
      body: { id, password: pass },
    });
    if (!res.ok) {
      throw Error(res.statusText);
    }
    return await res.json();
  });
};

export const registerUser: (
  email: string,
  password: string
) => Promise<readonly [boolean, null]> = async (email, password) => {
  if (isDev()) {
    return [false, null] as const;
  }

  return await tryCatch(async () => {
    const { id, password: pass } = Credentials.parse({ id: email, password });
    const res = await useFetch("/auth/create", {
      method: "POST",
      body: { id, password: pass },
    });

    if (!res.ok) {
      throw new Error(res.statusText);
    }
    return null;
  });
};

export const logoutUser: () => Promise<readonly [boolean, null]> = async () => {
  if (isDev()) return [true, null];
  return await tryCatch(async () => {
    const res = await useFetch("/auth/logout", { method: "GET" });
    if (!res.ok) throw Error(res.statusText);
    return null;
  });
};

export const isDev = () => import.meta.env.VITE_DEV === "true";
