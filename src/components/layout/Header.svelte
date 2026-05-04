<script lang="ts">
  import { clsx } from "clsx";

  import IconPlusBold from "phosphor-icons-svelte/IconPlusBold.svelte";
  import IconPlay from "phosphor-icons-svelte/IconPlayFill.svelte";
  import IconStop from "phosphor-icons-svelte/IconStopFill.svelte";

  interface Props {
    isRunning: boolean;
    isTogglingSync: boolean;
    serversCount: number;
    onToggleSync: () => Promise<void>;
    onClearAll: () => Promise<void>;
    onOpenAddModal: () => void;
  }

  let {
    isRunning,
    isTogglingSync,
    serversCount,
    onToggleSync,
    onClearAll,
    onOpenAddModal,
  }: Props = $props();
</script>

<header
  class="flex items-center justify-between sticky top-0 z-5 bg-[#272627] pb-3 pt-5 -mx-2 px-2"
>
  <div class="flex items-center gap-2">
    <h1 class="text-lg uppercase">Welcome to EndPoint</h1>
  </div>

  <div class="flex items-center gap-3">
    <button
      onclick={onToggleSync}
      disabled={isTogglingSync}
      class={clsx(
        "flex items-center gap-2 px-3 py-1.5 text-sm rounded-xs border  transition-all duration-150 disabled:opacity-50 disabled:cursor-not-allowed min-w-[120px] uppercase",
        isRunning
          ? "bg-terminal-green/10 border-terminal-green/30 text-terminal-green hover:bg-terminal-green/20 cursor-pointer"
          : "bg-white/10 border-white/10 text-white/60 hover:bg-white/20 hover:text-white/90 cursor-pointer",
      )}
    >
      {#if isTogglingSync}
        <span class="w-4 h-4"></span>
      {:else if isRunning}
        <IconStop class="w-4 h-4" />
      {:else}
        <IconPlay class="w-4 h-4" />
      {/if}
      {#if isTogglingSync}
        {isRunning ? "Stopping…" : "Starting…"}
      {:else}
        {isRunning ? "Syncing" : "Start Sync"}
      {/if}
    </button>

    <div class="w-px h-6 bg-white/10"></div>

    {#if serversCount > 0}
      <button
        onclick={onClearAll}
        class="px-3 py-1.5 rounded-xs text-sm border shadow-inner shadow-white/10 border-white/10 text-white/40 hover:text-rose-400 hover:border-rose-500/40 hover:bg-rose-500/10 transition-all duration-150 cursor-pointer uppercase"
      >
        Clear All
      </button>
    {/if}

    <button
      onclick={onOpenAddModal}
      class="px-3 py-1.5 rounded-xs text-sm bg-[#6220f3] hover:brightness-110 border border-[#cbb4ff] text-white transition-all duration-200 cursor-pointer inline-flex gap-2 items-center uppercase"
      style="box-shadow: inset 0 0 30px 8px rgba(255, 255, 255, 0.4);"
    >
      <span
        class="shrink-0 w-4 h-4 bg-white/80 rounded flex items-center justify-center"
      >
        <IconPlusBold class="shrink-0 w-3 h-3 text-[#651eff]" />
      </span>
      Add Server
    </button>
  </div>
</header>
