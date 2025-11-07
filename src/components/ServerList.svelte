<script lang="ts">
  import type { Server } from "../lib/types.js";
  import { getStatusDisplay } from "../lib/utils/statusHelpers.js";

  let {
    servers,
    onCheckHealth,
  }: {
    servers: Server[];
    onCheckHealth: (server: Server) => Promise<void>;
  } = $props();
</script>

<section class="flex-1">
  {#if servers.length === 0}
    <div class="h-full w-full flex items-center justify-center">
      <p class="mt-6 text-center text-gray-500">
        No servers added yet. Click "Add Server" to get started.
      </p>
    </div>
  {:else}
    <ul class="mt-6 space-y-4">
      {#each servers as server}
        {@const statusInfo = getStatusDisplay(server.status)}
        <li
          class="p-4 bg-[#202123] border border-[#2b2b2c] rounded-lg flex justify-between items-center"
        >
          <div>
            <h2 class="text-lg">{server.name}</h2>
            <p class="text-sm text-gray-600">{server.url}</p>
          </div>
          <div>
            <span class={statusInfo.class}>{statusInfo.text}</span>
          </div>
          <button
            class="bg-blue-600 text-white px-3 py-1.5 rounded-lg hover:bg-blue-700 transition cursor-pointer"
            onclick={() => onCheckHealth(server)}
          >
            Check Health
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</section>
