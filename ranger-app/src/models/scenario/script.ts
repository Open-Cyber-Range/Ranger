export type Script = {
  start_time: bigint;
  end_time: bigint;
  speed: number;
  events: string[];
};

export type Scripts = Record<string, Script>;
