import type {Deployment, DeploymentElement} from './deployment';
import type {UpdateExercise} from './exercise';
import {type Score} from './score';
import type {Log} from './log';

export enum WebsocketMessageType {
  ExerciseUpdate = 'ExerciseUpdate',
  Deployment = 'Deployment',
  DeploymentElement = 'DeploymentElement',
  DeploymentElementUpdate = 'DeploymentElementUpdate',
  Score = 'Score',
  Log = 'Log',
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

export type WebsocketLogWrapper = {
  type: WebsocketMessageType.Log;
  content: Log;
};
