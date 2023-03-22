import React from 'react';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {useGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {isNonNullable} from 'src/utils';
import {ExerciseRoleOrder} from 'src/models/scenario';
import ScoreTagBody from './ScoreTagBody';

const ScoreTags = ({exerciseId, deploymentId}:
{exerciseId: string;
  deploymentId: string;
}) => {
  const queryParameters = {exerciseId, deploymentId};
  const {data: scenario} = useGetDeploymentScenarioQuery(queryParameters);
  const goalMap = scenario?.goals;
  const entityMap = scenario?.entities;

  if (entityMap && goalMap) {
    const roles = Object.values(entityMap)
      .filter(entity => entity.role)
      .map(entity => entity.role!);
    roles.sort((a, b) => ExerciseRoleOrder[a] - ExerciseRoleOrder[b]);

    return (
      <div className='flex m-1 mt-auto mb-auto'>
        {roles.map(role => {
          const entities = Object.values(entityMap);
          const roleEntities = entities.filter(entity =>
            entity.role?.valueOf() === role);
          const roleTloKeys = roleEntities.flatMap(entity =>
            entity.goals?.flatMap(goalKey => goalMap[goalKey]?.tlos))
            .filter(tloKey => isNonNullable(tloKey));

          const roleHasTlos = roleTloKeys && roleTloKeys.length > 0;

          if (roleHasTlos) {
            return (
              <div key={role} className='flex mr-1'>
                <ScoreTagBody
                  key={role}
                  exerciseId={exerciseId}
                  deploymentId={deploymentId}
                  role={role}
                />
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

export default ScoreTags;
