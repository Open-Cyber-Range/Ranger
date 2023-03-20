import React from 'react';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {
  useGetDeploymentEntitiesQuery,
  useGetDeploymentGoalsQuery,
} from 'src/slices/apiSlice';
import {exerciseRoleOrder} from 'src/models/scenario/entity';
import {isNonNullable} from 'src/utils';
import ScoreTagBody from './ScoreTagBody';

const ScoreTags = ({exerciseId, deploymentId}:
{exerciseId: string;
  deploymentId: string;
}) => {
  const {data: entityMap}
  = useGetDeploymentEntitiesQuery({exerciseId, deploymentId});
  const {data: goalMap}
  = useGetDeploymentGoalsQuery({exerciseId, deploymentId});

  if (entityMap && goalMap) {
    const roles = Object.values(entityMap)
      .filter(entity => entity.role)
      .map(entity => entity.role!);
    roles.sort((a, b) => exerciseRoleOrder[a] - exerciseRoleOrder[b]);

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
