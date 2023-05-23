import type React from 'react';
import {useNavigate, useParams} from 'react-router-dom';
import type {ExerciseDetailRouteParameters} from 'src/models/routes';
import {
  useAdminGetDeploymentsQuery,
  useAdminGetExerciseQuery,
} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {
  H2,
  H6,
  Menu,
  MenuDivider,
  MenuItem,
} from '@blueprintjs/core';
import {useState} from 'react';
import {MENU_HEADER} from '@blueprintjs/core/lib/esm/common/classes';
import ScoresPanel from 'src/components/Scoring/ExerciseScores';
import DashboardPanel from 'src/components/Exercise/Dashboard';
import SendEmail from 'src/components/Email/SendEmail';

const ExerciseDetail = () => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const {exerciseId} = useParams<ExerciseDetailRouteParameters>();
  const {data: deployments} = useAdminGetDeploymentsQuery(exerciseId ?? skipToken);
  const {data: exercise} = useAdminGetExerciseQuery(exerciseId ?? skipToken);
  const hasDeployments = deployments && deployments.length > 0;
  const [activeTab, setActiveTab] = useState<'Dash' | 'Scores' | 'Emails'>('Dash');

  if (exercise && deployments) {
    return (

      <div className='flex h-[100%]'>
        <div className='pb-[2rem]'>
          <Menu large className='max-w-[10rem] bp4-elevation-3 h-[100%]'>
            <div className='mt-[2rem] px-[7px]'>
              <H2>{exercise.name}</H2>
            </div>
            <MenuDivider/>
            <MenuItem
              active={activeTab === 'Dash'}
              text={t('exercises.tabs.dashboard')}
              icon='control'
              onClick={() => {
                setActiveTab('Dash');
              }}
            />
            <MenuItem
              disabled={!hasDeployments}
              active={activeTab === 'Scores'}
              text={t('exercises.tabs.scores')}
              icon='chart'
              onClick={() => {
                setActiveTab('Scores');
              }}
            />
            <MenuItem
              active={activeTab === 'Emails'}
              text={t('emails.link')}
              icon='envelope'
              onClick={() => {
                setActiveTab('Emails');
              }}
            />
            <li className={MENU_HEADER}>
              <H6>{t('deployments.title')}</H6>
            </li>
            {hasDeployments && (
              deployments.map(deployment => (
                <MenuItem
                  key={deployment.id}
                  text={deployment.name}
                  icon='cloud-upload'
                  onClick={() => {
                    navigate(`deployments/${deployment.id}`);
                  }}
                />
              ))
            )}
          </Menu>
        </div>
        <div className='grow m-[2rem] flex justify-center'>
          <div className='max-w-[80rem] w-[60rem]'>
            {activeTab === 'Dash' && (<DashboardPanel
              exercise={exercise}
              deployments={deployments}
            />)}
            {activeTab === 'Scores' && (<ScoresPanel
              deployments={deployments}
            />)}
            {activeTab === 'Emails' && (<SendEmail exercise={exercise}/>)}
          </div>
        </div>
      </div>
    );
  }

  return null;
};

export default ExerciseDetail;
