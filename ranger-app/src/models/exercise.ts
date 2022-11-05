
type NewExercise = {
  name: string;
  sdlSchema?: string;
};

type Exercise = {
  id: string;
  createdAt: string;
  updatedAt: string;
} & NewExercise;

type UpdateExercise = NewExercise;

export type {NewExercise, Exercise, UpdateExercise};
