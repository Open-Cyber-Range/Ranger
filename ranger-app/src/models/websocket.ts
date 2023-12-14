import type {Deployment, DeploymentElement} from './deployment';
import type {UpdateExercise} from './exercise';
import {type Score} from './score';

export enum WebsocketAdminMessageType {
  ExerciseUpdate = 'ExerciseUpdate',
  Deployment = 'Deployment',
  DeploymentElement = 'DeploymentElement',
  DeploymentElementUpdate = 'DeploymentElementUpdate',
  Score = 'Score',
}

export type WebsocketAdminWrapper = {exerciseId: string; ownId: string} & ({
  type: WebsocketAdminMessageType.Deployment;
  content: Deployment;
} | {
  type: WebsocketAdminMessageType.ExerciseUpdate;
  content: UpdateExercise;
} | {
  type: WebsocketAdminMessageType.DeploymentElement;
  content: DeploymentElement;
} | {
  type: WebsocketAdminMessageType.DeploymentElementUpdate;
  content: DeploymentElement;
} | {
  type: WebsocketAdminMessageType.Score;
  content: Score;
});

export enum WebsocketParticipantMessageType {
  Score = 'Score',
}

export type WebsocketParticipantWrapper = {
  exerciseId: string;
  ownId: string;
} & ({
  type: WebsocketParticipantMessageType.Score;
  content: Score;
});
