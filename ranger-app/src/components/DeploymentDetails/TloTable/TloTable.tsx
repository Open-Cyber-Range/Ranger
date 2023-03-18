import React from 'react';
import {skipToken} from '@reduxjs/toolkit/dist/query';
import {useTranslation} from 'react-i18next';
import ScoreTagBody from 'src/components/Deployment/ScoreTags/ScoreTagBody';
import {
  type ExerciseRole,
  exerciseRoleOrder,
} from 'src/models/scenario/entity';
import {type TrainingLearningObjective} from 'src/models/scenario/tlo';
import {
  useGetDeploymentEntitiesQuery,
  useGetDeploymentGoalsQuery,
} from 'src/slices/apiSlice';
import {isNonNullable} from 'src/utils';
import styled from 'styled-components';
import TloRow from './TloRow';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  text-align: center;
  margin-top: 2rem;
`;

const ScoreTagsWrapper = styled.div`
  display: flex;
  flex-direction: column;
  align-items: stretch;
  margin-bottom: 1rem;
  `;

const TloTable = ({exerciseId, deploymentId, tloMap}:
{exerciseId: string;
  deploymentId: string;
  tloMap: Record<string, TrainingLearningObjective> | undefined;
}) => {
  const {t} = useTranslation();
  const queryParameters = exerciseId && deploymentId
    ? {exerciseId, deploymentId} : skipToken;
  const {data: entityMap}
  = useGetDeploymentEntitiesQuery(queryParameters);
  const {data: goalMap}
    = useGetDeploymentGoalsQuery(queryParameters);

  if (tloMap && entityMap && goalMap) {
    const entities = Object.values(entityMap);
    const roles = entities
      .filter(entity => entity.role)
      .map(entity => entity.role!);
    roles.sort((a, b) => exerciseRoleOrder[a] - exerciseRoleOrder[b]);

    type TloMapsByRole = {
      [key in ExerciseRole]?: Record<string, TrainingLearningObjective>};

    const tloMapsByRole: TloMapsByRole = {};

    for (const role of roles) {
      const roleEntities = entities.filter(entity =>
        entity.role?.valueOf() === role,
      );

      const roleTloKeys = roleEntities.flatMap(entity =>
        entity.goals?.flatMap(goalKey => goalMap[goalKey]?.tlos))
        // eslint-disable-next-line unicorn/no-array-callback-reference
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
      <Wrapper>
        {roles.map(role => {
          const tloMap = tloMapsByRole[role];
          if (tloMap) {
            const tloKeys = Object.keys(tloMap);

            return (
              <Wrapper key={role}>
                <ScoreTagsWrapper>
                  <ScoreTagBody
                    key={role}
                    large
                    exerciseId={exerciseId}
                    deploymentId={deploymentId}
                    role={role}
                  />
                </ScoreTagsWrapper>

                <table className='
                bp4-html-table
                bp4-html-table-striped'
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
              </Wrapper>
            );
          }

          return null;
        },
        )}
      </Wrapper>
    );
  }

  return null;
};

export default TloTable;
