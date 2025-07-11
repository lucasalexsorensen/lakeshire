import { attachConsole } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { handleGameStateScanned, handleBotStateChanged } from "$lib/events";
import { BotStateEventSchema, GameStateScannedEventSchema } from "./types";

export function initialize(): () => Promise<void> {
  const detachPromise = attachConsole();
  const unlisteners: Array<Promise<() => void>> = [];
  window.addEventListener("unload", function (e) {});

  unlisteners.push(
    listen("bot-state-changed", (event) => {
      handleBotStateChanged(BotStateEventSchema.parse(event.payload));
    })
  );

  unlisteners.push(
    listen("game-state-scanned", (event) => {
      handleGameStateScanned(GameStateScannedEventSchema.parse(event.payload));
    })
  );

  invoke("start_scanner");

  const uninitialize = async () => {
    for (const u of unlisteners) {
      (await u)();
    }
    invoke("stop_scanner");
    detachPromise.then((detach) => {
      detach();
    });
  };

  return uninitialize;
}
