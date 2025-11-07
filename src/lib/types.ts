  export type HealthResponse = { status: string; response_time: number | null };

  export interface Server {
    id: number;
    name: string;
    url: string;
    status: string;
    last_checked: string | null;
    response_time: number | null;
  }