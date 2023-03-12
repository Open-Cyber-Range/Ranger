export enum MetricType {
  Manual,
  Conditional,
}

export type Metric = {
  metric_type: MetricType;
  artifact?: boolean;
  max_score: number;
  condition?: string;
};

export type Metrics = Record<string, Metric>;
