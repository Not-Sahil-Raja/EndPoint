import { ServersService } from "./servers.service.js";
import { GroupsService } from "./groups.service.js";
import { HealthService } from "./health.service.js";

export class DatabaseService {
    async init() {
        // No initialization needed when using Tauri commands
        // All services are ready to use immediately
    }

    // Expose all services as static properties for easy access
    static servers = new ServersService();
    static groups = new GroupsService();
    static health = new HealthService();
}

// Export individual services for direct import
export { ServersService } from "./servers.service.js";
export { GroupsService } from "./groups.service.js";
export { HealthService } from "./health.service.js";