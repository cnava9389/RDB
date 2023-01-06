import * as auth from "$lib/utils/auth";
import type { CTX } from "$lib/utils/types";

describe("Testing auth functions", () => {
    let email = "test@test.com"
    let password = "Test@1234"
    import.meta.env.VITE_DEV = "false"
    
  it("testing valid user login", () => {
    const [failure, user] = auth.loginUser(email, password)
    expect(failure).toBe(false);
  });
  it("testing invalid user email login", () => {
    email = "test"
    const [failure, user] = auth.loginUser(email, password)
    console.log(failure,'check')
    expect(failure).toBe(true);
  });
  it("testing invalid user password login", () => {
    password = "test"
    const [failure, user] = auth.loginUser(email, password)
    expect(failure).toBe(true);
  });

  it("testing pathChecker returns true", () => {
    const path = "/login"
    const result = auth.checkIfPathIsAllowed({url:{pathname:path}} as CTX)
    expect(result).toBe(true);
  })
  it("testing pathChecker returns false", () => {
    const path = "/"
    const result = auth.checkIfPathIsAllowed({url:{pathname:path}} as CTX)
    expect(result).toBe(false);
  })
});