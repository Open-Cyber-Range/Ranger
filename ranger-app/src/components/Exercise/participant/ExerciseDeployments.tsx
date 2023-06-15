import type React from 'react';
import {Card, H2} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import type {ParticipantExercise} from 'src/models/exercise';

const ParticipantExerciseCard = ({exercise}: {exercise: ParticipantExercise}) => {
  const navigate = useNavigate();

  const handleCardClick = () => {
    navigate(exercise.id);
  };

  return (
    <Card interactive elevation={2} onClick={handleCardClick}>
      <div className='flex flex-row justify-between'>
        <H2>{exercise.name}</H2>
      </div>
    </Card>
  );
};

export default ParticipantExerciseCard;
