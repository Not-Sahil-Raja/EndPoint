<script lang="ts">
  import IconGlobe from "phosphor-icons-svelte/IconGlobeBold.svelte";
  import IconTag from "phosphor-icons-svelte/IconTagBold.svelte";
  import IconFolder from "phosphor-icons-svelte/IconFolderBold.svelte";

  let {
    onAddServer,
    onUpdateServer,
    onClose,
    closeDialog,
    editingServer,
    groups,
  }: {
    onAddServer: (
      url: string,
      name: string,
      groupId?: number | null,
    ) => Promise<void>;
    onUpdateServer: (
      id: number,
      url: string,
      name: string,
      groupId?: number | null,
    ) => Promise<void>;
    onClose: () => void;
    closeDialog?: () => void;
    editingServer?: any;
    groups?: { id: number; name: string }[];
  } = $props();

  let url = $state("");
  let name = $state("");
  let groupId = $state<number | null>(null);
  let loading = $state(false);
  let error = $state("");

  $effect(() => {
    if (editingServer) {
      url = editingServer.url;
      name = editingServer.name;
      groupId = editingServer.group_id || null;
    } else {
      url = "";
      name = "";
      groupId = null;
    }
  });

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!url || !name) return;
    loading = true;
    error = "";
    try {
      if (editingServer) {
        await onUpdateServer(editingServer.id, name, url, groupId);
      } else {
        await onAddServer(name, url, groupId);
      }
      url = "";
      name = "";
      groupId = null;
      onClose();
      closeDialog?.();
    } catch (e: any) {
      error =
        e?.message ?? `Failed to ${editingServer ? "update" : "add"} server.`;
    } finally {
      loading = false;
    }
  }
</script>

<form onsubmit={handleSubmit} class="flex flex-col gap-5 font-jetbrains">
  <div class="flex flex-col gap-3">
    <div class="flex flex-col gap-1.5">
      <label
        for="server-name"
        class="text-[11px] uppercase tracking-widest text-white/40"
      >
        Server Name
      </label>
      <div
        class="flex items-center gap-2.5 px-3 py-2.5 border border-white/10 bg-white/4 focus-within:border-indigo-500/60 focus-within:bg-white/6 transition-all"
      >
        <IconTag class="w-3.5 h-3.5 text-white/20 shrink-0" />
        <input
          id="server-name"
          type="text"
          bind:value={name}
          placeholder="e.g. Production API"
          required
          class="flex-1 bg-transparent text-sm text-white placeholder:text-white/20 outline-none"
        />
      </div>
    </div>

    <div class="flex flex-col gap-1.5">
      <label
        for="server-url"
        class="text-[11px] uppercase tracking-widest text-white/40"
      >
        Server URL
      </label>
      <div
        class="flex items-center gap-2.5 px-3 py-2.5 border border-white/10 bg-white/4 focus-within:border-indigo-500/60 focus-within:bg-white/6 transition-all"
      >
        <IconGlobe class="w-3.5 h-3.5 text-white/20 shrink-0" />
        <input
          id="server-url"
          type="url"
          bind:value={url}
          placeholder="https://example.com"
          required
          class="flex-1 bg-transparent text-sm text-white placeholder:text-white/20 outline-none"
        />
      </div>
    </div>

    {#if groups && groups.length > 0}
      <div class="flex flex-col gap-1.5">
        <label
          for="server-group"
          class="text-[11px] uppercase tracking-widest text-white/40"
        >
          Group
        </label>
        <div
          class="flex items-center gap-2.5 px-3 py-2.5 border border-white/10 bg-white/4 focus-within:border-indigo-500/60 focus-within:bg-white/6 transition-all"
        >
          <IconFolder class="w-3.5 h-3.5 text-white/20 shrink-0" />
          <select
            id="server-group"
            bind:value={groupId}
            class="flex-1 text-sm text-white outline-none appearance-none bg-no-repeat bg-position-[right_12px_center] bg-[length_16px] pr-10 cursor-pointer"
          >
            <option value={null}>No group</option>
            {#each groups as group}
              <option value={group.id}>{group.name}</option>
            {/each}
          </select>
        </div>
      </div>
    {/if}
  </div>

  {#if error}
    <p class="text-xs text-rose-400">{error}</p>
  {/if}

  <div class="flex items-center justify-end gap-2 pt-8">
    <button
      type="button"
      onclick={() => {
        onClose();
        closeDialog?.();
      }}
      class="px-4 py-2 uppercase text-sm text-white/40 hover:text-white/70 border border-white/10 hover:border-white/20 hover:bg-white/5 transition-all cursor-pointer"
    >
      Cancel
    </button>
    <button
      type="submit"
      disabled={loading}
      class="px-4 py-2 uppercase text-sm font-medium bg-indigo-600 hover:bg-indigo-500 border border-indigo-400/30 text-white transition-all cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
    >
      {loading
        ? editingServer
          ? "Updating…"
          : "Adding…"
        : editingServer
          ? "Update Server"
          : "Add Server"}
    </button>
  </div>
</form>

<style>
  #server-group::picker(select) {
    border: none;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(10px);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
    margin-top: 4px;
    max-height: 200px;
    overflow-y: auto;
  }

  #server-group option {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    padding: 12px 16px;
    background: transparent;
    color: white;
    font-size: 14px;
    transition: all 0.2s ease;
    border: none;
    cursor: pointer;
  }

  #server-group option:first-of-type {
    border-radius: 8px 8px 0 0;
  }

  #server-group option:last-of-type {
    border-radius: 0 0 8px 8px;
  }

  #server-group option:not(:last-of-type) {
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  #server-group option:nth-of-type(odd) {
    background: rgba(255, 255, 255, 0.02);
  }

  #server-group option:hover,
  #server-group option:focus {
    background: rgba(99, 102, 241, 0.2);
    color: white;
  }

  #server-group option:checked {
    background: rgba(99, 102, 241, 0.3);
    font-weight: 600;
  }

  #server-group option::checkmark {
    order: 1;
    margin-left: auto;
    content: "✓";
    color: #6366f1;
    font-weight: bold;
  }

  /* Scrollbar styling for the dropdown */
  #server-group::picker(select)::-webkit-scrollbar {
    width: 6px;
  }

  #server-group::picker(select)::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 3px;
  }

  #server-group::picker(select)::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    border-radius: 3px;
  }

  #server-group::picker(select)::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
  }
</style>
