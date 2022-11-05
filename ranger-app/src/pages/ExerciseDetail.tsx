import React from 'react';
import {useParams} from 'react-router-dom';
import type {ExerciseDetailRouteParameters} from 'src/models/routes';
import PageHolder from 'src/components/PageHolder';
import {
  useAddDeploymentMutation,
  useGetDeploymentsQuery,
  useGetExerciseQuery,
} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import ExerciseForm from 'src/components/Exercise/Form';
import DeploymentList from 'src/components/Deployment/List';
import type {NewDeployment} from 'src/models/deployment';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import Header from 'src/components/Header';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';

const ExerciseDetail = () => {
  const {t} = useTranslation();
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();
  const {data: deployments} = useGetDeploymentsQuery(exerciseId ?? skipToken);
  const {data: exercise} = useGetExerciseQuery(exerciseId ?? skipToken);
  useExerciseStreaming(exerciseId);

  const [addDeployment, _newDeployment] = useAddDeploymentMutation();
  const addNewDeployment = async (name: string) => {
    if (exercise?.sdlSchema) {
      try {
        const newDeployment: NewDeployment = {
          name,
          sdlSchema: exercise.sdlSchema,
        };
        const deployment = await addDeployment({
          newDeployment,
          exerciseId: exercise.id,
        }).unwrap();

        if (deployment) {
          toastSuccess(
            t(
              'deployments.addingSuccess',
              {newDeploymentName: newDeployment.name},
            ),
          );
        }
      } catch {
        toastWarning(t('deployments.addingFail'));
      }
    } else {
      toastWarning(t('deployments.sdlMissing'));
    }
  };

  if (exercise && deployments) {
    return (
      <PageHolder>
        <h2>{exercise.name}</h2>

        <ExerciseForm exercise={exercise}/>
        <br/>

        <Header
          headerTitle={t('deployments.title')}
          dialogTitle={t('deployments.title')}
          buttonTitle={t('deployments.add')}
          onSubmit={async name => {
            await addNewDeployment(name);
          }}/>
        <DeploymentList deployments={deployments ?? []}/>
      </PageHolder>
    );
  }

  return null;
};

export default ExerciseDetail;
