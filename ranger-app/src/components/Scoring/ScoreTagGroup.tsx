import React from 'react';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {useAdminGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {ExerciseRoleOrder} from 'src/models/scenario';
import {flattenEntities, getTloNamesByRole, getUniqueRoles} from 'src/utils';
import ScoreTag from './ScoreTag';

const ScoreTagGroup = ({exerciseId, deploymentId}:
{exerciseId: string;
  deploymentId: string;
}) => {
  const queryParameters = {exerciseId, deploymentId};
  const {data: scenario} = useAdminGetDeploymentScenarioQuery(queryParameters);
  const goals = scenario?.goals;
  const entities = scenario?.entities;

  if (entities && goals) {
    const flattenedEntities = flattenEntities(entities);
    const roles = getUniqueRoles(flattenedEntities);
    roles.sort((a, b) => ExerciseRoleOrder[a] - ExerciseRoleOrder[b]);

    return (
      <div className='flex m-1 mt-auto mb-auto'>
        {roles.map(role => {
          const roleTloNames
          = getTloNamesByRole(flattenedEntities, goals, role);

          const roleHasTlos = roleTloNames && roleTloNames.length > 0;

          if (roleHasTlos) {
            return (
              <div key={role} className='flex mr-1'>
                <ScoreTag
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

export default ScoreTagGroup;
