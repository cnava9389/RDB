import type { CTX, USER } from "./types";
import { z } from "zod";

export function getAccount(ctx: CTX): USER | null {
  if (import.meta.env.DEV)
    return {
      id: "1",
      email: "test@test.com",
      name: "testName",
    };
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
