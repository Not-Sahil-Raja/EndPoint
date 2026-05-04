import { invoke } from "@tauri-apps/api/core";
import type { Group } from "../types.js";

export class GroupsService {
  async getAllGroups(): Promise<Group[]> {
    return await invoke("get_all_groups");
  }

  async createGroup(name: string): Promise<void> {
    await invoke("create_group", { name });
  }

  async deleteGroup(id: number): Promise<void> {
    return await invoke("delete_group", { id });
  }

  async renameGroup(id: number, name: string): Promise<Group> {
    return await invoke("rename_group", { id, name });
  }
}
