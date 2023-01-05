import { z } from "zod";
import type { LayoutLoad } from "../../routes/$types";

export const User = z.object({
  id: z.string().uuid(),
  name: z.string().min(4).max(64).trim(),
  email: z.string().email(),
});

// add regex to check requirements for password and id
export const Credentials = z.object({
  id: z.string().email().or(z.string().trim().min(5)),
  // give me regex for password
  password: z.string().trim().min(5).regex(/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/)
})

export type CREDENTIALS = z.infer<typeof Credentials>

export type USER = z.infer<typeof User>;

export type CTX = Parameters<LayoutLoad>[0];

///////////////////////////////////////// ui 

export type Input = {
  type: "text" | "email" | "password";
  placeholder: string;
  className: string;
  name: string;
}

export type Button = {
  type: "submit" | "button";
  className:string;
  onClick: (event:MouseEvent) => void,
}

// create a type that contains tailwind colors
export type Colors = "" | "red" | "blue" | "white" | "green" | "slate" | "black" | "yellow" | "green";
export const colors: Record<Colors, string> = {
  "": "",
  red: "bg-red-700 hover:bg-red-500",
  blue: "bg-blue-700 hover:bg-blue-500",
  white: "bg-white-700 hover:bg-white-500",
  green: "bg-green-700 hover:bg-green-500",
  slate: "bg-slate-700 hover:bg-slate-500",
  black: "bg-black-700 hover:bg-black-500",
  yellow: "bg-yellow-700 hover:bg-yellow-500"
}

export type Icons = "" | "add" | "question";

type EventHandler<E extends Event = Event, T extends EventTarget = Element> =
(event: E & { currentTarget: EventTarget & T}) => any;

export type FormEventHandler = EventHandler<Event, HTMLFormElement>;