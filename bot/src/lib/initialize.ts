import { attachConsole } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { handleBusEvent } from "$lib/events";
import { BusEventSchema } from "./types";

export function initialize(): () => Promise<void> {
  const detachPromise = attachConsole();
  const unlisteners: Array<Promise<() => void>> = [];
  window.addEventListener("unload", function (e) {});

  unlisteners.push(
    listen("bus-event", (event) => {
      try {
        handleBusEvent(BusEventSchema.parse(event.payload));
      } catch (error) {
        console.error("Failed to parse bus event", error);
      }
    })
  );

  setTimeout(() => {
    invoke("initialize_app");
  }, 100);

  const uninitialize = async () => {
    for (const u of unlisteners) {
      (await u)();
    }
    invoke("uninitialize_app");
    detachPromise.then((detach) => {
      detach();
    });
  };

  return uninitialize;
}
