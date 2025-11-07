import Database from "@tauri-apps/plugin-sql";
import type { Server } from "../types.js";

export class DatabaseService {
    private db: any = null;

    async init() {
        this.db = await Database.load("sqlite:app.db");
    }

    async getServers(): Promise<Server[]> {
        return await this.db.select("SELECT * FROM servers");
    }

    async addServer(url: string, name: string): Promise<void> {
        await this.db.execute("INSERT INTO servers (url, name) VALUES (?, ?)", [url, name]);
    }

    async clearAllServers(): Promise<void> {
        await this.db.execute("DELETE FROM servers");
    }

    async updateServerStatus(id: number, status: string, response_time: number | null): Promise<void> {
        await this.db.execute(
            "UPDATE servers SET status = ?, last_checked = CURRENT_TIMESTAMP, response_time = ? WHERE id = ?",
            [status, response_time, id]
        );
    }
}