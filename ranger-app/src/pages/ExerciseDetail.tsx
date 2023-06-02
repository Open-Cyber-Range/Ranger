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

const ExerciseDetail = () => {
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();
  const {data: deployments} = useAdminGetDeploymentsQuery(exerciseId ?? skipToken);
  const {data: exercise} = useAdminGetExerciseQuery(exerciseId ?? skipToken);

  if (exercise && deployments) {
    return (
      <SideBar renderMainContent={activeTab => (
        <>
          {activeTab === 'Dash' && (<DashboardPanel
            exercise={exercise}
            deployments={deployments}
          />)}
          {activeTab === 'Scores' && (<ScoresPanel
            exercise={exercise}
            deployments={deployments}
          />)}
          {activeTab === 'Emails' && (<SendEmail exercise={exercise}/>)}
        </>
      )}/>
    );
  }

  return null;
};

export default ExerciseDetail;
