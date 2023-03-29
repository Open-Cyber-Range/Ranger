import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import ScoreTagBody from 'src/components/Deployment/ScoreTags/ScoreTagBody';
import {useGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {isNonNullable} from 'src/utils';
import {
  type TrainingLearningObjective,
  ExerciseRoleOrder,
  type ExerciseRole,
} from 'src/models/scenario';
import TloRow from './TloRow';

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

    type TloMapsByRole = {
      [key in ExerciseRole]?: Record<string, TrainingLearningObjective>};

    const tloMapsByRole: TloMapsByRole = {};

    for (const role of roles) {
      const roleEntities = entities.filter(entity =>
        entity.role?.valueOf() === role,
      );

      const roleTloKeys = roleEntities.flatMap(entity =>
        entity.goals?.flatMap(goalKey => goalMap[goalKey]?.tlos))
        .filter(isNonNullable);

      const tloByTloName: Record<string, TrainingLearningObjective> = {};
      for (const key of roleTloKeys) {
        if (tloMap[key]) {
          tloByTloName[key] = tloMap[key];
        }
      }

      tloMapsByRole[role] = tloByTloName;
    }

    return (
      <div className='flex flex-col mt-2'>
        {roles.map(role => {
          const tloMap = tloMapsByRole[role];
          if (tloMap && Object.keys(tloMap).length > 0) {
            const tloKeys = Object.keys(tloMap);
            return (
              <div key={role} className='flex flex-col mt-2 text-center'>
                <div className='flex flex-col mt-6 font-bold'>
                  <ScoreTagBody
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
                      <TloRow
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
