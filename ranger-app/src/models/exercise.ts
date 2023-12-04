
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

type Banner = {
  name: string;
  content: string;
};

type BannerVariable = {
  name: string;
  content: string;
};

type DeploymentEvent = {
  id: string;
  name: string;
  description?: string;
  deploymentId: string;
  triggeredAt: string;
  eventInfoDataChecksum?: string;
};

type EventInfo = {
  checksum: string;
  name: string;
  fileName: string;
  fileSize: number;
  content: Uint8Array;
  createdAt: string;
};

export enum ActiveTab {
  Dash,
  Banner,
  Scores,
  Emails,
  EmailLogs,
  SDL,
  Accounts,
  EntitySelector,
  UserSubmissions,
}

export type {
  Banner,
  BannerVariable,
  ParticipantExercise,
  NewExercise,
  Exercise,
  UpdateExercise,
  DeploymentEvent,
  EventInfo,
};
