import React from 'react';
import styled from 'styled-components';
import {useGetExercisesQuery} from 'src/slices/apiSlice';
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

  } = useGetExercisesQuery();
  const exercises = potentialExercises ?? [];

  return (
    <Wrapper>
      {exercises.map(exercise => (
        <ExerciseCard key={exercise.id} exercise={exercise}/>
      ))}

    </Wrapper>

  );
};

export default ExerciseList;
