import { invoke } from "@tauri-apps/api/core";
import type { Server } from "../types.js";

interface HealthCheckStatus {
  is_running: boolean;
  interval_secs: number;
  last_check: number; // Unix timestamp
  next_check: number; // Unix timestamp
}

export class HealthService {
  async checkHealth(serverId: number): Promise<void> {
    return await invoke("check_health", { serverId });
  }

  async startPeriodicHealthChecks(intervalSecs: number): Promise<void> {
    return await invoke("start_periodic_health_checks", { intervalSecs });
  }

  async stopPeriodicHealthChecks(): Promise<void> {
    return await invoke("stop_periodic_health_checks");
  }

  async getPeriodicHealthCheckStatus(): Promise<HealthCheckStatus> {
    return await invoke("get_periodic_health_check_status");
  }
}
