<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";

  import IconX from "phosphor-icons-svelte/IconXBold.svelte";

  import Modal from "../components/common/Modal.svelte";
  import ServerList from "../components/ui/ServerList.svelte";
  import AddServerForm from "../components/forms/AddServerForm.svelte";
  import CreateGroupForm from "../components/forms/CreateGroupForm.svelte";
  import StatusBar from "../components/layout/StatusBar.svelte";
  import Header from "../components/layout/Header.svelte";

  import type { Server } from "../lib/types.js";
  import { DatabaseService } from "../lib/services/database.js";
  import GroupedServerList from "../components/ui/GroupedServerList.svelte";
  import ViewToolbar from "../components/ui/ViewToolbar.svelte";

  const INTERVAL_SECS = 30;

  let showModal = $state(false);
  let showGroupModal = $state(false);
  let servers: Server[] = $state([]);
  let serversByGroup = $state<Map<number | null, Server[]>>(new Map());
  let groups = $state<{ id: number; name: string }[]>([]);
  let viewMode = $state<"list" | "groups">("list");
  let isRunning = $state(false);
  let lastChecked = $state<Date | null>(null);
  let isTogglingSync = $state(false);
  let editingServer: Server | null = $state(null);
  let search = $state("");

  let filtered = $derived(
    search.trim()
      ? servers.filter(
          (s) =>
            s.name.toLowerCase().includes(search.toLowerCase()) ||
            s.url.toLowerCase().includes(search.toLowerCase()),
        )
      : servers,
  );

  async function initializeHealthCheckState() {
    try {
      const status =
        await DatabaseService.health.getPeriodicHealthCheckStatus();
      isRunning = status.is_running;
      lastChecked =
        status.last_check > 0 ? new Date(status.last_check * 1000) : null;
    } catch (e) {
      console.error("[+page] Failed to initialize health check state:", e);
      isRunning = false;
      lastChecked = null;
    }
  }

  onMount(async () => {
    await fetchServers();
    await initializeHealthCheckState();
    await listen("servers-changed", () => {
      fetchServers();
      lastChecked = new Date();
    });
  });

  async function fetchServers() {
    try {
      servers = await DatabaseService.servers.getServers();
    } catch (e) {
      console.error("[+page] fetchServers getServers error:", e);
      throw e;
    }

    try {
      serversByGroup = await DatabaseService.servers.getServersByGroupsMap();
    } catch (e) {
      console.error("[+page] fetchServers getServersByGroupsMap error:", e);
      throw e;
    }

    try {
      groups = await DatabaseService.groups.getAllGroups();
    } catch (e) {
      console.error("[+page] fetchServers getAllGroups error:", e);
      throw e;
    }
  }

  async function addServer(name: string, url: string, groupId?: number | null) {
    await DatabaseService.servers.addServer(name, url, groupId);
    await fetchServers();
    closeModal();
  }

  async function updateServer(
    id: number,
    name: string,
    url: string,
    groupId?: number | null,
  ) {
    await DatabaseService.servers.updateServer(id, name, url, groupId);
    await fetchServers();
    closeModal();
  }

  function openAddModal() {
    editingServer = null;
    showModal = true;
  }

  function openEditModal(server: Server) {
    editingServer = server;
    showModal = true;
  }

  function closeModal() {
    showModal = false;
    editingServer = null;
  }

  async function clearAllServers() {
    for (const server of servers) {
      await DatabaseService.servers.deleteServer(server.id);
    }
  }

  async function handleCheckHealth(server: Server) {
    try {
      await DatabaseService.health.checkHealth(server.id);
      lastChecked = new Date();
      await fetchServers();
    } catch (error: any) {
      console.error(`Failed to check health of server ${server.id}: ${error}`);
    }
  }

  async function toggleSync() {
    isTogglingSync = true;
    try {
      const status =
        await DatabaseService.health.getPeriodicHealthCheckStatus();
      isRunning = status.is_running;
      lastChecked =
        status.last_check > 0 ? new Date(status.last_check * 1000) : null;

      if (isRunning) {
        await DatabaseService.health.stopPeriodicHealthChecks();
        isRunning = false;
      } else {
        await DatabaseService.health.startPeriodicHealthChecks(INTERVAL_SECS);
        isRunning = true;
      }
    } catch (e) {
      console.error("[+page] toggleSync error:", e);
      try {
        const status =
          await DatabaseService.health.getPeriodicHealthCheckStatus();
        isRunning = status.is_running;
        lastChecked =
          status.last_check > 0 ? new Date(status.last_check * 1000) : null;
      } catch (syncError) {
        console.error("[+page] sync error after toggle error:", syncError);
        isRunning = !isRunning;
      }
    } finally {
      isTogglingSync = false;
    }
  }
