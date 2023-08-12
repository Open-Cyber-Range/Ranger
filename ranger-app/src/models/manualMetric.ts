import {type ExerciseRole} from './scenario';

export type NewManualMetric = {
  exerciseId: string;
  deploymentId: string;
  entitySelector: string;
  metricKey: string;
  role: ExerciseRole;
  textSubmission?: string;
};

export type ManualMetric = {
  id: string;
  userId: string;
  name: string;
  description: string;
  score?: number;
  maxScore: number;
} & NewManualMetric;

export type UpdateManualMetric = {
  textSubmission?: string;
  score?: number;
};

export type FetchArtifact = {
  filename?: string;
  url?: string;
};
