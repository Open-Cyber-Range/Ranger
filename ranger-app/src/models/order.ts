
type NewOrder = {
  name: string;
  clientId: string;
};

type Order = {
  id: string;
  trainingObjectives?: TrainingObjective[];
  createdAt: string;
  updatedAt: string;
} & NewOrder;

type TrainingObjective = {
  objective: string;
  threats: string[];
};

type TrainingObjectiveForm = TrainingObjective;

export type {
  TrainingObjective,
  NewOrder,
  Order,
  TrainingObjectiveForm,
};
