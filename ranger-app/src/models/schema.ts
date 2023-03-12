import {type Entity} from './scenario/entity';
import {type Evaluation} from './scenario/evaluation';
import {type Goal} from './scenario/goal';
import {type Metric} from './scenario/metric';
import {type TrainingLearningObjective} from './scenario/tlo';

export type Schema = {
  entities: Record<string, Entity>;
  goals: Record<string, Goal>;
  tlos: Record<string, TrainingLearningObjective>;
  evaluations: Record<string, Evaluation>;
  metrics: Record<string, Metric>;
};
