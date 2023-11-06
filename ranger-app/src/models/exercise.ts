
type ParticipantExercise = {
  id: string;
  name: string;
  updatedAt: string;
};

type NewExercise = {
  name: string;
  sdlSchema?: string;
  groupName?: string;
};

type Exercise = {
  id: string;
  createdAt: string;
  updatedAt: string;
} & NewExercise;

type UpdateExercise = NewExercise;

type DeploymentEvent = {
  id: string;
  name: string;
  description?: string;
  deploymentId: string;
  triggeredAt: string;
};

export enum ActiveTab {
  Dash,
  Scores,
  Emails,
  EmailLogs,
  SDL,
  Accounts,
  EntitySelector,
  UserSubmissions,
}

export type {
  ParticipantExercise,
  NewExercise,
  Exercise,
  UpdateExercise,
  DeploymentEvent,
};
