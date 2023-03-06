
type DeploymentForm = {
  name: string;
  deploymentGroup: string;
  count: number;
};

type NewDeployment = {
  name: string;
  deploymentGroup?: string;
  sdlSchema: string;
};

type Deployment = {
  id: string;
  exerciseId: string;
  startTime: string;
  endTime: string;
  createdAt: string;
  updatedAt: string;
} & NewDeployment;

type Deployers = Record<string, string[]>;

export enum DeployerType {
  Switch = 'switch',
  Template = 'template',
  VirtualMachine = 'virtual_machine',
}

export enum ElementStatus {
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
  NewDeployment,
  Deployment,
  DeploymentElement,
  Deployers,
  DeploymentForm,
};
