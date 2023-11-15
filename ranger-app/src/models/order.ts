
type NewOrder = {
  name: string;
  clientId: string;
};

type NewThreat = {
  threat: string;
};

type Threat = NewThreat & {
  id: string;
};

type NewTrainingObjective = {
  objective: string;
  threats: NewThreat[];
};

type TrainingObjective = Omit<NewTrainingObjective, 'threats'> & {
  id: string;
  threats: Threat[];
};

type Order = {
  id: string;
  trainingObjectives?: TrainingObjective[];
  createdAt: string;
  updatedAt: string;
} & NewOrder;

export type {
  NewTrainingObjective,
  NewOrder,
  Order,
  TrainingObjective,
};
