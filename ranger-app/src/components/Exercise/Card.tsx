import React from 'react';
import {Button, Card, H2} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import type {Exercise} from 'src/models/Exercise';
import styled from 'styled-components';

const CardRow = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
`;

const ActionButtons = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
  > button {
    margin-left: 1rem;
  }
`;

const ExerciseCard = ({exercise}: {exercise: Exercise}) => {
  const navigate = useNavigate();

  const routeChange = () => {
    navigate(exercise.id);
  };

  return (
    <Card interactive elevation={2}>
      <CardRow>
        <H2>{exercise.name}</H2>
        <ActionButtons>
          <Button large intent='primary' onClick={routeChange}>
            View
          </Button>
          <Button large intent='danger'> Delete</Button>
        </ActionButtons>
      </CardRow>
    </Card>
  );
};

export default ExerciseCard;
