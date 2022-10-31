
type NewDeployment = {
  name: string;
  deploymentGroup?: string;
  sdlSchema: string;
};

type Deployment = {
  id: string;
  exerciseId: string;
  createdAt: string;
  updatedAt: string;
} & NewDeployment;

export type {NewDeployment, Deployment};
