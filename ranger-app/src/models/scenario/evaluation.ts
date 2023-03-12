import {type RequireAtLeastOne} from 'src/utils';

export type PotentialMinScore = {
  absolute?: number;
  percentage?: number;
};

type MinScore = RequireAtLeastOne<PotentialMinScore, 'absolute' | 'percentage'>;

export type Evaluation = {
  description?: string;
  metrics: string[];
  min_score?: MinScore;
};

export type Evaluations = Record<string, Evaluation>;
