import React, {useState, useEffect} from 'react';
import axios from 'axios';
import type {Exercise} from 'src/models/Exercise';
import styled from 'styled-components';
import ExerciseCard from './Card';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;

  > div {
    margin-bottom: 2rem;
  }
`;

const ExerciseList = () => {
  const [exercises, setExercises] = useState<Exercise[]>([]);

  useEffect(() => {
    const fetchData = async () => (axios.get<Exercise[]>('api/v1/exercise'));
    fetchData().then(response => {
      setExercises(response.data);
    }).catch(_error => {
      throw new Error('Error retrieving exercise data');
    });
  }, []);

  return (
    <Wrapper>
      {exercises.map(exercise => (
        <ExerciseCard key={exercise.id} exercise={exercise}/>
      ))}

    </Wrapper>

  );
};

export default ExerciseList;
