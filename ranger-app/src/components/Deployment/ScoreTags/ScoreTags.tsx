import React from 'react';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';
import {
  useGetDeploymentEntitiesQuery,
  useGetDeploymentGoalsQuery,
} from 'src/slices/apiSlice';
import {exerciseRoleOrder} from 'src/models/scenario/entity';
import ScoreTagBody from './ScoreTagBody';

const ScoreTags = ({exerciseId, deploymentId}:
{exerciseId: string;
  deploymentId: string;
}) => {
  const {data: entities}
  = useGetDeploymentEntitiesQuery({exerciseId, deploymentId});
  const {data: goals}
  = useGetDeploymentGoalsQuery({exerciseId, deploymentId});

  if (entities && goals) {
    const roles = Object.values(entities)
      .filter(entity => entity.role)
      .map(entity => entity.role!);
    roles.sort((a, b) => exerciseRoleOrder[a] - exerciseRoleOrder[b]);

    return (
      <div className='flex m-1 mt-auto mb-auto'>
        {roles.map(role => (
          <div key={role} className='flex mr-1'>
            <ScoreTagBody
              key={role}
              exerciseId={exerciseId}
              deploymentId={deploymentId}
              role={role}
            />
          </div>
        ))}
      </div>
    );
  }

  return null;
};

export default ScoreTags;
