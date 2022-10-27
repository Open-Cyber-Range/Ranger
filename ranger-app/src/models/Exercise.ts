
type NewExercise = {
  id: string;
  name: string;
  sdlSchema?: string;
};

type Exercise = {
  createdAt: string;
  updatedAt: string;
} & NewExercise;

export type {NewExercise, Exercise};
