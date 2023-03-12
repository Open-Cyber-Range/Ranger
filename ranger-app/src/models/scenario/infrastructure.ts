export type InfraNode = {
  count: number;
  links?: string[];
  dependencies?: string[];
};

export type Infrastructure = Record<string, InfraNode>;
