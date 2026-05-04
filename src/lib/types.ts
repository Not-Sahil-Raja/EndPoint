  export type HealthResponse = { status: string; response_time: number | null };

export interface Server {
  id: number;
  name: string;
  url: string;
  status: string | null;
  last_checked: string | null;
  response_time: number | null;
  group_id: number | null;
  sync_enabled: number;
}

export interface Group {
  id: number;
  name: string;
}