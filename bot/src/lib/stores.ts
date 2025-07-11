import type { GameState } from "./protos/Lakeshire_pb";
import { writable } from "svelte/store";

export const gameState = writable<GameState | null>(null);
export const botState = writable<BotState>({ scanner_running: false });
