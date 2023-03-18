import React from 'react';
import {useGetExercisesQuery} from 'src/slices/apiSlice';
import {sortByUpdatedAtDescending} from 'src/utils';
import humanInterval from 'human-interval';
import ExerciseCard from './Card';

const ExerciseList = () => {
  const {
    data: potentialExercises,
  } = useGetExercisesQuery(
    undefined,
    {pollingInterval: humanInterval('5 seconds')},
  );
  let exercises = potentialExercises ?? [];
  exercises = exercises.slice().sort(sortByUpdatedAtDescending);

  return (
    <div className='flex flex-col [&>div]:mb-8'>
      {exercises.map(exercise => (
        <ExerciseCard key={exercise.id} exercise={exercise}/>
      ))}

    </div>

  );
};

export default ExerciseList;
