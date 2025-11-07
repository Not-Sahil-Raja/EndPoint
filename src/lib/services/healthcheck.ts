import { invoke } from "@tauri-apps/api/core";
import type { Server, HealthResponse } from "../types.js";

export async function checkServerHealth(server: Server): Promise<HealthResponse | null> {
  try {
    const result = await invoke<HealthResponse>("check_health", { url: server.url });
    console.log("Health check result:", result, server);
    return result;
  } catch (error) {
    console.error("Health check failed:", error);
    return null;
  }
}