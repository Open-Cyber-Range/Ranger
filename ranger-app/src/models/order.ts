
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

type NewStructure = {
  name: string;
  description: string;
  parentId?: string;
};

type Structure = NewStructure & {
  id: string;
};

type Order = {
  id: string;
  trainingObjectives?: TrainingObjective[];
  structures?: Structure[];
  createdAt: string;
  updatedAt: string;
} & NewOrder;

export type {
  NewStructure,
  Structure,
  NewTrainingObjective,
  NewOrder,
  Order,
  TrainingObjective,
};
