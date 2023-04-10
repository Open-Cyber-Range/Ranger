import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import ScoreTag from 'src/components/Scoring/ScoreTag';
import {useGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {
  type TrainingLearningObjective,
  ExerciseRoleOrder,
} from 'src/models/scenario';
import {groupTloMapsByRoles} from 'src/utils';
import TloTableRow from './TloTableRow';

const TloTable = ({exerciseId, deploymentId, tloMap}:
{exerciseId: string;
  deploymentId: string;
  tloMap: Record<string, TrainingLearningObjective> | undefined;
}) => {
  const {t} = useTranslation();
  const {data: scenario} = useGetDeploymentScenarioQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken);
  const goalMap = scenario?.goals;
  const entityMap = scenario?.entities;

  if (tloMap && entityMap && goalMap) {
    const entities = Object.values(entityMap);
    const roles = entities
      .filter(entity => entity.role)
      .map(entity => entity.role!);
    roles.sort((a, b) => ExerciseRoleOrder[a] - ExerciseRoleOrder[b]);
    const tloMapsByRole = groupTloMapsByRoles(
      entityMap, goalMap, tloMap, roles);

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
