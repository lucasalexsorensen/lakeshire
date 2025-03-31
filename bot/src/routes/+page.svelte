<script lang="ts">
  import { initialize } from "$lib/initialize";
  import { registerShortcuts } from "$lib/shortcuts";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import protobuf from 'protobufjs'

  let name = $state("");
  let greetMsg = $state("");

  let gameStateSchema = $state<protobuf.Type | undefined>(undefined);
  let gameState = $state<protobuf.Message | undefined>(undefined);

  async function greet(event: Event) {
    event.preventDefault();

  }

  onMount(async () => {
    await initialize();
    try {
      await registerShortcuts();
    } catch (error) {}

    const root: protobuf.Root = await new Promise((resolve, reject) => {
      protobuf.load('/protos/Lakeshire.proto', (err, root) => {
        if (err) {
          reject(err);
        } else {
          resolve(root!);
        }
      })
    })
    gameStateSchema = root.lookupType("lakeshire.GameState");

    readGameState();
  })

  async function readGameState() {
    const arrayBuffer: ArrayBuffer = await invoke("read_game_state");
    gameState = gameStateSchema?.decode(new Uint8Array(arrayBuffer));
  }

  setInterval(readGameState, 100);
</script>

<main class="container bg-sky-100">
  {#if gameState}
    <pre>{JSON.stringify(gameState, null, 2)}</pre>
  {:else}
    <p>Loading...</p>
  {/if}
</main>
