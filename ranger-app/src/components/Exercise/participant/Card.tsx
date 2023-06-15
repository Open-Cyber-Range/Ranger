import type React from 'react';
import {Card, H2} from '@blueprintjs/core';
import type {ParticipantExercise} from 'src/models/exercise';
import {useParticipantGetDeploymentsQuery} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useNavigate} from 'react-router-dom';

const ExerciseDeployments = ({exercise}: {exercise: ParticipantExercise}) => {
  const navigate = useNavigate();
  const {data: deployments} = useParticipantGetDeploymentsQuery(exercise.id ?? skipToken);

  const handleCardClick = (deploymentId: string) => {
    navigate(`${exercise.id}/deployment/${deploymentId}`);
  };

  return deployments?.map(deployment => (
    <Card
      key={deployment.id}
      interactive
      elevation={2}
      onClick={() => {
        handleCardClick(deployment.id);
      }}
    >
      <div className='flex flex-row justify-between'>
        <H2>{exercise.name}</H2>
      </div>
    </Card>
  ));
};

export default ExerciseDeployments;
