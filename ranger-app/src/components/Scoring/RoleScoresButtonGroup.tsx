import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {
  useAdminGetDeploymentScenarioQuery,
  useAdminGetDeploymentScoresQuery,
} from 'src/slices/apiSlice';
import {ExerciseRoleOrder} from 'src/models/scenario';
import {
  calculateTotalScoreForRole,
  flattenEntities,
  getRoleColor,
  getUniqueRoles,
  roundToDecimalPlaces,
} from 'src/utils';
import {ButtonGroup, Button, Icon} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';

const RoleScoresButtonGroup = ({exerciseId, deploymentId}:
{exerciseId: string;
  deploymentId: string;
}) => {
  const {t} = useTranslation();
  const queryArguments = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryArguments);
  const {data: scores} = useAdminGetDeploymentScoresQuery(queryArguments);

  const entities = scenario?.entities;
  const navigate = useNavigate();

  if (entities && scores) {
    const flattenedEntities = flattenEntities(entities);
    const roles = getUniqueRoles(flattenedEntities);
    roles.sort((a, b) => ExerciseRoleOrder[a] - ExerciseRoleOrder[b]);

    return (
      <div className='flex flex-col mt-2 text-center'>
        <ButtonGroup fill>
          {roles.map(role => {
            const score = calculateTotalScoreForRole(
              {scenario, scores, role});
            const roundedScore = roundToDecimalPlaces(score);
            return (
              <Button
                key={role}
                style={{backgroundColor: getRoleColor(role)}}
                className='rounded-full mb-4 hover:scale-105 transition-all'
                rightIcon={
                  <Icon
                    icon='plus'
                    color='white'/>
                }
                alignText='center'
                onClick={() => {
                  navigate(`/exercises/${exerciseId}/deployments/${deploymentId}/scores/${role}`);
                }}
              >
                <span
                  className='font-bold text-white '
                >
                  {role} {`${t('common.team')}: ${roundedScore} ${t('common.points')}`}
                </span>
              </Button>
            );
          })}
        </ButtonGroup>
      </div>
    );
  }

  return null;
};

export default RoleScoresButtonGroup;
