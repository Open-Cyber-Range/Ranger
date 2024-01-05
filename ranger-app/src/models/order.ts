
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

type NewSkill = {
  skill: string;
};

type Skill = NewSkill & {
  id: string;
};

type NewTrainingObjectiveConnection = {
  trainingObjectiveId: string;
};

type TrainingObjectiveConnection = NewTrainingObjectiveConnection & {
  id: string;
};

type NewWeakness = {
  weakness: string;
};

type Weakness = NewWeakness & {
  id: string;
};

type NewStructure = {
  name: string;
  description?: string;
  parentId?: string;
  skills?: NewSkill[];
  trainingObjectiveIds?: NewTrainingObjectiveConnection[];
  weaknesses?: NewWeakness[];
};

type Structure = Omit<NewStructure, 'skills' | 'weaknesses' | 'trainingObjectiveIds' > & {
  id: string;
  skills?: Skill[];
  trainingObjectiveIds?: TrainingObjectiveConnection[];
  weaknesses?: Weakness[];
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
  Skill,
  NewSkill,
  Weakness,
  NewWeakness,
  TrainingObjectiveConnection,
  NewTrainingObjectiveConnection,
};
