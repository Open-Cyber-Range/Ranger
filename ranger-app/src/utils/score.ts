import {ExerciseRole} from 'src/models/scenario';

export function getExerciseRoleFromString(role: string): ExerciseRole | undefined {
  const roles = Object.keys(ExerciseRole) as ExerciseRole[];
  return roles.find(r => r.toLowerCase() === role.toLowerCase());
}
