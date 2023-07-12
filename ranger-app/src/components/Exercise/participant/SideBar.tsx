import type React from 'react';
import {useLocation, useNavigate, useParams} from 'react-router-dom';
import type {DeploymentDetailRouteParameters} from 'src/models/routes';
import {
  useParticipantGetDeploymentsQuery,
  useParticipantGetExercisesQuery,
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
import DeploymentList from './DeploymentList';

type ActiveTab = 'Dash' | undefined;

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

const SideBar = ({renderMainContent}: {
  renderMainContent?: (activeTab: ActiveTab) => ReactNode | undefined;}) => {
  const {t} = useTranslation();
  const navigate = useNavigate();
  const {exerciseId, deploymentId}
    = useParams<DeploymentDetailRouteParameters>();
  const {hash} = useLocation();
  const {data: deployments} = useParticipantGetDeploymentsQuery(exerciseId ?? skipToken);
  const {data: exercises} = useParticipantGetExercisesQuery();
  const [activeTab, setActiveTab] = useState<ActiveTab>(hashToTab(hash));

  if (exercises && deployments) {
    return (

      <div className='flex h-[100%]'>
        <div className='pb-[2rem]'>
          <Menu large className='max-w-[10rem] bp4-elevation-3 h-[100%]'>
            <div className='flex flex-col max-h-[100%] overflow-y-auto'>
              <div className='mt-[2rem] px-[7px]'>
                <H2>Main menu</H2>
              </div>
              <MenuDivider/>
              <MenuItem
                active={!deploymentId && activeTab === 'Dash'}
                text={t('exercises.tabs.dashboard')}
                icon='control'
                onClick={() => {
                  if (exerciseId && deploymentId) {
                    navigate(`/exercises/${exerciseId}/deployments/${deploymentId}#dash`);
                  }

                  setActiveTab('Dash');
                }}
              />

              <li className={MENU_HEADER}>
                <H6>Exercises</H6>
              </li>
              {exercises.map(exercise => (
                <DeploymentList key={exercise.id} exercise={exercise}/>
              ))}

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
