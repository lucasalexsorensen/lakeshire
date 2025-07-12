import { z } from "zod/v4";

export const ScannerStateChangedEventSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("ScannerStateChanged"),
    value: z.boolean(),
  }),
]);
export type ScannerStateChangedEvent = z.infer<
  typeof ScannerStateChangedEventSchema
>;

export const GameStateScannedEventSchema = z.object({
  type: z.literal("GameStateScanned"),
  bytes: z.array(z.number()),
});
export type GameStateScannedEvent = z.infer<typeof GameStateScannedEventSchema>;

export const ActiveWindowChangedEventSchema = z.object({
  type: z.literal("ActiveWindowChanged"),
  title: z.string(),
});
export type ActiveWindowChangedEvent = z.infer<
  typeof ActiveWindowChangedEventSchema
>;

export const BusEventSchema = z.discriminatedUnion("type", [
  ScannerStateChangedEventSchema,
  GameStateScannedEventSchema,
  ActiveWindowChangedEventSchema,
]);
export type BusEvent = z.infer<typeof BusEventSchema>;
