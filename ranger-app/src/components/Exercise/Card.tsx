import type React from 'react';
import {useEffect, useState} from 'react';
import {AnchorButton, Card, H2} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import type {Exercise} from 'src/models/exercise';
import styled from 'styled-components';
import {useTranslation} from 'react-i18next';
import {
  useDeleteExerciseMutation,
  useGetDeploymentsQuery,
} from 'src/slices/apiSlice';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import {Tooltip2} from '@blueprintjs/popover2';

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
  const {t} = useTranslation();
  const navigate = useNavigate();
  const [isPopoverOpen, setIsPopoverOpen] = useState(false);
  const {data: deployments} = useGetDeploymentsQuery(exercise.id);
  const deploymentsExist = deployments && deployments.length > 0;
  const [deleteExercise,
    {isLoading, data, error}] = useDeleteExerciseMutation();

  const handleCardClick = () => {
    if (!isLoading) {
      navigate(exercise.id);
    }
  };

  useEffect(() => {
    if (data) {
      toastSuccess(t('exercises.deleteSuccess', {
        exerciseName: exercise.name,
      }));
    }
  }, [data, exercise.name, t]);

  useEffect(() => {
    if (error) {
      toastWarning(t('exercises.deleteFail', {
        exerciseName: exercise.name,
      }));
    }
  }, [error, exercise.name, t]);

  const onMouseOver = () => {
    if (deploymentsExist) {
      setIsPopoverOpen(!isPopoverOpen);
    }
  };

  return (
    <Card interactive elevation={2} onClick={handleCardClick}>
      <CardRow>
        <H2>{exercise.name}</H2>
        <ActionButtons>

          <Tooltip2
            content='This exercise has active deployments!'
            disabled={!deploymentsExist}
          >
            <div
              onMouseLeave={() => {
                setIsPopoverOpen(false);
              }}
            >
              <AnchorButton
                large
                intent='danger'
                disabled={isLoading
|| deploymentsExist}
                onClick={async event => {
                  event.stopPropagation();
                  await deleteExercise({
                    exerciseId: exercise.id,
                  });
                }}
                onMouseOver={onMouseOver}
              >
                {isLoading ? t('common.deleting') : t('common.delete')}
              </AnchorButton>
            </div>
          </Tooltip2>

        </ActionButtons>
      </CardRow>
    </Card>
  );
};

export default ExerciseCard;
