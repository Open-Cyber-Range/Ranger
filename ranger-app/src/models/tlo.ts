import type {RequireAtLeastOne} from 'src/utils';

type TrainingLearningObjective = {
  name?: string;
  description?: string;
  evaluation: string;
  capabilities?: string[];
};

type PotentialMinScore = {
  absolute?: number;
  percentage?: number;
};

type MinScore = RequireAtLeastOne<PotentialMinScore, 'absolute' | 'percentage'>;

type Evaluation = {
  description?: string;
  metrics: string[];
  minScore: MinScore;
};

type ScoreElement = {
  id: string;
  exerciseId: string;
  deploymentId: string;
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
