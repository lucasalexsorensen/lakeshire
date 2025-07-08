<script lang="ts">
  import { GameStateSchema } from "$lib/protos/Lakeshire_pb";
  import { gameState } from "$lib/stores";
  import { invoke, Channel } from "@tauri-apps/api/core";
  import { fromBinary } from "@bufbuild/protobuf";
  import { listen } from "@tauri-apps/api/event";

  import { initialize } from "$lib/initialize";
  import { onMount } from "svelte";

  onMount(() => {
    console.log("MOUNT!");
    const uninitialize = initialize();

    const channel = new Channel<Uint8Array>();
    let lastTimeReceived: number | undefined;
    channel.onmessage = (event) => {
      const now = Date.now();
      if (lastTimeReceived) {
        console.log("time since last message", now - lastTimeReceived);
      }
      lastTimeReceived = now;
      $gameState = fromBinary(GameStateSchema, new Uint8Array(event));
    };

    invoke("init_scan_loop", { onEvent: channel });

    return () => {
      uninitialize();
    };
  });
</script>

<slot />
