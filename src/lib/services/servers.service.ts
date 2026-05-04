import { invoke } from "@tauri-apps/api/core";
import type { Server } from "../types.js";

export class ServersService {
  async getServers(): Promise<Server[]> {
    return await invoke("get_servers");
  }

  async getServer(id: number): Promise<Server> {
    return await invoke("get_server", { id });
  }

  async addServer(name: string, url: string, groupId?: number | null): Promise<Server> {
    return await invoke("add_server", { name, url, group_id: groupId });
  }

  async updateServer(id: number, name: string, url: string, groupId?: number | null): Promise<Server> {
    return await invoke("update_server", { id, name, url, group_id: groupId });
  }

  async deleteServer(id: number): Promise<void> {
    return await invoke("delete_server", { id });
  }

  async getServersByGroupsMap(): Promise<Map<number | null, Server[]>> {
    try {
      const result = await invoke<Record<string, Server[]>>("get_servers_by_groups_map");
      const map = new Map();
      for (const [key, servers] of Object.entries(result)) {
        const mapKey = key === 'null' ? null : Number(key);
        map.set(mapKey, servers);
      }
      return map;
    } catch (error) {
      console.error('[ServersService] getServersByGroupsMap error:', error);
      throw error;
    }
  }

  async assignServerGroup(serverId: number, groupId: number | null): Promise<void> {
    return await invoke("assign_server_group", { serverId, groupId });
  }

  async toggleServerSync(serverId: number): Promise<Server> {
    return await invoke("toggle_server_sync", { serverId });
  }

  async toggleGroupSync(groupId: number | null, enabled: boolean): Promise<Server[]> {
    return await invoke("toggle_group_sync", { groupId, enabled });
  }
}
