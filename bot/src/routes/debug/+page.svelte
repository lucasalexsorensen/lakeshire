<script lang="ts">
  import { botState, gameState } from "$lib/stores";
  import Plot from "$lib/components/Plot.svelte";
  import { onDestroy } from "svelte";

  let recentFacings = $state<[number, number][]>([]);

  const RETENTION = 200;

  const unsubscribe = gameState.subscribe((state) => {
    if (!state) return;

    recentFacings.push([
      new Date().getTime(),
      Number(state.Player!.PosInfo!.Facing) / 1e10,
    ]);
    if (recentFacings.length > RETENTION) {
      recentFacings.shift();
    }
  });

  onDestroy(() => {
    unsubscribe();
  });
</script>

<Plot values={recentFacings} />
Scanner: {$botState.scanner_running ? "✅" : "❌"}
