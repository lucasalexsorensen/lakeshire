<script lang="ts">
  import AreaMap from "$lib/components/AreaMap.svelte";
  import { gameState } from "$lib/stores";
</script>

{#if $gameState}
  <AreaMap gameState={$gameState} />
{/if}
