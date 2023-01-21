import { loginUser, checkIfPathIsAllowed, isDev } from "$lib/utils/auth";
import type { CTX } from "$lib/utils/types";
import { vi } from "vitest";

global.fetch = vi.fn(async (url, opts) => {
  let body = {};
  let ok = true;
  const path: string = new URL(url as string).pathname;
  switch (path) {
    case "/auth/login":
      if (!opts) {
        ok = false;
      } else {
        body = opts?.body ?? {};
      }
  }
  return {
    ok,
    json: async () => body,
  } as Response;
});

describe("Testing auth functions", () => {
  let email = "test@test.com";
  let password = "Test@1234";
  import.meta.env.VITE_DEV = "false";

  it("testing valid user login", async () => {
    const [failure, user] = await loginUser(email, password);
    expect(failure).toBe(false);
  });
  it("testing invalid user email login", async () => {
    email = "test";
    const [failure, user] = await loginUser(email, password);
    expect(failure).toBe(true);
  });
  it("testing invalid user password login", async () => {
    password = "test";
    const [failure, user] = await loginUser(email, password);
    expect(failure).toBe(true);
  });

  it("testing pathChecker returns true", () => {
    const path = "/login";
    const result = checkIfPathIsAllowed({
      url: { pathname: path },
    } as CTX);
    expect(result).toBe(true);
  });
  it("testing pathChecker returns false", () => {
    const path = "/";
    const result = checkIfPathIsAllowed({
      url: { pathname: path },
    } as CTX);
    expect(result).toBe(false);
  });
});
