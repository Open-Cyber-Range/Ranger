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
import ExerciseForm from 'src/components/Exercise/Form';
import DeploymentList from 'src/components/Deployment/List';
import type {NewDeployment} from 'src/models/deployment';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import Header from 'src/components/Header';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';

const ExerciseDetail = () => {
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
          toastSuccess(`Deployment "${newDeployment.name}" added`);
        }
      } catch {
        toastWarning('Failed to add the deployment');
      }
    } else {
      toastWarning('Exercise must have an sdl-schema');
    }
  };

  if (exercise && deployments) {
    return (
      <PageHolder>
        <h2>{exercise.name}</h2>

        <ExerciseForm exercise={exercise}/>
        <br/>

        <Header
          headerTitle='Deployments'
          dialogTitle='Add Deployment'
          buttonTitle='Add Deployment'
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
