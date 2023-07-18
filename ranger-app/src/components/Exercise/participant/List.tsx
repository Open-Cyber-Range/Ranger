import React from 'react';
import {useParticipantGetExercisesQuery} from 'src/slices/apiSlice';
import humanInterval from 'human-interval';
import {sortByProperty} from 'sort-by-property';
import ExerciseDeployments from './ExerciseDeployments';

const ExerciseList = () => {
  const {
    data: potentialExercises,
  } = useParticipantGetExercisesQuery(
    undefined,
    {pollingInterval: humanInterval('5 seconds')},
  );
  let exercises = potentialExercises ?? [];
  exercises = exercises.slice().sort(sortByProperty('updatedAt', 'desc'));

  return (
    <div className='flex flex-col gap-8'>
      {exercises.map(exercise => (
        <ExerciseDeployments key={exercise.id} exercise={exercise}/>
      ))}
    </div>

  );
};

export default ExerciseList;
