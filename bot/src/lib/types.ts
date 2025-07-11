import { z } from "zod/v4";

export const BotStateEventSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("ScannerRunningChanged"),
    scanner_running: z.boolean(),
  }),
]);
export type BotStateEvent = z.infer<typeof BotStateEventSchema>;

export const GameStateScannedEventSchema = z.object({
  bytes: z.array(z.number()),
});
export type GameStateScannedEvent = z.infer<typeof GameStateScannedEventSchema>;
