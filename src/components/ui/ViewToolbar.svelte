<script lang="ts">
  import { clsx } from "clsx";
  import IconListBold from "phosphor-icons-svelte/IconListBold.svelte";
  import IconSquaresFourBold from "phosphor-icons-svelte/IconSquaresFourBold.svelte";
  import IconMagnifyingGlassBold from "phosphor-icons-svelte/IconMagnifyingGlassBold.svelte";
  import IconPlusBold from "phosphor-icons-svelte/IconPlusBold.svelte";
  import IconXBold from "phosphor-icons-svelte/IconXBold.svelte";
  import IconCheckBold from "phosphor-icons-svelte/IconCheckBold.svelte";

  interface Props {
    viewMode: "list" | "groups";
    search: string;
    serverCount: number;
    groupCount: number;
    onViewModeChange: (mode: "list" | "groups") => void;
    onSearchChange: (value: string) => void;
    onCreateGroupClick: () => void;
  }

  let {
    viewMode,
    search,
    onViewModeChange,
    onSearchChange,
    onCreateGroupClick,
  }: Props = $props();
</script>

<div
  class="flex items-center justify-between w-full gap-4 sticky top-16 z-20 bg-[#272627]"
>
  <div
    class="w-full h-12 bg-linear-to-b from-[#272627] to-80% to-[#272627]/0 absolute -bottom-[120%] left-0 pointer-events-none -z-1"
  ></div>

  <div class="flex items-baseline gap-2 shrink-0">
    <span class="text-base uppercase tracking-wider text-white/70"
      >Server Overview</span
    >
  </div>

  <div class="flex items-center gap-2 flex-1 justify-end">
    {#if viewMode === "groups"}
      <button
        onclick={onCreateGroupClick}
        class="flex items-center gap-1.5 px-3 py-1.75 rounded-sm text-sm text-white/80 border border-white/8 transition-all duration-150 cursor-pointer shrink-0 uppercase relative overflow-hidden group animate-fade-in-up"
      >
        <div
          class="absolute inset-0 bg-linear-to-br from-orange-500/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 blur-lg"
        ></div>
        <div class="relative flex items-center gap-1.5">
          <IconPlusBold class="w-3 h-3" />
          New Group
        </div>
      </button>
    {/if}
    <div class="relative group">
      <div
        class="flex items-center gap-2 px-3 py-1.75 bg-white/2 border border-white/8 rounded transition-all duration-200 focus-within:border-white/25 focus-within:bg-white/5 max-w-xs"
      >
        <IconMagnifyingGlassBold class="w-3.5 h-3.5 text-white/30 shrink-0" />
        <input
          value={search}
          oninput={(e) => onSearchChange((e.target as HTMLInputElement).value)}
          placeholder="Search servers…"
          class="bg-transparent text-sm text-white/60 placeholder:text-white/25 outline-none w-full transition-colors"
        />

        <button
          onclick={() => onSearchChange("")}
          class={clsx(
            "text-white/30 hover:text-white/60 hover:bg-white/10 p-0.5 transition-all duration-150 cursor-pointer shrink-0",
            !search && " opacity-0",
          )}
          disabled={!search}
        >
          <IconXBold class="w-3 h-3" />
        </button>
      </div>
    </div>

    <div
      class="flex items-center bg-white/3 rounded-sm p-0.5 border border-white/8 gap-0.5 shrink-0"
    >
      <button
        onclick={() => onViewModeChange("list")}
        class={clsx(
          "p-1.75 rounded-sm transition-all duration-150 cursor-pointer",
          viewMode === "list"
            ? "bg-white/8 text-white/80"
            : "text-white/25 hover:text-white/50",
        )}
      >
        <IconListBold class="w-4 h-4" />
      </button>
      <button
        onclick={() => onViewModeChange("groups")}
        class={clsx(
          "p-1.75 rounded-sm transition-all duration-150 cursor-pointer",
          viewMode === "groups"
            ? "bg-white/8 text-white/80"
            : "text-white/25 hover:text-white/50",
        )}
      >
        <IconSquaresFourBold class="w-4 h-4" />
      </button>
    </div>
  </div>
</div>
