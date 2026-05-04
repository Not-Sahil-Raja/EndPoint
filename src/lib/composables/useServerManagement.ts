import { DatabaseService } from "$lib/services/database.js";
import type { Server } from "$lib/types.js";

export function useServerManagement() {
  let showModal = $state(false);
  let editingServer: Server | null = $state(null);

  async function fetchServers() {
    await DatabaseService.servers.getServers();
    await DatabaseService.servers.getServersByGroupsMap();
    await DatabaseService.groups.getAllGroups();
  }

  async function addServer(name: string, url: string) {
    await DatabaseService.servers.addServer(name, url);
    closeModal();
  }

  async function updateServer(id: number, name: string, url: string) {
    await DatabaseService.servers.updateServer(id, name, url);
    closeModal();
  }

  async function deleteServer(id: number) {
    await DatabaseService.servers.deleteServer(id);
  }

  async function clearAllServers() {
    const servers = await DatabaseService.servers.getServers();
    for (const server of servers) {
      await DatabaseService.servers.deleteServer(server.id);
    }
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

  return {
    showModal,
    editingServer,
    
    fetchServers,
    addServer,
    updateServer,
    deleteServer,
    clearAllServers,
    openAddModal,
    openEditModal,
    closeModal,
  };
}
