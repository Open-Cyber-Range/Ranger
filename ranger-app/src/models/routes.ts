import {type ExerciseRole} from './scenario';

type ExerciseDetailRouteParameters = {
  exerciseId: string;
};
type DeploymentDetailRouteParameters = {
  exerciseId: string;
  deploymentId: string;
};

type DeploymentDetailScoresRouteParameters = {
  exerciseId: string;
  deploymentId: string;
  role: ExerciseRole;
};
export type {
  ExerciseDetailRouteParameters,
  DeploymentDetailRouteParameters,
  DeploymentDetailScoresRouteParameters,
};
