import type {Deployment, DeploymentElement} from './deployment';
import type {UpdateExercise} from './exercise';

export enum WebsocketMessageType {
  ExerciseUpdate = 'ExerciseUpdate',
  Deployment = 'Deployment',
  DeploymentElement = 'DeploymentElement',
  DeploymentElementUpdate = 'DeploymentElementUpdate',
}

export type WebsocketWrapper = {exerciseId: string; ownId: string} & ({
  type: WebsocketMessageType.Deployment;
  content: Deployment;
} | {
  type: WebsocketMessageType.ExerciseUpdate;
  content: UpdateExercise;
} | {
  type: WebsocketMessageType.DeploymentElement;
  content: DeploymentElement;
} | {
  type: WebsocketMessageType.DeploymentElementUpdate;
  content: DeploymentElement;
});
