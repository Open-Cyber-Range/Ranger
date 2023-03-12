export type TrainingLearningObjective = {
  name?: string;
  description?: string;
  evaluation: string;
  capabilities?: string[];
};

export type TrainingLearningObjectives =
  Record<string, TrainingLearningObjective>;
