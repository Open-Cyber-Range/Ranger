export type Goal = {
  name?: string;
  description?: string;
  tlos: string[];
};

export type Goals = Record<string, Goal>;
