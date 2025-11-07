<script lang="ts">
  import { onMount } from "svelte";
  import Modal from "../components/common/Modal.svelte";
  import ServerList from "../components/ServerList.svelte";
  import AddServerForm from "../components/AddServerForm.svelte";
  import type { Server } from "../lib/types.js";
  import { checkServerHealth } from "../lib/services/healthcheck";
  import { DatabaseService } from "../lib/services/database";

  let showModal = $state(false);
  let servers: Server[] = $state([]);
  const dbService = new DatabaseService();
  console.log(showModal);
  onMount(async () => {
    await dbService.init();
    await fetchServers();
  });

  async function fetchServers() {
    servers = await dbService.getServers();
    console.log("Fetched servers:", servers);
  }

  async function addServer(url: string, name: string) {
    await dbService.addServer(url, name);
    await fetchServers();
  }

  async function clearAllServers() {
    await dbService.clearAllServers();
    await fetchServers();
  }

  async function handleCheckHealth(server: Server) {
    const result = await checkServerHealth(server);
    if (result) {
      await dbService.updateServerStatus(
        server.id,
        result.status,
        result.response_time
      );
    } else {
      await dbService.updateServerStatus(server.id, "failed", null);
    }
    await fetchServers();
  }
</script>

<main class="container">
  <div class="w-full flex justify-between items-center">
    <h1 class="text-xl font-mono">Welcome to EndPoint!</h1>
    <div class="flex gap-2">
      <button
        class="bg-rose-700 text-base px-3 py-1.5 rounded-xl border border-b-2 border-rose-400 cursor-pointer text-white hover:bg-rose-800 transition"
        onclick={clearAllServers}
      >
        Clear All Servers
      </button>
      <button
        class="bg-indigo-700 text-base px-3 py-1.5 rounded-xl border border-b-2 border-indigo-400 cursor-pointer text-white hover:bg-indigo-800 transition"
        onclick={() => (showModal = true)}
      >
        Add Server
      </button>
    </div>
  </div>

  <ServerList {servers} onCheckHealth={handleCheckHealth} />

  <Modal bind:showModal>
    {#snippet header()}
      <h2 class="text-xl font-semibold mb-4">Add a New Server</h2>
    {/snippet}
    <AddServerForm
      onAddServer={addServer}
      onClose={() => (showModal = false)}
    />
  </Modal>
</main>

<style>
  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    display: flex;
    height: 100vh;
    flex-direction: column;
    gap: 1rem;
  }
  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #1a1b1c;
    }
  }
</style>
