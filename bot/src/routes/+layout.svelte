<script lang="ts">
  import { initialize } from "$lib/initialize";
  import { onMount } from "svelte";
  import { page } from "$app/state";

  onMount(() => {
    const uninitialize = initialize();
    return () => {
      uninitialize();
    };
  });
</script>

<div
  class="bg-gray-900 text-white w-screen h-screen rounded-xl flex flex-col items-center"
>
  <div role="tablist" class="tabs tabs-box">
    <a
      role="tab"
      class="tab"
      href="/map"
      class:tab-active={page.url.pathname === "/map"}>Map</a
    >
    <a
      role="tab"
      class="tab"
      href="/debug"
      class:tab-active={page.url.pathname === "/debug"}>Debug</a
    >
  </div>
  <slot />
</div>
