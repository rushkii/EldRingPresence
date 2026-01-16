<script lang="ts">
  import "../app.css";

  let { children } = $props();
</script>

<main class="flex flex-col w-full min-h-screen bg-neutral-800 text-white p-4">
  {@render children()}
</main>
