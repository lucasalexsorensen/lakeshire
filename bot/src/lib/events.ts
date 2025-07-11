import type { BotStateEvent, GameStateScannedEvent } from "./types";
import { botState, gameState } from "./stores";
import { GameStateSchema } from "$lib/protos/Lakeshire_pb";
import { fromBinary } from "@bufbuild/protobuf";

export function handleBotStateChanged(payload: BotStateEvent) {
  switch (payload.type) {
    case "ScannerRunningChanged":
      botState.update((state) => ({
        ...state,
        scanner_running: payload.scanner_running,
      }));
      break;
  }
}

export function handleGameStateScanned(payload: GameStateScannedEvent) {
  const deserialized = fromBinary(
    GameStateSchema,
    new Uint8Array(payload.bytes)
  );
  gameState.set(deserialized);
}
