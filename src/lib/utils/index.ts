import * as types from "./types";
import * as auth from "./auth";
import * as store from "./store";
import * as actions from "./actions";

export { listify, range, proxied } from "radash";
import { default as Animate } from "@formkit/auto-animate";

export { types, auth, store, UUIDGeneratorBrowser, actions };
// its type definition may like this
import type * as app from "@tauri-apps/api/app";
import { writable } from "svelte/store";

declare global {
  interface Window {
    __TAURI__: {
      app?: typeof app;
      // ... the other tauri modules
    };
  }
}

const UUIDGeneratorBrowser = () =>
  // @ts-ignore
  ([1e7] + -1e3 + -4e3 + -8e3 + -1e11).replace(/[018]/g, (c) =>
    (
      c ^
      (crypto.getRandomValues(new Uint8Array(1))[0] & (15 >> (c / 4)))
    ).toString(16)
  );

// This function returns a function that is only executed once for a given amount of time.
export function debounce(
  func: (e?: MouseEvent) => (() => void) | void,
  timeout = 500
) {
  const { subscribe, set } = writable(false);
  let busy = false;
  subscribe((value) => (busy = value));
  return (e?: MouseEvent) => {
    if (!busy) {
      set(true);
      const cleanup = func(e);
      setTimeout(() => {
        cleanup ? cleanup() : null;
        set(false);
      }, timeout);
    }
  };
}

export async function tryCatch<T>(fn: () => Promise<T>) {
  try {
    return [false, await fn()] as const;
  } catch (e) {
    // log error
    console.error(e);
    return [true, null as T] as const;
  }
}

export function autoAnimate(node: Node) {
  Animate(node as HTMLElement);
}
