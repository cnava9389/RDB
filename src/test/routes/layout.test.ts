import { render } from "@testing-library/svelte";
import Layout from "../../routes/+layout.svelte";

describe("Layout", () => {
  it("render page successfully", () => {
    const screen = render(Layout as any);
    expect(screen.getByTestId("layout")).toBeInTheDocument();
  });

  it("test", () => {
    expect(true).toBe(true);
  });
});
