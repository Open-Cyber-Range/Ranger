import React from 'react';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {useGetDeploymentScenarioQuery} from 'src/slices/apiSlice';
import {ExerciseRoleOrder} from 'src/models/scenario';
import {getTloNamesByRole} from 'src/utils';
import ScoreTagBody from './ScoreTagBody';

const ScoreTags = ({exerciseId, deploymentId}:
{exerciseId: string;
  deploymentId: string;
}) => {
  const queryParameters = {exerciseId, deploymentId};
  const {data: scenario} = useGetDeploymentScenarioQuery(queryParameters);
  const goals = scenario?.goals;
  const entities = scenario?.entities;

  if (entities && goals) {
    const roleValues = Object.values(entities)
      .filter(entity => entity.role)
      .map(entity => entity.role!);
    roleValues.sort((a, b) => ExerciseRoleOrder[a] - ExerciseRoleOrder[b]);

    return (
      <div className='flex m-1 mt-auto mb-auto'>
        {roleValues.map(role => {
          const roleTloNames = getTloNamesByRole(entities, goals, role);

          const roleHasTlos = roleTloNames && roleTloNames.length > 0;

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
