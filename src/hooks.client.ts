import { UUIDGeneratorBrowser } from "$lib/utils";
import type { HandleClientError } from "@sveltejs/kit";
// log errors and return a generic error message to users
export const handleError = (({ error, event }) => {
  const errorId = UUIDGeneratorBrowser();
  // implement actual error logging here
  console.error("Error ID:", errorId, "\nerror:", error);

  return {
    message: "Whoops!",
    errorId,
  };
}) satisfies HandleClientError;
