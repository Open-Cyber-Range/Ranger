import React from 'react';
import {useParticipantGetExercisesQuery} from 'src/slices/apiSlice';
import humanInterval from 'human-interval';
import {sortByProperty} from 'sort-by-property';
import {Spinner} from '@blueprintjs/core';
import ExerciseDeployments from './ExerciseDeployments';

const ExerciseList = () => {
  const {
    data: potentialExercises, isLoading,
  } = useParticipantGetExercisesQuery(
    undefined,
    {pollingInterval: humanInterval('5 seconds')},
  );
  let exercises = potentialExercises ?? [];
  exercises = exercises.slice().sort(sortByProperty('updatedAt', 'desc'));

  if (!isLoading) {
    return (
      <div className='flex flex-col items-center justify-center space-y-4 min-h-screen'>
        <h4>Loading exercises, please wait.</h4>
        <Spinner/>
      </div>
    );
  }

  return (
    <div className='flex flex-col gap-8'>
      {exercises.map(exercise => (
        <ExerciseDeployments key={exercise.id} exercise={exercise}/>
      ))}
    </div>
  );
};

export default ExerciseList;
