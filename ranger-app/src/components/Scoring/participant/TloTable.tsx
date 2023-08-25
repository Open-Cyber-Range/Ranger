import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import ScoreTag from 'src/components/Scoring/ScoreTag';
import {useParticipantGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {
  type TrainingLearningObjective,
  ExerciseRoleOrder,
  type Entity,
} from 'src/models/scenario';
import {flattenEntities, getUniqueRoles, groupTloMapsByRoles} from 'src/utils';
import {useSelector} from 'react-redux';
import {selectedEntity as sliceSelectedEntity} from 'src/slices/userSlice';
import TloTableRow from './TloTableRow';

const TloTable = ({exerciseId, deploymentId, tloMap, selectedEntity}:
{exerciseId: string;
  deploymentId: string;
  tloMap: Record<string, TrainingLearningObjective> | undefined;
  selectedEntity: Entity | undefined;
}) => {
  const {t} = useTranslation();
  const entitySelector = useSelector(sliceSelectedEntity);
  const queryArguments = exerciseId && deploymentId && entitySelector
    ? {exerciseId, deploymentId, entitySelector} : skipToken;
  const {data: scenario} = useParticipantGetDeploymentScenarioQuery(queryArguments);
  const entities = scenario?.entities;

  if (tloMap && entities) {
    const flattenedEntities = flattenEntities(entities);

    let roles = getUniqueRoles(flattenedEntities);
    roles.sort((a, b) => ExerciseRoleOrder[a] - ExerciseRoleOrder[b]);

    if (selectedEntity?.role) {
      roles = [];
      roles.push(selectedEntity.role);
    }

    const tloMapsByRole = groupTloMapsByRoles(
      flattenedEntities, tloMap, roles);

    return (
      <div className='flex flex-col mt-2'>
        {roles.map(role => {
          const tloMap = tloMapsByRole[role];
          if (tloMap && Object.keys(tloMap).length > 0) {
            const tloKeys = Object.keys(tloMap);
            return (
              <div key={role} className='flex flex-col mt-2 text-center'>
                <div className='flex flex-col mt-6 font-bold'>
                  <ScoreTag
                    key={role}
                    large
                    exerciseId={exerciseId}
                    deploymentId={deploymentId}
                    role={role}
                  />
                </div>

                <table className='
                  bp4-html-table
                  bp4-html-table-striped
                  bp4-html-table-bordered'
                >
                  <tbody>
                    <tr>
                      <th>{t('tloTable.headers.tlo')}</th>
                      <th>{t('tloTable.headers.evaluation')}</th>
                      <th>{t('tloTable.headers.metric')}</th>
                    </tr>

                    { tloKeys.map(tloKey => (
                      <TloTableRow
                        key={tloKey}
                        scenario={scenario}
                        exerciseId={exerciseId}
                        deploymentId={deploymentId}
                        tloKey={tloKey}
                        tlo={tloMap[tloKey]}/>
                    )) }
                  </tbody>
                </table>
              </div>
            );
          }

          return null;
        },
        )}
      </div>
    );
  }

  return null;
};

export default TloTable;
