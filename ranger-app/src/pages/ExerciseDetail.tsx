import React from 'react';
import {useParams} from 'react-router-dom';
import type {ExerciseDetailRouteParameters} from 'src/models/Routes';
import PageHolder from 'src/components/PageHolder';
import {
  useAddDeploymentMutation,
  useGetDeploymentsQuery,
  useGetExerciseQuery,
} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import ExerciseForm from 'src/components/Exercise/Form';
import DeploymentList from 'src/components/Deployment/List';
import type {NewDeployment} from 'src/models/Deployment';
import {Intent} from '@blueprintjs/core';
import {AppToaster} from 'src/components/Toaster';
import Header from 'src/components/Header';
import {useTranslation} from 'react-i18next';

const ExerciseDetail = () => {
  const {t} = useTranslation();
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();
  const {data: deployments} = useGetDeploymentsQuery(exerciseId ?? skipToken);
  const {data: exercise} = useGetExerciseQuery(exerciseId ?? skipToken);

  const [addDeployment, _newDeployment] = useAddDeploymentMutation();
  const addNewDeployment = async (name: string) => {
    if (exercise?.sdlSchema) {
      try {
        const newDeployment: NewDeployment = {
          name,
          sdlSchema: exercise.sdlSchema,
        };
        const newDeploymentName = newDeployment.name;
        await addDeployment({newDeployment, exerciseId: exercise.id});

        AppToaster.show({
          icon: 'tick',
          intent: Intent.SUCCESS,
          message: t('deployments.addingSuccess', {newDeploymentName}),
        });
      } catch {
        AppToaster.show({
          icon: 'warning-sign',
          intent: Intent.DANGER,
          message: t('deployments.addingFail'),
        });
      }
    } else {
      AppToaster.show({
        icon: 'warning-sign',
        intent: Intent.DANGER,
        message: t('deployments.sdlMissing'),
      });
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
          dialogTitle={t('deployments.add')}
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
