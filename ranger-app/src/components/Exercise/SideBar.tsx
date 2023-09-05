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

type ActiveTab = 'Dash' | 'Scores' | 'Emails' | 'Graph' | 'SDL'
| 'Accounts' | 'Entity Selector' | 'Manual Metrics' | undefined;

const hashTabs: Record<string, ActiveTab> = {
  '#dash': 'Dash',
  '#scores': 'Scores',
  '#emails': 'Emails',
  '#graph': 'Graph',
  '#sdl': 'SDL',
  '#accounts': 'Accounts',
  '#entities': 'Entity Selector',
  '#metrics': 'Manual Metrics',
};

const hashToTab = (hash: string): ActiveTab => (
  hashTabs[hash] ?? 'Dash'
);

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
            <div className='flex flex-col max-h-[100%] overflow-y-scroll'>
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
                    popoverProps={{hoverCloseDelay: 200}}
                    active={deploymentId === deployment.id}
                    text={deployment.name}
                    icon='cloud-upload'
                    onClick={() => {
                      navigate(
                        `/exercises/${deployment.exerciseId}/deployments/${deployment.id}`);
                    }}
                  >
                    <MenuItem
                      icon='graph'
                      text='Graph'
                      onClick={() => {
                        navigate(
                          // eslint-disable-next-line max-len
                          `/exercises/${deployment.exerciseId}/deployments/${deployment.id}/focus#graph`);
                        setActiveTab('Graph');
                      }}/>
                    <MenuItem
                      icon='text-highlight'
                      text='SDL'
                      onClick={() => {
                        navigate(
                          // eslint-disable-next-line max-len
                          `/exercises/${deployment.exerciseId}/deployments/${deployment.id}/focus#sdl`);
                        setActiveTab('SDL');
                      }}/>
                    <MenuItem
                      icon='join-table'
                      text='Accounts'
                      onClick={() => {
                        navigate(
                          // eslint-disable-next-line max-len
                          `/exercises/${deployment.exerciseId}/deployments/${deployment.id}/focus#accounts`);
                        setActiveTab('Accounts');
                      }}/>
                    <MenuItem
                      icon='data-connection'
                      text='Entity Connector'
                      onClick={() => {
                        navigate(
                          // eslint-disable-next-line max-len
                          `/exercises/${deployment.exerciseId}/deployments/${deployment.id}/focus#entitites`);
                        setActiveTab('Entity Selector');
                      }}/>
                    <MenuItem
                      icon='manually-entered-data'
                      text='Manual Metrics'
                      onClick={() => {
                        navigate(
                          // eslint-disable-next-line max-len
                          `/exercises/${deployment.exerciseId}/deployments/${deployment.id}/focus#metrics`);
                        setActiveTab('Manual Metrics');
                      }}/>
                  </MenuItem>
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
