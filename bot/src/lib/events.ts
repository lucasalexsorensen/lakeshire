import type {
  BusEvent,
  GameStateScannedEvent,
  ScannerStateChangedEvent,
} from "./types";
import { botState, gameState } from "./stores";
import { GameStateSchema } from "$lib/protos/Lakeshire_pb";
import { fromBinary } from "@bufbuild/protobuf";

export function handleBusEvent(payload: BusEvent) {
  switch (payload.type) {
    case "ScannerStateChanged":
      handleScannerStateChanged(payload);
      break;
    case "GameStateScanned":
      handleGameStateScanned(payload);
      break;
  }
}

function handleScannerStateChanged(payload: ScannerStateChangedEvent) {
  botState.update((state) => ({
    ...state,
    scanner_running: payload.value,
  }));
}

function handleGameStateScanned(payload: GameStateScannedEvent) {
  const deserialized = fromBinary(
    GameStateSchema,
    new Uint8Array(payload.bytes)
  );
  gameState.set(deserialized);
}
