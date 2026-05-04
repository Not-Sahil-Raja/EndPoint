import type { Server } from "$lib/types.js";
import { DatabaseService } from "$lib/services/database.js";

export function useHealthChecks() {
  let isRunning = $state(false);
  let lastChecked = $state<Date | null>(null);
  let isTogglingSync = $state(false);

  async function toggleSync() {
    isTogglingSync = true;
    try {
      if (isRunning) {
        await DatabaseService.health.stopPeriodicHealthChecks();
        isRunning = false;
      } else {
        await DatabaseService.health.startPeriodicHealthChecks();
        isRunning = true;
      }
    } catch (e) {
      console.error('[useHealthChecks] toggleSync error:', e);
    } finally {
      isTogglingSync = false;
    }
  }

  async function handleCheckHealth(server: Server) {
    await DatabaseService.health.checkHealth(server.id);
    lastChecked = new Date();
  }

  return {
    isRunning,
    lastChecked,
    isTogglingSync,
    
    toggleSync,
    handleCheckHealth,
  };
}
