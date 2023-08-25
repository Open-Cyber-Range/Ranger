import type React from 'react';
import {useLocation, useNavigate, useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
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
import {type ReactNode, useState} from 'react';
import {MENU_HEADER} from '@blueprintjs/core/lib/esm/common/classes';
import {sortByProperty} from 'sort-by-property';

type ActiveTab = 'Dash' | 'Scores' | 'Emails' | undefined;

const hashToTab = (hash: string): ActiveTab => {
  switch (hash) {
    case '#dash': {
      return 'Dash';
    }

    case '#scores': {
      return 'Scores';
    }

    case '#emails': {
      return 'Emails';
    }

    default: {
      return 'Dash';
    }
  }
};

const SideBar = ({renderMainContent}: {
  renderMainContent?: (activeTab: ActiveTab) => ReactNode | undefined;}) => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const {exerciseId, deploymentId}
    = useParams<DeploymentDetailRouteParameters>();
  const {hash} = useLocation();
  const {data: deployments} = useAdminGetDeploymentsQuery(exerciseId ?? skipToken);
  const {data: exercise} = useAdminGetExerciseQuery(exerciseId ?? skipToken);
  const hasDeployments = deployments && deployments.length > 0;
  const [activeTab, setActiveTab] = useState<ActiveTab>(hashToTab(hash));
  if (exercise && deployments) {
    const orderedDeployments = deployments.slice().sort(sortByProperty('updatedAt', 'desc'));
    return (

      <div className='flex h-[100%]'>
        <div className='pb-[2rem]'>
          <Menu large className='max-w-[10rem] bp4-elevation-3 h-[100%]'>
            <div className='flex flex-col max-h-[100%] overflow-y-auto'>
              <div className='mt-[2rem] px-[7px]'>
                <H2>{exercise.name}</H2>
              </div>
              <MenuDivider/>
              <MenuItem
                active={!deploymentId && activeTab === 'Dash'}
                text={t('exercises.tabs.dashboard')}
                icon='control'
                onClick={() => {
                  if (exerciseId) {
                    navigate(`/exercises/${exerciseId}`);
                  }

                  setActiveTab('Dash');
                }}
              />
              <MenuItem
                disabled={!hasDeployments}
                active={!deploymentId && activeTab === 'Scores'}
                text={t('exercises.tabs.scores')}
                icon='chart'
                onClick={() => {
                  if (exerciseId) {
                    navigate(`/exercises/${exerciseId}#scores`);
                  }

                  setActiveTab('Scores');
                }}
              />
              <MenuItem
                active={!deploymentId && activeTab === 'Emails'}
                text={t('emails.link')}
                icon='envelope'
                onClick={() => {
                  if (exerciseId) {
                    navigate(`/exercises/${exerciseId}#emails`);
                  }

                  setActiveTab('Emails');
                }}
              />

              <li className={MENU_HEADER}>
                <H6>{t('deployments.title')}</H6>
              </li>

              {hasDeployments && (
                orderedDeployments.map(deployment => (
                  <MenuItem
                    key={deployment.id}
                    active={deploymentId === deployment.id}
                    text={deployment.name}
                    icon='cloud-upload'
                    onClick={() => {
                      navigate(
                        `/exercises/${deployment.exerciseId}/deployments/${deployment.id}`);
                    }}
                  />
                ))
              )}

            </div>
          </Menu>
        </div>
        <div className='grow m-[2rem] flex justify-center'>
          <div className='max-w-[80rem] w-[60rem]'>
            {renderMainContent?.(activeTab)}
          </div>
        </div>
      </div>
    );
  }

  return null;
};

export default SideBar;