</script>

<main class="container font-jetbrains">
  <Header
    {isRunning}
    {isTogglingSync}
    serversCount={servers.length}
    onToggleSync={toggleSync}
    onClearAll={clearAllServers}
    onOpenAddModal={openAddModal}
  />

  <StatusBar {servers} {isRunning} {lastChecked} intervalSecs={INTERVAL_SECS} />

  <ViewToolbar
    {viewMode}
    {search}
    serverCount={servers.length}
    groupCount={groups.length}
    onViewModeChange={(mode) => (viewMode = mode)}
    onSearchChange={(v) => (search = v)}
    onCreateGroupClick={() => (showGroupModal = true)}
  />

  {#if viewMode === "list"}
    <ServerList
      servers={filtered}
      onCheckHealth={handleCheckHealth}
      onEditServer={openEditModal}
    />
  {/if}

  {#if viewMode === "groups"}
    <GroupedServerList
      servers={filtered}
      {groups}
      onCheckHealth={handleCheckHealth}
      onEditServer={openEditModal}
      onRefresh={fetchServers}
    />
  {/if}

  <Modal bind:showModal>
    {#snippet header(closeDialog: () => void)}
      <div class="flex items-start justify-between mb-8">
        <div>
          <h2 class="text-base uppercase font-semibold text-white">
            {editingServer ? "Edit Endpoint" : "Add New Endpoint"}
          </h2>
          <p class="text-xs text-white/40 mt-0.5 uppercase">
            {editingServer
              ? "Update endpoint details"
              : "Monitor a new endpoint"}
          </p>
        </div>
        <button
          type="button"
          onclick={() => {
            closeModal();
            closeDialog?.();
          }}
          class="text-white/30 hover:text-white/70 transition-colors cursor-pointer p-0.5"
        >
          <IconX class="w-4 h-4" />
        </button>
      </div>
    {/snippet}
    {#snippet children(closeDialog: () => void)}
      <AddServerForm
        onAddServer={addServer}
        onUpdateServer={updateServer}
        onClose={() => {
          closeModal();
        }}
        {editingServer}
        {closeDialog}
        {groups}
      />
    {/snippet}
  </Modal>

  <Modal bind:showModal={showGroupModal}>
    {#snippet header(closeDialog: () => void)}
      <div class="flex items-start justify-between mb-8">
        <div>
          <h2 class="text-base uppercase font-semibold text-white">
            Create Group
          </h2>
          <p class="text-xs text-white/40 mt-0.5 uppercase">
            Organize servers into groups
          </p>
        </div>
        <button
          type="button"
          onclick={() => {
            showGroupModal = false;
            closeDialog?.();
          }}
          class="text-white/40 hover:text-white/60 transition-colors cursor-pointer"
        >
          <IconX class="w-4 h-4" />
        </button>
      </div>
    {/snippet}

    {#snippet children(closeDialog: () => void)}
      <CreateGroupForm
        onCreateGroup={async (name: string) => {
          try {
            await DatabaseService.groups.createGroup(name);
            await fetchServers();
            showGroupModal = false;
            closeDialog?.();
          } catch (error: any) {
            throw error;
          }
        }}
        onClose={() => (showGroupModal = false)}
        {closeDialog}
      />
    {/snippet}
  </Modal>
</main>

<style>
  .container {
    min-width: 800px;
    margin: 0 auto;
    display: flex;
    height: 100vh;
    flex-direction: column;
    gap: 1.5rem;
    padding: 0 1rem;
  }
  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #272627;
    }
  }
</style>
