import type {Deployment, DeploymentElement} from './deployment';
import type {UpdateExercise} from './exercise';
import {type Score} from './score';

export enum WebsocketMessageType {
  ExerciseUpdate = 'ExerciseUpdate',
  Deployment = 'Deployment',
  DeploymentElement = 'DeploymentElement',
  DeploymentElementUpdate = 'DeploymentElementUpdate',
  Score = 'Score',
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
} | {
  type: WebsocketMessageType.Score;
  content: Score;
});
