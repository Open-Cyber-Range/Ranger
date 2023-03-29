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
import type {DeploymentForm, NewDeployment} from 'src/models/deployment';
import {toastSuccess, toastWarning} from 'src/components/Toaster';
import Header from 'src/components/Header';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';
import AddDialog from 'src/components/Deployment/AddDialog';
import styled from 'styled-components';
import {AnchorButton} from '@blueprintjs/core';

const HeaderHolder = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin-bottom: 1rem;
  max-height: 2.5rem;
`;

const ExerciseDetail = () => {
  const {t} = useTranslation();
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();
  const {data: deployments} = useGetDeploymentsQuery(exerciseId ?? skipToken);
  const {data: exercise} = useGetExerciseQuery(exerciseId ?? skipToken);
  useExerciseStreaming(exerciseId);

  const [addDeployment, _newDeployment] = useAddDeploymentMutation();

  const createNewDeployment = (
    name: string,
    deploymentGroup?: string,
  ): [NewDeployment, string] | undefined => {
    if (exercise?.sdlSchema && exercise?.id) {
      return [{
        name,
        sdlSchema: exercise.sdlSchema,
        deploymentGroup,
      }, exercise.id];
    }

    toastWarning(t('deployments.sdlMissing'));
  };

  const createPromises = (
    count: number,
    exerciseId: string,
    deployment: NewDeployment,
  ) => {
    const promises = [];
    if (count < 2) {
      promises.push(
        addDeployment({newDeployment: deployment, exerciseId}),
      );
    } else {
      for (let index = 0; index < count; index += 1) {
        promises.push(
          addDeployment({newDeployment: {
            ...deployment,
            name: `${deployment.name}-${index}`,
          }, exerciseId}),
        );
      }
    }

    return promises.map(async promise =>
      promise.unwrap()
        .then(newDeployment => {
          toastSuccess(
            t(
              'deployments.addingSuccess',
              {newDeploymentName: newDeployment.name},
            ),
          );
        })
        .catch(() => {
          toastWarning(t('deployments.addingFail'));
        }));
  };

  const addNewDeployment = async (
    {count, deploymentGroup, name}: DeploymentForm,
  ) => {
    const deploymentInfo = createNewDeployment(name, deploymentGroup);
    if (deploymentInfo) {
      const [deployment, exerciseId] = deploymentInfo;

      const promises = createPromises(
        count,
        exerciseId,
        deployment,
      );
      await Promise.all(promises);
    }
  };

  if (exercise && deployments) {
    return (
      <PageHolder>
        <HeaderHolder>
          <h2>{exercise.name}</h2>

          <AnchorButton
            large
            intent='primary'
            text={t('emails.link')}
            href={`/exercises/${exercise.id}/email`}
            icon='envelope'
            rightIcon='arrow-right'
          />
        </HeaderHolder>

        <ExerciseForm exercise={exercise}/>
        <br/>

        <Header
          headerTitle={t('deployments.title')}
          buttonTitle={t('deployments.add')}
          onSubmit={async (value: DeploymentForm) => {
            await addNewDeployment(value);
          }}
        >
          <AddDialog
            title={t('deployments.title')}
          />
        </Header>
        <DeploymentList deployments={deployments ?? []}/>
      </PageHolder>
    );
  }

  return null;
};

export default ExerciseDetail;
