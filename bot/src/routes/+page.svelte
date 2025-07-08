<script lang="ts">
  import { registerShortcuts } from "$lib/shortcuts";
  import AreaMap from "$lib/components/AreaMap.svelte";
  import { gameState } from "$lib/stores";
  import { toJson } from "@bufbuild/protobuf";
  import { GameStateSchema } from "$lib/protos/Lakeshire_pb";

  // prints the key,value pairs of an object like 'k=v,k=v,...,k=v'
  // it should work recursively
  const prettifyObject = (obj?: Record<string, any>): string => {
    if (!obj) return "";
    return Object.entries(obj)
      .map(([key, value]) => {
        if (typeof value === "object") {
          return `${key}=(${prettifyObject(value)})`;
        }
        return `${key}=${value}`;
      })
      .join(", ");
  };
</script>

<div
  class="bg-gray-900 text-white w-screen h-screen rounded-xl flex flex-col items-center"
>
  {#if $gameState}
    <AreaMap />
    <div class="p-5">
      <p>{prettifyObject(toJson(GameStateSchema, $gameState)["Player"])}</p>
      <p>{prettifyObject(toJson(GameStateSchema, $gameState)["Target"])}</p>
    </div>
  {:else}
    <p>Loading...</p>
  {/if}
</div>
