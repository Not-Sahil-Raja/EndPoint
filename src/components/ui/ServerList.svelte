<script lang="ts">
  import { parseISO } from "date-fns";
  import IconArrowClockwise from "phosphor-icons-svelte/IconArrowClockwiseBold.svelte";
  import IconPencil from "phosphor-icons-svelte/IconPencilBold.svelte";
  import IconPlay from "phosphor-icons-svelte/IconPlayBold.svelte";

  import type { Server } from "$lib/types";
  import { DatabaseService } from "$lib/services/database";

  let {
    servers,
    onCheckHealth,
    onEditServer,
  }: {
    servers: Server[];
    onCheckHealth: (server: Server) => Promise<void>;
    onEditServer?: (server: Server) => void;
  } = $props();

  let checking = $state<number | null>(null);

  async function handleCheck(server: Server) {
    checking = server.id;
    try {
      await onCheckHealth(server);
    } finally {
      checking = null;
    }
  }

  async function toggleServerSync(server: Server) {
    try {
      await DatabaseService.servers.toggleServerSync(server.id);
    } catch (error) {
      console.error(`Failed to toggle sync for server ${server.id}:`, error);
    }
  }

  function formatTime(timestamp: string | null): string {
    if (!timestamp) return "—";
    const normalized = timestamp.replace(" ", "T") + "Z";
    const date = parseISO(normalized);
    const secs = Math.floor((Date.now() - date.getTime()) / 1000);

    if (secs === 0) return "Just now";
    if (secs < 60) return `${secs}s ago`;
    if (secs < 3600) return `${Math.floor(secs / 60)}m ago`;
    if (secs < 86400) return `${Math.floor(secs / 3600)}h ago`;
    return `${Math.floor(secs / 86400)}d ago`;
  }

  function getStatus(status: string | null): {
    label: string;
    pill: string;
    dot: string;
  } {
    if (!status || status === "unknown") {
      return {
        label: "Unknown",
        pill: "bg-white/5 text-white/30 border-white/10",
        dot: "bg-white/20",
      };
    }
    if (status.startsWith("error")) {
      return {
        label: "Error",
        pill: "bg-rose-500/10 text-rose-400 border-rose-500/20",
        dot: "bg-rose-400",
      };
    }
    const code = parseInt(status.split(" ")[0], 10);
    if (isNaN(code)) {
      return {
        label: "Unknown",
        pill: "bg-white/5 text-white/30 border-white/10",
        dot: "bg-white/20",
      };
    }
    if (code >= 200 && code < 400) {
      return {
        label: code.toString(),
        pill: "bg-terminal-green/10 text-terminal-green border-terminal-green/20",
        dot: "bg-terminal-green",
      };
    }
    return {
      label: code.toString(),
      pill: "bg-rose-500/10 text-rose-400 border-rose-500/20",
      dot: "bg-rose-400",
    };
  }
</script>

<section
  class="flex-1 flex flex-col font-jetbrains min-h-0 pb-10 animate-fade-smooth"
>
  {#if servers.length === 0}
    <div
      class="flex-1 flex flex-col items-center justify-center gap-2 text-center py-20"
    >
      <p class="text-sm text-white/20">No servers added yet.</p>
      <p class="text-xs text-white/10">Click "Add Server" to get started.</p>
    </div>
  {:else}
    <div class="overflow-hidden">
      <table
        class="w-full"
        style="border-collapse: separate; border-spacing: 0 0.25em;"
      >
        <thead>
          <tr>
            <th
              class="text-left py-2.5 px-4 text-xs uppercase tracking-widest text-white/40"
              >Name</th
            >
            <th
              class="text-left py-2.5 px-4 text-xs uppercase tracking-widest text-white/40"
              >URL</th
            >
            <th
              class="text-left py-2.5 px-4 text-xs uppercase tracking-widest text-white/40"
              >Status</th
            >
            <th
              class="text-left py-2.5 px-4 text-xs uppercase tracking-widest text-white/40"
              >Response</th
            >
            <th
              class="text-left py-2.5 px-4 text-xs uppercase tracking-widest text-white/40 w-28"
              >Last Checked</th
            >
            <th class="py-2.5 px-4 w-24 text-right"></th>
          </tr>
        </thead>

        <tbody class="space-y-2">
          {#each servers as server, i}
            {@const st = getStatus(server.status)}
            <tr
              class="group bg-white/5 hover:bg-white/10 transition-colors duration-100 rounded-lg"
            >
              <td class="py-3.5 px-4">
                <div class="flex items-center gap-2.5">
                  <span class="w-1.5 h-1.5 rounded-full shrink-0 {st.dot}"
                  ></span>
                  <span class="text-sm font-medium text-white/80"
                    >{server.name}</span
                  >
                </div>
              </td>

              <td class="py-3.5 px-4 max-w-[200px]">
                <span class="text-xs text-white/50 font-mono truncate block"
                  >{server.url}</span
                >
              </td>

              <td class="py-3.5 px-4">
                <span
                  class="inline-flex items-center px-2 py-0.5 rounded-xs border text-xs font-semibold tabular-nums shadow-inner shadow-white/10 {st.pill}"
                >
                  {st.label}
                </span>
              </td>

              <td class="py-3.5 px-4 uppercase">
                {#if server.response_time}
                  <span class="text-sm tabular-nums text-white/50">
                    {server.response_time}<span
                      class="text-xs text-white/45 ml-0.5">ms</span
                    >
                  </span>
                {:else}
                  <span class="text-sm text-white/25">-</span>
                {/if}
              </td>

              <td class="py-3.5 px-4 w-32">
                <span class="text-xs text-white/25 tabular-nums uppercase"
                  >{formatTime(server.last_checked)}</span
                >
              </td>

              <td class="py-3.5 px-4">
                <div class="flex items-center justify-end gap-1">
                  <button
                    onclick={() => onEditServer?.(server)}
                    class="opacity-0 group-hover:opacity-100 transition-all duration-150 p-1.5 rounded-lg hover:bg-white/8 text-white/30 hover:text-white/70 cursor-pointer"
                    title="Edit server"
                  >
                    <IconPencil class="w-3.5 h-3.5" />
                  </button>
                  <button
                    onclick={() => handleCheck(server)}
                    disabled={checking === server.id}
                    class="opacity-0 group-hover:opacity-100 transition-all duration-150 p-1.5 rounded-lg hover:bg-white/8 text-white/30 hover:text-white/70 disabled:cursor-not-allowed cursor-pointer"
                    title="Check health"
                  >
                    <IconArrowClockwise
                      class="w-3.5 h-3.5 {checking === server.id
                        ? 'animate-spin'
                        : ''}"
                    />
                  </button>
                  <button
                    onclick={() => toggleServerSync(server)}
                    class="opacity-0 group-hover:opacity-100 transition-all duration-150 p-1.5 rounded-lg hover:bg-white/8 {server.sync_enabled
                      ? 'text-terminal-green hover:text-terminal-green'
                      : 'text-white/30 hover:text-white/70'} cursor-pointer"
                    title={server.sync_enabled ? "Disable sync" : "Enable sync"}
                  >
                    <IconPlay class="w-3.5 h-3.5" />
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</section>
