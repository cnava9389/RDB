import { z } from "zod";
import type { LayoutLoad } from "../../routes/$types";

export const User = z.object({
  id: z.string().uuid(),
  name: z.string().min(4).max(64).trim(),
  email: z.string().email(),
});

export type USER = z.infer<typeof User>;

export type CTX = Parameters<LayoutLoad>[0];
