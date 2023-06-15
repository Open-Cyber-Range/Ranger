
type ParticipantDeployment = {
  name: string;
  id: string;
};

type DeploymentForm = {
  name: string;
  deploymentGroup?: string;
  groupName: string;
  count: number;
};

type NewDeployment = {
  name: string;
  deploymentGroup?: string;
  groupName: string;
  sdlSchema: string;
};

type Deployment = {
  id: string;
  exerciseId: string;
  startTime: string;
  endTime: string;
  groupName?: string;
  createdAt: string;
  updatedAt: string;
} & NewDeployment;

type Deployers = Record<string, string[]>;

enum DeployerType {
  Switch = 'switch',
  Template = 'template',
  VirtualMachine = 'virtual_machine',
}

enum ElementStatus {
  Success = 'Success',
  Ongoing = 'Ongoing',
  Failed = 'Failed',
  Removed = 'Removed',
  RemoveFailed = 'RemoveFailed',
}

type DeploymentElement = {
  id: string;
  deploymentId: string;
  scenarioReference: string;
  handlerReference?: string;
  deployerType: DeployerType;
  status: ElementStatus;
};

export type {
  DeployerType,
  ElementStatus,
  ParticipantDeployment,
  NewDeployment,
  Deployment,
  DeploymentElement,
  Deployers,
  DeploymentForm,
};
