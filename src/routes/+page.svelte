<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { gameCore } from "../hooks/game.svelte";
  import type { GameStatus } from "../types/core.type";

  let gameStatus = $state<GameStatus>({ attached: false, process_name: "" });
  let statusMessage = $state("");

  $effect(() => {
    const interval = setInterval(async () => {
      gameStatus = await gameCore.refreshStatus();
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  });

  onMount(async () => {
    statusMessage = await gameCore.attachToGame();
    await gameCore.listen();
  });

  onDestroy(async () => {
    await gameCore.unlisten();
  });
</script>

<div class="flex flex-col items-center gap-y-2 mx-auto">
  <h1 class="text-2xl font-bold">Elden Ring Presence (Beta Preview)</h1>

  <div class="flex flex-col items-center status">
    <p>Status: {statusMessage}</p>
  </div>

  {#if gameStatus.attached}
    <pre
      class="flex h-full w-160 bg-neutral-900 p-4 rounded-xl mt-2">{JSON.stringify(
        gameCore.event,
        null,
        2,
      )}</pre>
  {/if}
</div>
