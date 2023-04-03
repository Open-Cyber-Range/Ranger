import type React from 'react';
import {useParams} from 'react-router-dom';
import type {ExerciseDetailRouteParameters} from 'src/models/routes';
import PageHolder from 'src/components/PageHolder';
import {
  useGetDeploymentsQuery,
  useGetExerciseQuery,
} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {H2, Tab, Tabs} from '@blueprintjs/core';
import SendEmail from 'src/components/Email/SendEmail';
import DashboardPanel from 'src/components/Exercise/Dashboard';
import ScoresPanel from 'src/components/Scoring/ExerciseScores';

const ExerciseDetail = () => {
  const {t} = useTranslation();
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();
  const {data: deployments} = useGetDeploymentsQuery(exerciseId ?? skipToken);
  const {data: exercise} = useGetExerciseQuery(exerciseId ?? skipToken);
  const hasDeployments = deployments && deployments.length > 0;

  if (exercise && deployments) {
    return (
      <PageHolder>
        <H2>{exercise.name}</H2>
        <Tabs
          large
          renderActiveTabPanelOnly
        >
          <Tab
            id='Dash'
            title={t('exercises.tabs.dashboard')}
            icon='control'
            panel={<DashboardPanel
              exercise={exercise}
              deployments={deployments}/>}
          />
          <Tab
            id='Scores'
            title={t('exercises.tabs.scores')}
            icon='chart'
            disabled={!hasDeployments}
            panel={<ScoresPanel
              deployments={deployments}/>}
          />
          <Tab
            id='Emails'
            title={t('emails.link')}
            icon='envelope'
            panel={<SendEmail exercise={exercise}/>}
          />
        </Tabs>
      </PageHolder>
    );
  }

  return null;
};

export default ExerciseDetail;
