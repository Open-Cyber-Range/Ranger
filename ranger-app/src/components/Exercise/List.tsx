import React from 'react';
import styled from 'styled-components';
import {useGetExercisesQuery} from 'src/slices/apiSlice';
import {sortByUpdatedAtDescending} from 'src/utils';
import humanInterval from 'human-interval';
import ExerciseCard from './Card';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;

  > div {
    margin-bottom: 2rem;
  }
`;

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
    <Wrapper>
      {exercises.map(exercise => (
        <ExerciseCard key={exercise.id} exercise={exercise}/>
      ))}

    </Wrapper>

  );
};

export default ExerciseList;
