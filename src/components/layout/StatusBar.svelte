<script lang="ts">
  import type { Server } from "../../lib/types.js";
  import IconArrowUp from "phosphor-icons-svelte/IconArrowUpBold.svelte";
  import IconArrowDown from "phosphor-icons-svelte/IconArrowDownBold.svelte";
  import CornerBorders from "../ui/CornerBorders.svelte";

  interface Props {
    servers: Server[];
    isRunning: boolean;
    intervalSecs: number;
    lastChecked: Date | null;
  }

  let { servers, isRunning, intervalSecs, lastChecked }: Props = $props();

  let up = $derived(servers.filter((s) => s.status === "200 OK").length);
  let down = $derived(
    servers.filter((s) => s.status?.startsWith("error")).length,
  );
  let unknown = $derived(
    servers.filter((s) => !s.status || s.status === "unknown").length,
  );
  let upPct = $derived(
    servers.length === 0 ? 0 : Math.round((up / servers.length) * 100),
  );

  let nextSyncIn = $state(intervalSecs);

  $effect(() => {
    if (!isRunning) {
      nextSyncIn = intervalSecs;
      return;
    }
    void lastChecked;
    nextSyncIn = intervalSecs;
    const t = setInterval(() => {
      nextSyncIn = Math.max(0, nextSyncIn - 1);
    }, 1000);
    return () => clearInterval(t);
  });

  function fmt(s: number) {
    if (s < 60) return `${s}s`;
    return `${Math.floor(s / 60)}m ${s % 60}s`;
  }

  let syncPct = $derived(Math.round((nextSyncIn / intervalSecs) * 100));
  let healthColor = $derived(
    upPct === 100 ? "#19BB1F" : upPct >= 50 ? "#fbbf24" : "#f87171",
  );
  let syncColor = "#818cf8";
</script>

<div class="grid grid-cols-3 gap-3 font-jetbrains">
  <CornerBorders
    class="flex flex-col gap-4 px-4 pt-3 pb-4 border border-white/15 bg-white/3"
    style="box-shadow: inset 0px 21px 75px -13px rgba(255,255,255,0.07);"
  >
    {#snippet children()}
      <span
        class="text-[11px] uppercase tracking-widest text-white/60 font-medium"
        >Servers</span
      >

      <div class="text-3xl font-bold tabular-nums text-white leading-none">
        {servers.length}
      </div>

      <div class="flex items-end justify-between mt-auto">
        <div>
          <div class="flex items-center gap-1 text-terminal-green">
            <IconArrowUp class="w-3 h-3" />
            <div class="text-xl font-bold tabular-nums">{up}</div>
          </div>
          <div
            class="text-[10px] text-terminal-green/60 uppercase tracking-widest mt-0.5"
          >
            Up
          </div>
        </div>
        <div>
          <div class="flex items-center gap-1 text-rose-400">
            <IconArrowDown class="w-3 h-3" />
            <div class="text-xl font-bold tabular-nums">{down}</div>
          </div>
          <div
            class="text-[10px] text-rose-400/80 uppercase tracking-widest mt-0.5"
          >
            Down
          </div>
        </div>
      </div>
    {/snippet}
  </CornerBorders>

  <CornerBorders
    class="flex flex-col gap-4 px-4 pt-3 pb-4 border border-white/15 bg-white/3"
    style="box-shadow: inset 0px 21px 75px -13px rgba(255,255,255,0.07);"
  >
    {#snippet children()}
      <span
        class="text-[11px] uppercase tracking-widest text-white/60 font-medium"
        >Health</span
      >

      <div class="h-6 overflow-hidden bg-white/5 flex">
        {#if servers.length > 0}
          <div
            class="h-full transition-all duration-500 relative overflow-hidden rounded"
            style="width: {upPct}%; background: {healthColor};"
          >
            <div
              class="absolute inset-0"
              style="background-image: repeating-linear-gradient(-45deg, transparent, transparent 4px, rgba(0,0,0,0.15) 4px, rgba(0,0,0,0.15) 8px);"
            ></div>
          </div>
        {/if}
      </div>

      <div class="flex items-end justify-between mt-auto">
        {#if servers.length > 0}
          <div>
            <div
              class="text-xl font-bold tabular-nums"
              style="color: {healthColor}"
            >
              {upPct}%
            </div>
            <div
              class="text-[10px] text-white/60 uppercase tracking-widest mt-0.5"
            >
              Healthy
            </div>
          </div>
          <div>
            <div class="text-xl font-bold tabular-nums text-white/50">
              {100 - upPct}%
            </div>
            <div
              class="text-[10px] text-white/60 uppercase tracking-widest mt-0.5"
            >
              Degraded
            </div>
          </div>
        {:else}
          <div>
            <div class="text-xl font-bold tabular-nums text-white/20">—</div>
            <div
              class="text-[10px] text-white/30 uppercase tracking-widest mt-0.5"
            >
              No data
            </div>
          </div>
        {/if}
      </div>
    {/snippet}
  </CornerBorders>

  <CornerBorders
    class="flex flex-col gap-4 px-4 pt-3 pb-4 border border-white/15 bg-white/3"
    style="box-shadow: inset 0px 21px 75px -13px rgba(255,255,255,0.07);"
  >
    {#snippet children()}
      <span
        class="text-[11px] uppercase tracking-widest text-white/60 font-medium"
        >Next Sync</span
      >

      <div class="h-6 overflow-hidden bg-white/5 flex">
        {#if isRunning}
          <div
            class="h-full transition-all duration-1000 relative overflow-hidden rounded"
            style="width: {syncPct}%; background: {syncColor};"
          >
            <div
              class="absolute inset-0"
              style="background-image: repeating-linear-gradient(-45deg, transparent, transparent 4px, rgba(0,0,0,0.15) 4px, rgba(0,0,0,0.15) 8px);"
            ></div>
          </div>
        {/if}
      </div>

      <div class="flex items-end justify-between mt-auto">
        {#if isRunning}
          <div>
            <div class="text-xl font-bold tabular-nums font-mono text-white">
              {fmt(nextSyncIn)}
            </div>
            <div
              class="text-[10px] text-white/60 uppercase tracking-widest mt-0.5"
            >
              Remaining
            </div>
          </div>
          <div>
            <div class="text-xl font-bold tabular-nums text-white/50">
              {intervalSecs}s
            </div>
            <div
              class="text-[10px] text-white/60 uppercase tracking-widest mt-0.5"
            >
              Interval
            </div>
          </div>
        {:else}
          <div>
            <div class="text-xl font-bold tabular-nums text-white/20">—</div>
            <div
              class="text-[10px] text-white/30 uppercase tracking-widest mt-0.5"
            >
              Not running
            </div>
          </div>
        {/if}
      </div>
    {/snippet}
  </CornerBorders>
</div>
