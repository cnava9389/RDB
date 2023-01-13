import type { CTX, USER } from "./types";
import { Credentials } from "./types";
import { z } from "zod";
import type { types } from ".";

const DEV_USER = { id: "", email: "", name: "test" };

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
export async function getAccountFromCtx(ctx: CTX): Promise<USER | null> {
  if (isDev()) return DEV_USER;
  // !todo
  const res = await useFetch("/auth/account", { method: "GET" });
  if (!res.ok) return null;
  const user = await res.json();
  return user;
}

const publicPaths = ["/login", "/register", "/forgot-password"];
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
) => Promise<readonly [boolean, types.USER]> = async (email, password) => {
  let failure = false;
  let user = { id: "", email: "", name: "" };

  if (isDev()) {
    user = DEV_USER;
  }

  if (email === "" || password === "") return [true, user] as const;

  try {
    const { id, password: pass } = isDev()
      ? { id: email, password }
      : Credentials.parse({ id: email, password });

    const res = await useFetch("/auth/login", {
      method: "POST",
      body: { id, password: pass },
    });
    if (!res.ok) {
      console.error(res.statusText);
      failure = true;
    } else {
      user = await res.json();
    }
  } catch (error) {
    console.error(error);
    failure = true;
  }
  return [failure, user] as const;
};

export const isDev = () => import.meta.env.VITE_DEV === "true";
