export type Story = {
  clock: bigint;
  scripts: string[];
};

export type Stories = Record<string, Story>;
