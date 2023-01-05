import * as types from "./types";
import * as auth from "./auth";
import * as store from "./store";

export { types, auth, store };

// its type definition may like this
import type * as app from '@tauri-apps/api/app';

declare global {
  interface Window {
    __TAURI__: {
      app?: typeof app;
      // ... the other tauri modules
    };
  }
}