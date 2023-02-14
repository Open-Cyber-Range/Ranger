type TrainingLearningObjective = {
  name?: string;
  description?: string;
  evaluation: string;
  capabilities?: string[];
};

type MinScore = {
  absolute?: number;
  percentage?: number;
};

type Evaluation = {
  description?: string;
  metrics: string[];
  minScore: MinScore;
};

type ScoreElement = {
  id: string;
  exerciseId: string;
  deploymentId: string;
  tloName?: string;
  metricName: string;
  value: number;
  createdAt: string;
  vmName: string;
};

export type {
  TrainingLearningObjective,
  MinScore,
  Evaluation,
  ScoreElement,
};
