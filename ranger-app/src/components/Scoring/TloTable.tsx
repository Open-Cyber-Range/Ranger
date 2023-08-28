import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import {useAdminGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {
  type TrainingLearningObjective,
  ExerciseRoleOrder,
} from 'src/models/scenario';
import {flattenEntities, getUniqueRoles, groupTloMapsByRoles} from 'src/utils';
import TloTableRow from './TloTableRow';

const TloTable = ({exerciseId, deploymentId, tloMap}:
{exerciseId: string;
  deploymentId: string;
  tloMap: Record<string, TrainingLearningObjective> | undefined;
}) => {
  const {t} = useTranslation();
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(
    exerciseId && deploymentId ? {exerciseId, deploymentId} : skipToken);
  const entities = scenario?.entities;

  if (tloMap && entities) {
    const flattenedEntities = flattenEntities(entities);
    const roles = getUniqueRoles(flattenedEntities);
    roles.sort((a, b) => ExerciseRoleOrder[a] - ExerciseRoleOrder[b]);
    const tloMapsByRole = groupTloMapsByRoles(
      flattenedEntities, tloMap, roles);

    return (
      <div className='flex flex-col mt-2'>
        {roles.map(role => {
          const tloMap = tloMapsByRole[role];
          if (tloMap && Object.keys(tloMap).length > 0) {
            const tloKeys = Object.keys(tloMap);
            return (
              <div key={role} className='flex flex-col text-center'>
                <table className='my-8 min-w-full border dark:border-neutral-500'>
                  <thead className='
                  border-b bg-slate-300 text-base font-medium dark:border-neutral-500'
                  >
                    <tr>
                      <th
                        className='border-r px-6 py-2 dark:border-neutral-500'
                      >
                        {t('tloTable.headers.tlo')}
                      </th>
                      <th
                        className='border-r px-6 py-2 dark:border-neutral-500'
                      >
                        {t('tloTable.headers.evaluation')}
                      </th>
                      <th
                        className='flex py-2 font-bold justify-center'
                        colSpan={3}
                      >
                        <tr className='w-full'>
                          <th className='pl-2 w-2/5'>{t('tloTable.headers.metric')}</th>
                          <th className='px-2 w-2/5'>{t('tloTable.headers.vm')}</th>
                          <th className='pr-2 w-1/5'>{t('tloTable.headers.points')}</th>
                        </tr>
                      </th>
                    </tr>
                  </thead>
                  <tbody className='border-b dark:border-neutral-500 bg-neutral-50'>
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
