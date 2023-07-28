import {type ExerciseRole} from './scenario';

export type DummyManualMetric = {
  id: string;
  name: string;
  maxScore: number;
  role: ExerciseRole;
  textSubmissionValue?: string;
};

export type UpdateDummyManualMetric = {
  score: number;
} & DummyManualMetric;
