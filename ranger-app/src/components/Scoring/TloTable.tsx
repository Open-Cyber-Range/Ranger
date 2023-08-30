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
      <div className='flex mt-2'>
        {roles.map(role => {
          const tloMap = tloMapsByRole[role];
          if (tloMap && Object.keys(tloMap).length > 0) {
            const tloKeys = Object.keys(tloMap);
            return (
              <div key={role} className='w-full text-center'>
                <table
                  className='w-full my-8 border border-separate border-spacing-0
                  border-neutral-500 rounded-xl overflow-hidden'
                >
                  <colgroup/>
                  <colgroup/>
                  <colgroup
                    span={3}/>
                  <thead
                    className='bg-slate-300 font-medium'
                  >
                    <tr>
                      <th
                        className='px-6 py-2 border-r border-b border-neutral-500 text-lg'
                        rowSpan={2}
                      >
                        {t('tloTable.headers.tlo')}
                      </th>
                      <th
                        className='px-6 py-2 border-r border-b border-neutral-500 text-lg'
                        rowSpan={2}
                      >
                        {t('tloTable.headers.evaluation')}
                      </th>
                      <th
                        className='px-6 py-2 border-b border-neutral-500 text-lg'
                        colSpan={3}
                      >
                        {t('tloTable.headers.metrics')}
                      </th>
                    </tr>
                    <tr className='flex border-b border-neutral-500 text-sm'>
                      <th className='pl-1 w-2/5'>{t('tloTable.headers.name')}</th>
                      <th className='px-1 w-2/5'>{t('tloTable.headers.vm')}</th>
                      <th className='pr-1 w-1/5'>{t('tloTable.headers.points')}</th>
                    </tr>
                  </thead>
                  <tbody className='bg-neutral-100'>
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
