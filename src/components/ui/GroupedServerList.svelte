<script lang="ts">
  import { parseISO } from "date-fns";
  import clsx from "clsx";

  import IconCaretDown from "phosphor-icons-svelte/IconCaretDownBold.svelte";
  import IconArrowClockwise from "phosphor-icons-svelte/IconArrowClockwiseBold.svelte";
  import IconPencil from "phosphor-icons-svelte/IconPencilBold.svelte";
  import IconTrash from "phosphor-icons-svelte/IconTrashBold.svelte";
  import IconDotsThree from "phosphor-icons-svelte/IconDotsThreeBold.svelte";
  import IconPlusBold from "phosphor-icons-svelte/IconPlusBold.svelte";
  import IconCheck from "phosphor-icons-svelte/IconCheckBold.svelte";
  import IconX from "phosphor-icons-svelte/IconXBold.svelte";
  import IconSpinner from "phosphor-icons-svelte/IconSpinnerBold.svelte";
  import IconPlay from "phosphor-icons-svelte/IconPlayBold.svelte";

  import { invoke } from "@tauri-apps/api/core";
  import { DatabaseService } from "$lib/services/database";
  import type { Group, Server } from "$lib/types";

  interface Props {
    servers: Server[];
    groups: Group[];
    onCheckHealth: (server: Server) => Promise<void>;
    onEditServer: (server: Server) => void;
    onRefresh: () => Promise<void>;
  }

  let { servers, groups, onCheckHealth, onEditServer, onRefresh }: Props =
    $props();

  let collapsed = $state<Set<string>>(new Set());
  let checking = $state<number | null>(null);
  let openMenu = $state<string | null>(null);
  let renamingId = $state<number | null>(null);
  let renameValue = $state("");
  let creatingGroup = $state(false);
  let newGroupName = $state("");

  let grouped = $derived(() => {
    const result: {
      key: string;
      label: string;
      groupId: number | null;
      servers: Server[];
    }[] = [];

    for (const group of groups) {
      const groupServers = servers.filter((s) => s.group_id === group.id);
      result.push({
        key: `group-${group.id}`,
        label: group.name,
        groupId: group.id,
        servers: groupServers,
      });
    }

    const ungrouped = servers.filter((s) => s.group_id === null);
    if (ungrouped.length > 0 || groups.length === 0) {
      result.push({
        key: "ungrouped",
        label: "Ungrouped",
        groupId: null,
        servers: ungrouped,
      });
    }

    return result;
  });

  function toggleCollapse(key: string) {
    const next = new Set(collapsed);
    next.has(key) ? next.delete(key) : next.add(key);
    collapsed = next;
  }

  function formatTime(ts: string | null): string {
    if (!ts) return "-";
    const normalized = ts.replace(" ", "T") + "Z";
    const secs = Math.floor(
      (Date.now() - parseISO(normalized).getTime()) / 1000,
    );
    if (secs === 0) return "Just now";
    if (secs < 60) return `${secs}s ago`;
    if (secs < 3600) return `${Math.floor(secs / 60)}m ago`;
    if (secs < 86400) return `${Math.floor(secs / 3600)}h ago`;
    return `${Math.floor(secs / 86400)}d ago`;
  }

  function getStatus(status: string | null) {
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

  function closeMenus() {
    openMenu = null;
  }

  async function startRename(group: Group) {
    renamingId = group.id;
    renameValue = group.name;
    openMenu = null;
  }

  async function confirmRename() {
    if (!renamingId || !renameValue.trim()) return;
    await DatabaseService.groups.renameGroup(renamingId, renameValue.trim());
    renamingId = null;
    await onRefresh();
  }

  async function deleteGroup(id: number) {
    await DatabaseService.groups.deleteGroup(id);
    openMenu = null;
    await onRefresh();
  }

  async function handleCheck(server: Server) {
    checking = server.id;
    try {
      await onCheckHealth(server);
    } finally {
      checking = null;
    }
  }

  async function deleteServer(id: number) {
    await DatabaseService.servers.deleteServer(id);
    openMenu = null;
    await onRefresh();
  }

  async function assignGroup(server: Server, groupId: number | null) {
    await DatabaseService.servers.assignServerGroup(server.id, groupId);
    openMenu = null;
    await onRefresh();
  }

  async function toggleServerSync(server: Server) {
    try {
      await DatabaseService.servers.toggleServerSync(server.id);
    } catch (error) {
      console.error(`Failed to toggle sync for server ${server.id}:`, error);
    }
  }

  async function toggleGroupSync(groupId: number | null, enabled: boolean) {
    try {
      await DatabaseService.servers.toggleGroupSync(groupId, enabled);
      openMenu = null;
      await onRefresh();
    } catch (error) {
      console.error(`Failed to toggle group sync for group ${groupId}:`, error);
    }
  }

  function handleWindowClick(e: MouseEvent) {
    const t = e.target as HTMLElement;
    if (!t.closest("[data-menu]")) openMenu = null;
  }
</script>

<svelte:window onclick={handleWindowClick} />

<section
  class="flex-1 flex flex-col gap-3 font-jetbrains pb-10 animate-fade-smooth"
>
  {#if servers.length === 0 && groups.length === 0}
    <div
      class="flex-1 flex flex-col items-center justify-center gap-2 text-center py-20"
    >
      <p class="text-sm text-white/20">No servers added yet.</p>
      <p class="text-xs text-white/10">Click "Add Server" to get started.</p>
    </div>
  {:else}
    <div class="flex flex-col gap-2">
      {#each grouped() as section}
        {@const isCollapsed = collapsed.has(section.key)}

        <div>
          <div class="flex items-center justify-between py-2.5">
            <div class="flex items-center gap-3">
              <button
                onclick={() => toggleCollapse(section.key)}
                class="text-white/20 hover:text-white/50 transition-colors cursor-pointer bg-stone-50/2 hover:bg-stone-50/10 p-1.5 rounded-sm"
              >
                <IconCaretDown
                  class={clsx(
                    "w-3 h-3 transition-transform duration-200",
                    isCollapsed && "-rotate-90",
                  )}
                />
              </button>

              {#if renamingId === section.groupId && section.groupId !== null}
                <input
                  bind:value={renameValue}
                  onkeydown={(e) => {
                    if (e.key === "Enter") confirmRename();
                    if (e.key === "Escape") renamingId = null;
                  }}
                  onblur={confirmRename}
                  class="bg-transparent border-b border-indigo-500/50 text-sm text-white outline-none w-32"
                />
              {:else}
                <span class="text-sm font-medium text-white/70"
                  >{section.label}</span
                >
              {/if}
              <div class="h-4 w-px bg-white/10 self-center"></div>
              <span
                class="inline-flex items-center py-0.5 text-sm uppercase text-white/50 tabular-nums"
              >
                {section.servers.length} server{section.servers.length !== 1
                  ? "s"
                  : ""}
              </span>
            </div>

            {#if section.groupId !== null}
              <div class="relative" data-menu>
                <button
                  onclick={() =>
                    (openMenu = openMenu === section.key ? null : section.key)}
                  class="p-1.5 text-white/50 hover:text-white/80 hover:bg-white/8 transition-all cursor-pointer"
                >
                  <IconDotsThree class="w-4 h-4" />
                </button>

                {#if openMenu === section.key}
                  <div
                    class="absolute right-0 top-8 z-20 w-36 border border-white/10 bg-[#1a1a1b] shadow-xl overflow-hidden"
                  >
                    <button
                      onclick={() =>
                        startRename({
                          id: section.groupId!,
                          name: section.label,
                        })}
                      class="w-full flex items-center gap-2.5 px-3 py-2 text-xs text-white/60 hover:text-white/90 hover:bg-white/8 transition-colors cursor-pointer uppercase"
                    >
                      <IconPencil class="w-3 h-3" /> Rename
                    </button>
                    <button
                      onclick={() => deleteGroup(section.groupId!)}
                      class="w-full flex items-center gap-2.5 px-3 py-2 text-xs text-rose-400/70 hover:text-rose-400 hover:bg-rose-500/8 transition-colors cursor-pointer uppercase"
                    >
                      <IconTrash class="w-3 h-3" /> Delete group
                    </button>
                    <div class="border-t border-white/8">
                      <div
                        class="px-3 py-1.5 text-[10px] uppercase tracking-widest text-white/20"
                      >
                        Sync All
                      </div>
                      <button
                        onclick={() => toggleGroupSync(section.groupId, true)}
                        class="w-full flex items-center gap-2.5 px-3 py-2 text-xs text-terminal-green/70 hover:text-terminal-green hover:bg-terminal-green/8 transition-colors cursor-pointer uppercase"
                      >
                        <IconPlay class="w-3 h-3" /> Enable all
                      </button>
                      <button
                        onclick={() => toggleGroupSync(section.groupId, false)}
                        class="w-full flex items-center gap-2.5 px-3 py-2 text-xs text-white/40 hover:text-white/70 hover:bg-white/8 transition-colors cursor-pointer uppercase"
                      >
                        <IconX class="w-3 h-3" /> Disable all
                      </button>
                    </div>
                  </div>
                {/if}
              </div>
            {/if}
          </div>

          {#if !isCollapsed}
            {#if section.servers.length === 0}
              <div class="px-4 py-4 text-xs text-white/20 italic">
                No servers in this group
              </div>
            {:else}
              <table
                class="w-full"
                style="border-collapse: separate; border-spacing: 0 0.25em;"
              >
                <tbody>
                  {#each section.servers as server}
                    {@const st = getStatus(server.status)}
                    <tr
                      class="group/row bg-white/5 hover:bg-white/10 transition-colors duration-100"
                    >
                      <td class="py-3.5 px-4 w-1/4">
                        <div class="flex items-center gap-2.5">
                          <span
                            class="w-1.5 h-1.5 rounded-full shrink-0 {st.dot}"
                          ></span>
                          <span class="text-sm font-medium text-white/80"
                            >{server.name}</span
                          >
                        </div>
                      </td>

                      <td class="py-3.5 px-4">
                        <span
                          class="text-xs text-white/50 font-mono truncate block max-w-[180px]"
                          >{server.url}</span
                        >
                      </td>

                      <td class="py-3.5 px-4 w-24">
                        <span
                          class="inline-flex items-center px-2 py-0.5 rounded-xs border text-xs font-semibold tabular-nums shadow-inner shadow-white/10 {st.pill}"
                        >
                          {st.label}
                        </span>
                      </td>

                      <td class="py-3.5 px-4 w-24 uppercase">
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
                        <span
                          class="text-xs text-white/25 tabular-nums uppercase"
                          >{formatTime(server.last_checked)}</span
                        >
                      </td>

                      <td class="py-3.5 px-4 w-24">
                        <div
                          class="flex items-center justify-end gap-1 opacity-0 group-hover/row:opacity-100 transition-opacity"
                        >
                          <button
                            onclick={() => handleCheck(server)}
                            disabled={checking === server.id}
                            class="p-1.5 rounded-lg hover:bg-white/8 text-white/30 hover:text-white/70 cursor-pointer disabled:cursor-not-allowed"
                          >
                            {#if checking === server.id}
                              <IconSpinner class="w-3.5 h-3.5 animate-spin" />
                            {:else}
                              <IconArrowClockwise class="w-3.5 h-3.5" />
                            {/if}
                          </button>

                          <button
                            onclick={() => toggleServerSync(server)}
                            class="p-1.5 rounded-lg hover:bg-white/8 {server.sync_enabled
                              ? 'text-terminal-green hover:text-terminal-green'
                              : 'text-white/30 hover:text-white/70'} cursor-pointer"
                            title={server.sync_enabled
                              ? "Disable sync"
                              : "Enable sync"}
                          >
                            <IconPlay class="w-3.5 h-3.5" />
                          </button>

                          <div class="relative" data-menu>
                            <button
                              onclick={() =>
                                (openMenu =
                                  openMenu === `server-${server.id}`
                                    ? null
                                    : `server-${server.id}`)}
                              class="p-1.5 rounded-lg hover:bg-white/8 text-white/30 hover:text-white/70 cursor-pointer"
                            >
                              <IconDotsThree class="w-3.5 h-3.5" />
                            </button>

                            {#if openMenu === `server-${server.id}`}
                              <div
                                class="absolute right-0 top-8 z-20 w-44 border border-white/10 bg-[#1a1a1b] shadow-xl overflow-hidden"
                              >
                                <button
                                  onclick={() => {
                                    onEditServer(server);
                                    openMenu = null;
                                  }}
                                  class="w-full flex items-center gap-2.5 px-3 py-2 text-xs text-white/60 hover:text-white/90 hover:bg-white/8 transition-colors cursor-pointer"
                                >
                                  <IconPencil class="w-3 h-3" /> Edit
                                </button>
                                <div class="border-t border-white/8">
                                  <div
                                    class="px-3 py-1.5 text-[10px] uppercase tracking-widest text-white/20"
                                  >
                                    Move to
                                  </div>
                                  {#each groups as g}
                                    {#if g.id !== server.group_id}
                                      <button
                                        onclick={() =>
                                          assignGroup(server, g.id)}
                                        class="w-full flex items-center gap-2.5 px-3 py-2 text-xs text-white/60 hover:text-white/90 hover:bg-white/8 transition-colors cursor-pointer"
                                      >
                                        {g.name}
                                      </button>
                                    {/if}
                                  {/each}
                                  {#if server.group_id !== null}
                                    <button
                                      onclick={() => assignGroup(server, null)}
                                      class="w-full flex items-center gap-2.5 px-3 py-2 text-xs text-white/40 hover:text-white/70 hover:bg-white/8 transition-colors cursor-pointer"
                                    >
                                      Remove from group
                                    </button>
                                  {/if}
                                </div>
                                <div class="border-t border-white/8">
                                  <button
                                    onclick={() => deleteServer(server.id)}
                                    class="w-full flex items-center gap-2.5 px-3 py-2 text-xs text-rose-400/70 hover:text-rose-400 hover:bg-rose-500/8 transition-colors cursor-pointer"
                                  >
                                    <IconTrash class="w-3 h-3" /> Delete
                                  </button>
                                </div>
                              </div>
                            {/if}
                          </div>
                        </div>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            {/if}
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</section>
