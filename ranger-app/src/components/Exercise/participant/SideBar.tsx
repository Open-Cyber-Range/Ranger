import type React from 'react';
import {useLocation, useNavigate, useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {
  useParticipantGetDeploymentQuery,
  useParticipantGetExerciseQuery,
} from 'src/slices/apiSlice';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {
  H2,
  H4,
  Menu,
  MenuDivider,
  MenuItem,
} from '@blueprintjs/core';
import {useState} from 'react';

type ActiveTab = 'Dash' | 'Score' | 'Events' | 'Accounts' | undefined;

const hashToTab = (hash: string): ActiveTab => {
  switch (hash) {
    case '#dash': {
      return 'Dash';
    }

    default: {
      return 'Dash';
    }
  }
};

const SideBar = () => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const {exerciseId, deploymentId}
    = useParams<DeploymentDetailRouteParameters>();
  const {hash} = useLocation();
  const {data: exercise} = useParticipantGetExerciseQuery(exerciseId ?? skipToken);
  const deploymentQueryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: deployment} = useParticipantGetDeploymentQuery(deploymentQueryArguments);
  const [activeTab, setActiveTab] = useState<ActiveTab>(hashToTab(hash));

  if (exercise && deployment) {
    return (

      <div className='flex h-[100%]'>
        <div className='pb-[2rem]'>
          <Menu large className='max-w-[10rem] bp4-elevation-3 h-[100%]'>
            <div className='flex flex-col max-h-[100%] overflow-y-auto'>
              <div className='mt-[2rem] px-[7px]'>
                <H2>{exercise.name}</H2>
                <H4>{deployment.name}</H4>
              </div>
              <MenuDivider/>
              <MenuItem
                active={!deploymentId && activeTab === 'Dash'}
                text={t('participant.exercise.tabs.dash')}
                icon='control'
                onClick={() => {
                  navigate(`/exercises/${exercise.id}/deployments/${deployment.id}#dash`);

                  setActiveTab('Dash');
                }}
              />

              <MenuItem
                active={activeTab === 'Score'}
                text={t('participant.exercise.tabs.score')}
                icon='chart'
                onClick={() => {
                  navigate(`/exercises/${exercise.id}/deployments/${deployment.id}#dash`);

                  setActiveTab('Score');
                }}
              />

              <MenuItem
                active={activeTab === 'Events'}
                text={t('participant.exercise.tabs.events')}
                icon='timeline-events'
                onClick={() => {
                  navigate(`/exercises/${exercise.id}/deployments/${deployment.id}#dash`);

                  setActiveTab('Events');
                }}
              />

              <MenuItem
                active={activeTab === 'Accounts'}
                text={t('participant.exercise.tabs.accounts')}
                icon='mugshot'
                onClick={() => {
                  navigate(`/exercises/${exercise.id}/deployments/${deployment.id}#dash`);

                  setActiveTab('Accounts');
                }}
              />
            </div>
          </Menu>
        </div>
      </div>
    );
  }

  return null;
};

export default SideBar;
