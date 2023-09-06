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
import {ActiveTab} from 'src/models/exercise';

const hashTabs: Record<string, ActiveTab> = {
  '#dash': ActiveTab.Dash,
  '#scores': ActiveTab.Scores,
  '#emails': ActiveTab.Emails,
  '#sdl': ActiveTab.SDL,
  '#accounts': ActiveTab.Accounts,
  '#entities': ActiveTab.EntitySelector,
  '#metrics': ActiveTab.ManualMetrics,
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
  const [activeTab, setActiveTab] = useState<ActiveTab>(hashTabs[hash] ?? ActiveTab.Dash);
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
                active={!deploymentId && activeTab === ActiveTab.Dash}
                text={t('exercises.tabs.dashboard')}
                icon='control'
                onClick={() => {
                  if (exerciseId) {
                    navigate(`/exercises/${exerciseId}`);
                  }

                  setActiveTab(ActiveTab.Dash);
                }}
              />
              <MenuItem
                disabled={!hasDeployments}
                active={!deploymentId && activeTab === ActiveTab.Scores}
                text={t('exercises.tabs.scores')}
                icon='chart'
                onClick={() => {
                  if (exerciseId) {
                    navigate(`/exercises/${exerciseId}#scores`);
                  }

                  setActiveTab(ActiveTab.Scores);
                }}
              />
              <MenuItem
                active={!deploymentId && activeTab === ActiveTab.Emails}
                text={t('emails.link')}
                icon='envelope'
                onClick={() => {
                  if (exerciseId) {
                    navigate(`/exercises/${exerciseId}#emails`);
                  }

                  setActiveTab(ActiveTab.Emails);
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
                      icon='chart'
                      text={t('exercises.tabs.scores')}
                      onClick={() => {
                        navigate(
                          // eslint-disable-next-line max-len
                          `/exercises/${deployment.exerciseId}/deployments/${deployment.id}/focus#scores`);
                        setActiveTab(ActiveTab.Scores);
                      }}/>
                    <MenuItem
                      icon='text-highlight'
                      text={t('exercises.tabs.sdl')}
                      onClick={() => {
                        navigate(
                          // eslint-disable-next-line max-len
                          `/exercises/${deployment.exerciseId}/deployments/${deployment.id}/focus#sdl`);
                        setActiveTab(ActiveTab.SDL);
                      }}/>
                    <MenuItem
                      icon='join-table'
                      text={t('exercises.tabs.accounts')}
                      onClick={() => {
                        navigate(
                          // eslint-disable-next-line max-len
                          `/exercises/${deployment.exerciseId}/deployments/${deployment.id}/focus#accounts`);
                        setActiveTab(ActiveTab.Accounts);
                      }}/>
                    <MenuItem
                      icon='data-connection'
                      text={t('exercises.tabs.entities')}
                      onClick={() => {
                        navigate(
                          // eslint-disable-next-line max-len
                          `/exercises/${deployment.exerciseId}/deployments/${deployment.id}/focus#entitites`);
                        setActiveTab(ActiveTab.EntitySelector);
                      }}/>
                    <MenuItem
                      icon='manually-entered-data'
                      text={t('exercises.tabs.metrics')}
                      onClick={() => {
                        navigate(
                          // eslint-disable-next-line max-len
                          `/exercises/${deployment.exerciseId}/deployments/${deployment.id}/focus#metrics`);
                        setActiveTab(ActiveTab.ManualMetrics);
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
