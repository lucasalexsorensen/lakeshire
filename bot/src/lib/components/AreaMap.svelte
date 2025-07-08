<script lang="ts">
  import { gameState } from "$lib/stores";
  import { onMount } from "svelte";

  const facingLineLength = 5;

  const mapIdToAreaId: Record<number, number> = {
    1411: 14,
  };

  const mapId = $derived(Number($gameState!.Player!.PosInfo!.MapId));
  const areaId = $derived(mapIdToAreaId[mapId] as number);

  // Map coordinates are floats between 0 and 100
  // these are normalized to the map size
  const playerPos = $derived([
    (Number($gameState!.Player!.PosInfo!.MapX) / 1e14) * 100,
    (Number($gameState!.Player!.PosInfo!.MapY) / 1e14) * 100,
  ]);

  // Facing is a float denoting the radians of the player's facing direction
  // 0 = north
  const playerFacing = $derived(
    Number($gameState!.Player!.PosInfo!.Facing) / 1e10
  );
</script>

<div class="relative">
  <img
    class="object-fill w-screen"
    src={`/areas/${areaId}.png`}
    alt={`Map of ${areaId}`}
  />
  <svg viewBox="0 0 100 100" class="absolute left-0 top-0 w-full h-full">
    <line
      class="stroke-red-500 stroke-[0.5]"
      x1={playerPos[0]}
      y1={playerPos[1]}
      x2={playerPos[0] - facingLineLength * Math.sin(playerFacing)}
      y2={playerPos[1] - facingLineLength * Math.cos(playerFacing)}
    />
    <circle cx={playerPos[0]} cy={playerPos[1]} r="1" class="fill-red-500" />
  </svg>
</div>
{playerFacing}
