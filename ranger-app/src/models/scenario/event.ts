export type Event = {
  time?: number;
  conditions?: string[];
  injects: string[];
};

export type Events = Record <string, Event>;
