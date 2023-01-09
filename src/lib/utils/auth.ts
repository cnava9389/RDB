import type { CTX, USER } from "./types";
import { Credentials } from "./types";
import { z } from "zod";
import type { types } from ".";

const DEV_USER = { id: "", email: "", name: "test" };

// on each request this will check the context and return a user if user has been authenticated
export function getAccountFromCtx(ctx: CTX): USER | null {
  console.log(ctx);
  if (isDev()) return DEV_USER;
  // !todo
  return null;
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
) => readonly [boolean, types.USER] = (email, password) => {
  let failure = false;
  let user = { id: "", email: "", name: "" };
  try {
    if (isDev()) {
      user = DEV_USER;
    }

    if (email === "" || password === "")
      throw new Error("Email or password is empty");

    const { id, password: pass } = Credentials.parse({ id: email, password });
    // axios call
    // !todo
  } catch (error) {
    // !todo log error
    // console.error(error)
    failure = !isDev();
  }
  return [failure, user] as const;
};

export const isDev = () => import.meta.env.VITE_DEV === "true";
