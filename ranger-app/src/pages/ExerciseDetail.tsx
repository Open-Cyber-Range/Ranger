import type React from 'react';
import {useParams} from 'react-router-dom';
import type {ExerciseDetailRouteParameters} from 'src/models/routes';
import {
  useAdminGetDeploymentsQuery,
  useAdminGetExerciseQuery,
} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import ScoresPanel from 'src/components/Scoring/ExerciseScores';
import DashboardPanel from 'src/components/Exercise/Dashboard';
import SendEmail from 'src/components/Email/SendEmail';
import SideBar from 'src/components/Exercise/SideBar';
import useExerciseStreaming from 'src/hooks/useExerciseStreaming';
import {ActiveTab} from 'src/models/exercise';
import {H2} from '@blueprintjs/core';
import {useTranslation} from 'react-i18next';

const ExerciseDetail = () => {
  const {t} = useTranslation();
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();
  useExerciseStreaming(exerciseId);
  const {data: deployments} = useAdminGetDeploymentsQuery(exerciseId ?? skipToken);
  const {data: exercise} = useAdminGetExerciseQuery(exerciseId ?? skipToken);

  if (exercise && deployments) {
    return (
      <SideBar renderMainContent={activeTab => (
        <>
          {activeTab === ActiveTab.Dash && (
            <>
              <H2>{t('exercises.tabs.dashboard')}</H2>
              <DashboardPanel
                exercise={exercise}
                deployments={deployments}
              />
            </>
          )}
          {activeTab === ActiveTab.Scores && (
            <>
              <H2>{t('exercises.tabs.scores')}</H2>
              <ScoresPanel
                deployments={deployments}
              />
            </>)}
          {activeTab === ActiveTab.Emails && (
            <>
              <H2>{t('exercises.tabs.emails')}</H2>
              <SendEmail exercise={exercise}/>
            </>)}
        </>
      )}/>
    );
  }

  return null;
};

export default ExerciseDetail;
